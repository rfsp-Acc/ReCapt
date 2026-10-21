//! Soma as faixas temporarias (mono, 16 kHz) num unico WAV, em streaming.

use std::path::Path;

use hound::{WavReader, WavWriter};

pub fn mix(inputs: &[&Path], output: &Path) -> hound::Result<()> {
    let mut readers = inputs
        .iter()
        .map(WavReader::open)
        .collect::<hound::Result<Vec<_>>>()?;
    let mut tracks: Vec<_> = readers.iter_mut().map(|r| r.samples::<i16>()).collect();
    let mut writer = WavWriter::create(output, super::wav_spec())?;

    // As faixas comecam juntas; a mais curta e completada com silencio.
    loop {
        let mut sum: i32 = 0;
        let mut any = false;
        for track in tracks.iter_mut() {
            if let Some(sample) = track.next() {
                sum += sample? as i32;
                any = true;
            }
        }
        if !any {
            break;
        }
        writer.write_sample(sum.clamp(i16::MIN as i32, i16::MAX as i32) as i16)?;
    }

    writer.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, samples: &[i16]) {
        let mut w = WavWriter::create(path, crate::audio::wav_spec()).unwrap();
        for &s in samples {
            w.write_sample(s).unwrap();
        }
        w.finalize().unwrap();
    }

    fn read(path: &Path) -> Vec<i16> {
        WavReader::open(path).unwrap().samples::<i16>().map(Result::unwrap).collect()
    }

    #[test]
    fn sums_tracks_pads_shorter_and_clamps() {
        let dir = std::env::temp_dir().join(format!("recapt-mixer-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (a, b, out) = (dir.join("a.wav"), dir.join("b.wav"), dir.join("out.wav"));

        write(&a, &[100, 200, 30_000, -30_000, 5]);
        write(&b, &[1, 2, 10_000, -10_000]);
        mix(&[&a, &b], &out).unwrap();

        assert_eq!(read(&out), vec![101, 202, i16::MAX, i16::MIN, 5]);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn single_track_is_copied() {
        let dir = std::env::temp_dir().join(format!("recapt-mixer1-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (a, out) = (dir.join("a.wav"), dir.join("out.wav"));

        write(&a, &[1, -2, 3]);
        mix(&[&a], &out).unwrap();

        assert_eq!(read(&out), vec![1, -2, 3]);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
