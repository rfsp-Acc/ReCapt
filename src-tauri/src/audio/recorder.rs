//! Grava microfone e audio do sistema em faixas temporarias e mixa tudo ao parar.
//!
//! Cada faixa tem um stream do cpal e uma thread de escrita: o callback de audio so
//! converte para mono e envia o bloco pelo canal; a thread converte para 16 kHz e grava.
//! Um vigia reabre os streams quando o dispositivo muda ou falha (TASK-07), o que cobre
//! fones e microfones Bluetooth conectados ou desconectados no meio da reuniao.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    Device, DeviceId, ErrorKind, FromSample, Host, Sample, SampleFormat, SizedSample, Stream,
    StreamConfig, I24, U24,
};
use serde::Serialize;

use super::clock::{self, Clock};
use super::resampler::Resampler;
use super::{mixer, wav_spec, SAMPLE_RATE};

/// Faixa mais atrasada que isso em relacao ao relogio recebe silencio (troca de dispositivo).
const MAX_LAG_SAMPLES: u64 = SAMPLE_RATE as u64 / 2;
/// Cabecalho do WAV atualizado periodicamente: o arquivo continua legivel se o app fechar.
const FLUSH_EVERY: Duration = Duration::from_secs(5);
const WATCH_EVERY: Duration = Duration::from_secs(1);
/// Dispositivos Bluetooth costumam aparecer antes de estarem prontos; espera antes de tentar de novo.
const RETRY_AFTER: Duration = Duration::from_secs(3);

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

    fn title(self) -> &'static str {
        match self {
            SourceKind::Microphone => "Microfone",
            SourceKind::System => "Áudio do sistema",
        }
    }

    pub fn temp_file(self) -> &'static str {
        match self {
            SourceKind::Microphone => ".microfone.wav",
            SourceKind::System => ".sistema.wav",
        }
    }
}

/// Microfone escolhido pelo usuario; `None` segue o padrao do sistema.
pub type MicChoice = Arc<Mutex<Option<String>>>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

