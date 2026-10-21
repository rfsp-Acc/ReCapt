//! Captura de audio: microfone + audio do sistema, mixados em audio.wav.

pub mod clock;
pub mod mixer;
pub mod recorder;
mod resampler;

/// Formato final: o Whisper consome 16 kHz mono direto (TASK-03).
pub const SAMPLE_RATE: u32 = 16_000;

pub fn wav_spec() -> hound::WavSpec {
    hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    }
}
