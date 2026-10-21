use std::path::Path;

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use super::language::Language;
use crate::audio::SAMPLE_RATE;

#[derive(Debug, thiserror::Error)]
pub enum TranscribeError {
    #[error("falha ao ler o audio: {0}")]
    Wav(#[from] hound::Error),
    #[error("audio em formato inesperado: esperado 16 kHz mono 16 bits")]
    UnexpectedFormat,
    #[error("a gravacao nao tem audio")]
    EmptyAudio,
    #[error("erro do Whisper: {0}")]
    Whisper(#[from] whisper_rs::WhisperError),
}

pub struct Transcript {
    pub segments: Vec<Segment>,
    /// Codigo do idioma usado, ex.: "en".
    pub language: String,
    /// `true` quando o idioma veio da deteccao automatica.
    pub detected: bool,
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
}

/// Transcreve um WAV gerado pelo recorder. Bloqueante: chamar fora da thread principal.
pub fn transcribe(
    model: &Path,
    audio: &Path,
    language: Language,
    on_progress: impl FnMut(i32) + 'static,
) -> Result<Transcript, TranscribeError> {
    let samples = read_samples(audio)?;
    if samples.is_empty() {
        return Err(TranscribeError::EmptyAudio);
    }

    // Sem isso o whisper.cpp despeja logs de inicializacao no stderr.
    whisper_rs::install_logging_hooks();

    let ctx = WhisperContext::new_with_params(model, WhisperContextParameters::default())?;
    let mut state = ctx.create_state()?;

    let mut params = FullParams::new(SamplingStrategy::BeamSearch { beam_size: 5, patience: -1.0 });
    // Nao usar set_detect_language(true): no whisper.cpp ele so detecta e nao transcreve.
    params.set_language(Some(language.whisper_code()));
    params.set_n_threads(threads());
    // Cada janela de 30 s e decodificada sem o texto anterior: evita loops de repeticao.
    params.set_no_context(true);
    params.set_suppress_blank(true);
    params.set_suppress_nst(true);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_progress_callback_safe(on_progress);

    state.full(params, &samples)?;

    let mut segments = Vec::new();
    for segment in state.as_iter() {
        let text = segment.to_str_lossy()?.trim().to_string();
        if text.is_empty() {
            continue;
        }
        segments.push(Segment {
            // whisper.cpp usa centesimos de segundo.
            start_ms: segment.start_timestamp() * 10,
            end_ms: segment.end_timestamp() * 10,
            text,
        });
    }

    let detected = language == Language::Auto;
    let language = match language {
        Language::Auto => whisper_rs::get_lang_str(state.full_lang_id_from_state())
            .unwrap_or("desconhecido")
            .to_string(),
        chosen => chosen.whisper_code().to_string(),
    };
    Ok(Transcript { segments, language, detected })
}

fn read_samples(path: &Path) -> Result<Vec<f32>, TranscribeError> {
    let reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    if spec.sample_rate != SAMPLE_RATE || spec.channels != 1 || spec.bits_per_sample != 16 {
        return Err(TranscribeError::UnexpectedFormat);
    }
    reader
        .into_samples::<i16>()
        .map(|s| Ok(s? as f32 / 32_768.0))
        .collect()
}

fn threads() -> i32 {
    std::thread::available_parallelism()
        .map(|n| n.get().min(8) as i32)
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    /// Teste real com modelo e audio locais:
    /// RECAPT_MODEL=... RECAPT_AUDIO=... cargo test --release real_transcription -- --ignored --nocapture
    #[test]
    #[ignore]
    fn real_transcription() {
        let model = std::env::var("RECAPT_MODEL").expect("defina RECAPT_MODEL");
        let audio = std::env::var("RECAPT_AUDIO").expect("defina RECAPT_AUDIO");
        let start = std::time::Instant::now();
        let t = super::transcribe(model.as_ref(), audio.as_ref(), super::Language::Auto, |_| {}).unwrap();
        println!("idioma: {} (detectado: {})", t.language, t.detected);
        let segments = t.segments;
        for s in &segments {
            println!("[{} - {}] {}", s.start_ms, s.end_ms, s.text);
        }
        println!("{} segmentos em {:?}", segments.len(), start.elapsed());
        assert!(!segments.is_empty());
    }
}