pub fn input_devices() -> Vec<InputDevice> {
    let host = cpal::default_host();
    let default_id = host.default_input_device().and_then(|d| d.id().ok());
    host.input_devices()
        .map(|devices| {
            devices
                .filter_map(|d| {
                    let id = d.id().ok()?;
                    Some(InputDevice {
                        name: device_name(&d),
                        is_default: Some(&id) == default_id.as_ref(),
                        id: id.to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

struct Block {
    rate: u32,
    samples: Vec<f32>,
}

struct Live {
    _stream: Stream,
    device_id: Option<DeviceId>,
}

struct Track {
    kind: SourceKind,
    live: Option<Live>,
    /// Mantido para entregar clones aos streams reabertos; solto ao finalizar.
    tx: Sender<Block>,
    writer: JoinHandle<Result<PathBuf, RecorderError>>,
    /// Marcado pelo callback de erro quando o dispositivo some ou o stream e invalidado.
    failed: Arc<AtomicBool>,
    retry_at: Option<Instant>,
}

struct Shared {
    clock: Clock,
    paused: Arc<AtomicBool>,
    tracks: Mutex<Vec<Track>>,
    events: Mutex<Vec<String>>,
    mic_choice: MicChoice,
}

pub struct Recorder {
    dir: PathBuf,
    shared: Arc<Shared>,
    stop_tx: Sender<()>,
    watchdog: JoinHandle<()>,
}

impl Recorder {
    /// Inicia a gravacao. O microfone e obrigatorio; o audio do sistema e opcional.
    pub fn start(dir: &Path, mic_choice: MicChoice) -> Result<Self, RecorderError> {
        let host = cpal::default_host();
        let shared = Arc::new(Shared {
            clock: Clock::start(),
            paused: Arc::new(AtomicBool::new(false)),
            tracks: Mutex::new(Vec::new()),
            events: Mutex::new(Vec::new()),
            mic_choice,
        });

        let mic = wanted_device(&host, SourceKind::Microphone, &shared.mic_choice)
            .ok_or(RecorderError::NoMicrophone)?;
        let mut mic_track = new_track(SourceKind::Microphone, dir, &shared);
        mic_track.live = Some(open_stream(&mic_track, &mic, &shared)?);

        let mut sys_track = new_track(SourceKind::System, dir, &shared);
        // Loopback: stream de entrada no dispositivo de saida (process tap no macOS 14.6+,
        // WASAPI loopback no Windows).
        match wanted_device(&host, SourceKind::System, &shared.mic_choice) {
            Some(output) => match open_stream(&sys_track, &output, &shared) {
                Ok(live) => sys_track.live = Some(live),
                Err(e) => {
                    sys_track.retry_at = Some(Instant::now() + RETRY_AFTER);
                    shared.event(format!("Gravando só o microfone por enquanto: {e}."));
                }
            },
            None => shared.event("Gravando só o microfone: nenhuma saída de áudio encontrada.".into()),
        }
        *shared.tracks.lock().unwrap() = vec![mic_track, sys_track];

        let (stop_tx, stop_rx) = mpsc::channel();
        let watchdog = spawn_watchdog(shared.clone(), stop_rx);
        Ok(Self { dir: dir.to_path_buf(), shared, stop_tx, watchdog })
    }

    /// Fontes gravando neste momento.
    pub fn sources(&self) -> Vec<SourceKind> {
        let tracks = self.shared.tracks.lock().unwrap();
        tracks.iter().filter(|t| t.live.is_some()).map(|t| t.kind).collect()
    }

    pub fn warnings(&self) -> Vec<String> {
        self.shared.events.lock().unwrap().clone()
    }

    pub fn elapsed(&self) -> Duration {
        self.shared.clock.elapsed()
    }

    pub fn is_paused(&self) -> bool {
        self.shared.clock.is_paused()
    }

    /// Durante a pausa os callbacks descartam o audio e o relogio para; as faixas continuam alinhadas.
    pub fn set_paused(&self, paused: bool) {
        if paused {
            self.shared.paused.store(true, Ordering::SeqCst);
            self.shared.clock.pause();
        } else {
            self.shared.clock.resume();
            self.shared.paused.store(false, Ordering::SeqCst);
        }
    }

    /// Para os streams, mixa as faixas em `audio.wav` e remove os temporarios.
    /// Bloqueante: chamar fora da thread principal.
    pub fn finish(self) -> Result<PathBuf, RecorderError> {
        drop(self.stop_tx);
        let _ = self.watchdog.join();

        let tracks = std::mem::take(&mut *self.shared.tracks.lock().unwrap());
        let mut files = Vec::new();
        for track in tracks {
            // Soltar o stream e o canal encerra a thread de escrita, que finaliza o WAV.
            drop(track.live);
            drop(track.tx);
            let path = track.writer.join().expect("thread de escrita de audio entrou em panico")?;
            files.push(path);
        }
        mix_and_cleanup(&self.dir, &files)
    }
}

impl Shared {
    fn event(&self, message: String) {
        let at = clock::format(self.clock.elapsed());
        self.events.lock().unwrap().push(format!("[{at}] {message}"));
    }
}

/// Recupera uma sessao interrompida (app fechado durante a gravacao): mixa as faixas
/// temporarias que ainda puderem ser lidas. Devolve `true` se gerou `audio.wav`.
pub fn recover(dir: &Path) -> Result<bool, RecorderError> {
    let temps: Vec<PathBuf> = [SourceKind::Microphone, SourceKind::System]
        .iter()
        .map(|k| dir.join(k.temp_file()))
        .filter(|p| p.is_file())
        .collect();
    if temps.is_empty() {
        return Ok(false);
    }
    if dir.join("audio.wav").exists() {
        // Falhou entre a mixagem e a limpeza: o audio final ja existe.
        for t in &temps {
            fs::remove_file(t)?;
        }
        return Ok(false);
    }
    // Faixas com cabecalho ilegivel (app fechou antes do primeiro flush) ficam no disco.
    let readable: Vec<PathBuf> = temps.into_iter().filter(|p| hound::WavReader::open(p).is_ok()).collect();
    if readable.is_empty() {
        return Ok(false);
    }
    mix_and_cleanup(dir, &readable)?;
    Ok(true)
}

fn mix_and_cleanup(dir: &Path, files: &[PathBuf]) -> Result<PathBuf, RecorderError> {
    let output = dir.join("audio.wav");
    // Mixa num arquivo parcial e renomeia: uma falha no meio nunca deixa audio.wav incompleto.
    let partial = dir.join("audio.wav.part");
    let inputs: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
    mixer::mix(&inputs, &partial)?;
    fs::rename(&partial, &output)?;
    for file in files {
        fs::remove_file(file)?;
    }
    Ok(output)
}

fn new_track(kind: SourceKind, dir: &Path, shared: &Arc<Shared>) -> Track {
    let (tx, rx) = mpsc::channel();
    let writer = spawn_writer(rx, dir.join(kind.temp_file()), shared.clone());
    Track { kind, live: None, tx, writer, failed: Arc::new(AtomicBool::new(false)), retry_at: None }
}

/// Microfone escolhido (se conectado) ou o padrao; para o sistema, sempre a saida padrao,
/// que e para onde os apps de reuniao mandam o som.
fn wanted_device(host: &Host, kind: SourceKind, mic_choice: &MicChoice) -> Option<Device> {
    match kind {
        SourceKind::Microphone => {
            let chosen = mic_choice.lock().unwrap().clone();
            chosen
                .and_then(|id| id.parse().ok())
                .and_then(|id| host.device_by_id(&id))
                .or_else(|| host.default_input_device())
        }
        SourceKind::System => host.default_output_device(),
    }
}

fn device_name(device: &Device) -> String {
    device.description().map(|d| d.name().to_string()).unwrap_or_else(|_| "dispositivo sem nome".into())
}

fn spawn_watchdog(shared: Arc<Shared>, stop_rx: Receiver<()>) -> JoinHandle<()> {
    thread::spawn(move || {
        let host = cpal::default_host();
        while let Err(RecvTimeoutError::Timeout) = stop_rx.recv_timeout(WATCH_EVERY) {
            let mut tracks = shared.tracks.lock().unwrap();
            for track in tracks.iter_mut() {
                watch(&host, track, &shared);
            }
        }
    })
}

fn watch(host: &Host, track: &mut Track, shared: &Shared) {
    let failed = track.failed.load(Ordering::SeqCst);
    let wanted = wanted_device(host, track.kind, &shared.mic_choice);
    let wanted_id = wanted.as_ref().and_then(|d| d.id().ok());
    let current_id = track.live.as_ref().map(|l| l.device_id.clone());

    let changed = current_id.as_ref() != Some(&wanted_id);
    if !failed && !changed {
        return;
    }
    if track.retry_at.is_some_and(|at| Instant::now() < at) {
        return;
    }

    let had_stream = track.live.take().is_some();
    track.failed.store(false, Ordering::SeqCst);

    let Some(device) = wanted else {
        if had_stream {
            shared.event(format!("{} desconectado; aguardando outro dispositivo.", track.kind.title()));
        }
        return;
    };

    match open_stream(track, &device, shared) {
        Ok(live) => {
            track.live = Some(live);
            track.retry_at = None;
            shared.event(format!("{}: gravando de {}.", track.kind.title(), device_name(&device)));
        }
        Err(e) => {
            track.retry_at = Some(Instant::now() + RETRY_AFTER);
            if had_stream {
                shared.event(format!("{}: {e}. Tentando de novo.", track.kind.title()));
            }
        }
    }
}

fn open_stream(track: &Track, device: &Device, shared: &Shared) -> Result<Live, RecorderError> {
    let kind = track.kind;
    let device_err = |message: String| RecorderError::Device { source_name: kind.label(), message };

    let supported = if device.supports_input() {
        device.default_input_config()
    } else {
        device.default_output_config()
    }
    .map_err(|e| device_err(e.to_string()))?;

    let format = supported.sample_format();
    let config: StreamConfig = supported.into();
    let (tx, paused, failed) = (track.tx.clone(), shared.paused.clone(), track.failed.clone());

    // Todos os formatos PCM: dispositivos Bluetooth e interfaces de audio variam bastante.
    let stream = match format {
        SampleFormat::I8 => build_stream::<i8>(device, &config, tx, paused, failed),
        SampleFormat::I16 => build_stream::<i16>(device, &config, tx, paused, failed),
        SampleFormat::I24 => build_stream::<I24>(device, &config, tx, paused, failed),
        SampleFormat::I32 => build_stream::<i32>(device, &config, tx, paused, failed),
        SampleFormat::I64 => build_stream::<i64>(device, &config, tx, paused, failed),
        SampleFormat::U8 => build_stream::<u8>(device, &config, tx, paused, failed),
        SampleFormat::U16 => build_stream::<u16>(device, &config, tx, paused, failed),
        SampleFormat::U24 => build_stream::<U24>(device, &config, tx, paused, failed),
        SampleFormat::U32 => build_stream::<u32>(device, &config, tx, paused, failed),
        SampleFormat::U64 => build_stream::<u64>(device, &config, tx, paused, failed),
        SampleFormat::F32 => build_stream::<f32>(device, &config, tx, paused, failed),
        SampleFormat::F64 => build_stream::<f64>(device, &config, tx, paused, failed),
        other => return Err(device_err(format!("formato de amostra nao suportado: {other}"))),
    }
    .map_err(|e| device_err(e.to_string()))?;

    stream.play().map_err(|e| device_err(e.to_string()))?;
    Ok(Live { _stream: stream, device_id: device.id().ok() })
}

fn build_stream<T>(
    device: &Device,
    config: &StreamConfig,
    tx: Sender<Block>,
    paused: Arc<AtomicBool>,
    failed: Arc<AtomicBool>,
) -> Result<Stream, cpal::Error>
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let channels = config.channels.max(1) as usize;
    let rate = config.sample_rate;
    device.build_input_stream::<T, _, _>(
        *config,
        move |data: &[T], _| {
            if paused.load(Ordering::Relaxed) {
                return;
            }
            let samples: Vec<f32> = data
                .chunks(channels)
                .map(|frame| frame.iter().map(|&s| f32::from_sample(s)).sum::<f32>() / channels as f32)
                .collect();
            // Erro so acontece se a thread de escrita ja terminou; nada a fazer no callback.
            let _ = tx.send(Block { rate, samples });
        },
        move |err| match err.kind() {
            // O vigia reabre o stream no dispositivo atual.
            ErrorKind::DeviceNotAvailable | ErrorKind::StreamInvalidated | ErrorKind::DeviceChanged => {
                failed.store(true, Ordering::SeqCst)
            }
            _ => eprintln!("[audio] erro no stream: {err}"),
        },
        None,
    )
}

fn spawn_writer(rx: Receiver<Block>, path: PathBuf, shared: Arc<Shared>) -> JoinHandle<Result<PathBuf, RecorderError>> {
    thread::spawn(move || {
        let mut writer = hound::WavWriter::create(&path, wav_spec())?;
        let mut resampler: Option<(u32, Resampler)> = None;
        let mut out = Vec::new();
        let mut written: u64 = 0;
        let mut last_flush = Instant::now();

        // Termina quando o stream e o canal sao soltos.
        for block in rx {
            // Ao trocar de dispositivo a taxa pode mudar (Bluetooth usa 8, 16 ou 24 kHz).
            if resampler.as_ref().is_none_or(|(rate, _)| *rate != block.rate) {
                resampler = Some((block.rate, Resampler::new(block.rate, SAMPLE_RATE)));
            }
            out.clear();
            resampler.as_mut().unwrap().1.process(&block.samples, &mut out);

            let gap = lag(shared.clock.elapsed(), written, out.len() as u64);
            for _ in 0..gap {
                writer.write_sample(0i16)?;
            }
            written += gap;

            for &s in &out {
                writer.write_sample((s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)?;
            }
            written += out.len() as u64;

            if last_flush.elapsed() >= FLUSH_EVERY {
                writer.flush()?;
                last_flush = Instant::now();
            }
        }

        writer.finalize()?;
        Ok(path)
    })
}

/// Silencio necessario para a faixa alcancar o relogio, ignorando atrasos pequenos (latencia normal).
fn lag(elapsed: Duration, written: u64, incoming: u64) -> u64 {
    let expected = (elapsed.as_secs_f64() * SAMPLE_RATE as f64) as u64;
    let after = written + incoming;
    if expected > after + MAX_LAG_SAMPLES { expected - after } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_lag_is_ignored() {
        // 10 s de relogio, 9,8 s gravados: latencia normal.
        assert_eq!(lag(Duration::from_secs(10), 156_800, 0), 0);
    }

    #[test]
    fn large_lag_is_filled_with_silence() {
        // Dispositivo ficou 2 s sem enviar audio.
        assert_eq!(lag(Duration::from_secs(10), 128_000, 0), 32_000);
        assert_eq!(lag(Duration::from_secs(10), 120_000, 8_000), 32_000);
    }

    #[test]
    fn recover_mixes_readable_tracks_and_skips_finished_sessions() {
        let dir = std::env::temp_dir().join(format!("recapt-recover-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        // Faixa interrompida: so o flush, sem finalize.
        let mut w = hound::WavWriter::create(dir.join(".microfone.wav"), wav_spec()).unwrap();
        for s in [1i16, 2, 3] {
            w.write_sample(s).unwrap();
        }
        w.flush().unwrap();
        std::mem::forget(w);
        // Faixa com cabecalho corrompido fica no disco.
        fs::write(dir.join(".sistema.wav"), b"lixo").unwrap();

        assert!(recover(&dir).unwrap());
        let samples: Vec<i16> =
            hound::WavReader::open(dir.join("audio.wav")).unwrap().samples().map(Result::unwrap).collect();
        assert_eq!(samples, [1, 2, 3]);
        assert!(!dir.join(".microfone.wav").exists());
        assert!(dir.join(".sistema.wav").exists());

        // Sessao ja finalizada: so limpa.
        assert!(!recover(&dir).unwrap());
        assert!(!dir.join(".sistema.wav").exists());
        fs::remove_dir_all(&dir).unwrap();
    }
}

#[cfg(test)]
mod device_tests {
    /// Lista os microfones da maquina: cargo test --lib list_devices -- --ignored --nocapture
    #[test]
    #[ignore]
    fn list_devices() {
        for d in super::input_devices() {
            println!("{}{} ({})", if d.is_default { "* " } else { "  " }, d.name, d.id);
        }
    }
}
