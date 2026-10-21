//! Grava microfone e audio do sistema em faixas temporarias e mixa tudo ao parar.
//!
//! Cada fonte tem um stream do cpal e uma thread de escrita: o callback de audio so
//! converte para mono e envia o bloco pelo canal; a thread converte para 16 kHz e grava.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig};
use serde::Serialize;

use super::resampler::Resampler;
use super::{mixer, wav_spec, SAMPLE_RATE};

#[derive(Debug, thiserror::Error)]
pub enum RecorderError {
    #[error("nenhum microfone encontrado")]
    NoMicrophone,
    #[error("falha ao abrir {source_name}: {message}")]
    Device { source_name: &'static str, message: String },
    #[error("falha ao gravar o audio: {0}")]
    Wav(#[from] hound::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SourceKind {
    Microphone,
    System,
}

impl SourceKind {
    fn label(self) -> &'static str {
        match self {
            SourceKind::Microphone => "o microfone",
            SourceKind::System => "o audio do sistema",
        }
    }

    fn temp_file(self) -> &'static str {
        match self {
            SourceKind::Microphone => ".microfone.wav",
            SourceKind::System => ".sistema.wav",
        }
    }
}

struct Track {
    kind: SourceKind,
    stream: Stream,
    writer: JoinHandle<Result<PathBuf, RecorderError>>,
}

pub struct Recorder {
    dir: PathBuf,
    tracks: Vec<Track>,
    paused: Arc<AtomicBool>,
    /// Avisos para a interface, ex.: loopback indisponivel.
    pub warnings: Vec<String>,
}

impl Recorder {
    /// Inicia a gravacao. O microfone e obrigatorio; o audio do sistema e opcional.
    pub fn start(dir: &Path) -> Result<Self, RecorderError> {
        let host = cpal::default_host();
        let paused = Arc::new(AtomicBool::new(false));
        let mut warnings = Vec::new();

        let mic = host.default_input_device().ok_or(RecorderError::NoMicrophone)?;
        let mut tracks = vec![open_track(SourceKind::Microphone, &mic, dir, &paused)?];

        // Loopback: stream de entrada no dispositivo de saida (process tap no macOS 14.6+,
        // WASAPI loopback no Windows).
        match host.default_output_device() {
            Some(output) => match open_track(SourceKind::System, &output, dir, &paused) {
                Ok(track) => tracks.push(track),
                Err(e) => warnings.push(format!(
                    "Gravando só o microfone: não foi possível capturar o áudio do sistema ({e})."
                )),
            },
            None => warnings.push(
                "Gravando só o microfone: nenhum dispositivo de saída de áudio encontrado.".into(),
            ),
        }

        Ok(Self { dir: dir.to_path_buf(), tracks, paused, warnings })
    }

    pub fn sources(&self) -> Vec<SourceKind> {
        self.tracks.iter().map(|t| t.kind).collect()
    }

    /// Durante a pausa os callbacks descartam o audio, entao as faixas continuam alinhadas.
    pub fn set_paused(&self, paused: bool) {
        self.paused.store(paused, Ordering::SeqCst);
    }

    /// Para os streams, mixa as faixas em `audio.wav` e remove os temporarios.
    /// Bloqueante: chamar fora da thread principal.
    pub fn finish(self) -> Result<PathBuf, RecorderError> {
        let mut files = Vec::new();
        for track in self.tracks {
            // Soltar o stream encerra o callback e fecha o canal; a thread finaliza o WAV.
            drop(track.stream);
            let path = track.writer.join().expect("thread de escrita de audio entrou em panico")?;
            files.push(path);
        }

        let output = self.dir.join("audio.wav");
        let inputs: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
        mixer::mix(&inputs, &output)?;
        for file in &files {
            fs::remove_file(file)?;
        }
        Ok(output)
    }
}

fn open_track(
    kind: SourceKind,
    device: &Device,
    dir: &Path,
    paused: &Arc<AtomicBool>,
) -> Result<Track, RecorderError> {
    let device_err = |message: String| RecorderError::Device { source_name: kind.label(), message };

    let supported = if device.supports_input() {
        device.default_input_config()
    } else {
        device.default_output_config()
    }
    .map_err(|e| device_err(e.to_string()))?;

    let format = supported.sample_format();
    let config: StreamConfig = supported.into();
    let (tx, rx) = mpsc::channel::<Vec<f32>>();

    let path = dir.join(kind.temp_file());
    let writer = spawn_writer(rx, path, config.sample_rate);

    let stream = match format {
        SampleFormat::F32 => build_stream::<f32>(device, &config, tx, paused.clone()),
        SampleFormat::I16 => build_stream::<i16>(device, &config, tx, paused.clone()),
        SampleFormat::I32 => build_stream::<i32>(device, &config, tx, paused.clone()),
        SampleFormat::U16 => build_stream::<u16>(device, &config, tx, paused.clone()),
        other => return Err(device_err(format!("formato de amostra nao suportado: {other}"))),
    }
    .map_err(|e| device_err(e.to_string()))?;

    stream.play().map_err(|e| device_err(e.to_string()))?;
    Ok(Track { kind, stream, writer })
}

fn build_stream<T>(
    device: &Device,
    config: &StreamConfig,
    tx: Sender<Vec<f32>>,
    paused: Arc<AtomicBool>,
) -> Result<Stream, cpal::Error>
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let channels = config.channels.max(1) as usize;
    device.build_input_stream::<T, _, _>(
        *config,
        move |data: &[T], _| {
            if paused.load(Ordering::Relaxed) {
                return;
            }
            let mono: Vec<f32> = data
                .chunks(channels)
                .map(|frame| frame.iter().map(|&s| f32::from_sample(s)).sum::<f32>() / channels as f32)
                .collect();
            // Erro so acontece se a thread de escrita ja terminou; nada a fazer no callback.
            let _ = tx.send(mono);
        },
        |err| eprintln!("[audio] erro no stream: {err}"),
        None,
    )
}

fn spawn_writer(
    rx: Receiver<Vec<f32>>,
    path: PathBuf,
    input_rate: u32,
) -> JoinHandle<Result<PathBuf, RecorderError>> {
    thread::spawn(move || {
        let mut writer = hound::WavWriter::create(&path, wav_spec())?;
        let mut resampler = Resampler::new(input_rate, SAMPLE_RATE);
        let mut out = Vec::new();

        // Termina quando o stream e solto e o canal fecha.
        for block in rx {
            out.clear();
            resampler.process(&block, &mut out);
            for &s in &out {
                writer.write_sample((s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)?;
            }
        }

        writer.finalize()?;
        Ok(path)
    })
}
