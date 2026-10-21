//! Conversao de taxa em streaming, bloco a bloco, para nao acumular o audio em memoria.
//! Media movel como filtro anti-aliasing simples + interpolacao linear: suficiente para voz.

pub struct Resampler {
    /// Amostras de entrada por amostra de saida.
    step: f64,
    /// Posicao da proxima saida, em indices de `[prev, input...]`.
    pos: f64,
    prev: f32,
    window: Vec<f32>,
    window_pos: usize,
    window_sum: f32,
}

impl Resampler {
    pub fn new(from_rate: u32, to_rate: u32) -> Self {
        let step = from_rate as f64 / to_rate as f64;
        // So filtra quando reduz a taxa; janela de tamanho ~step.
        let taps = if step > 1.0 { step.round() as usize } else { 1 };
        Self {
            step,
            pos: 1.0,
            prev: 0.0,
            window: vec![0.0; taps],
            window_pos: 0,
            window_sum: 0.0,
        }
    }

    pub fn process(&mut self, input: &[f32], out: &mut Vec<f32>) {
        if input.is_empty() {
            return;
        }
        let filtered: Vec<f32> = input.iter().map(|&s| self.filter(s)).collect();
        let len = filtered.len() as f64;

        while self.pos < len {
            let i = self.pos.floor() as usize;
            let frac = (self.pos - i as f64) as f32;
            let a = if i == 0 { self.prev } else { filtered[i - 1] };
            let b = filtered[i];
            out.push(a + (b - a) * frac);
            self.pos += self.step;
        }

        self.pos -= len;
        self.prev = *filtered.last().unwrap();
    }

    fn filter(&mut self, sample: f32) -> f32 {
        if self.window.len() == 1 {
            return sample;
        }
        self.window_sum += sample - self.window[self.window_pos];
        self.window[self.window_pos] = sample;
        self.window_pos = (self.window_pos + 1) % self.window.len();
        self.window_sum / self.window.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(from: u32, to: u32, input: &[f32], chunk: usize) -> Vec<f32> {
        let mut r = Resampler::new(from, to);
        let mut out = Vec::new();
        for c in input.chunks(chunk) {
            r.process(c, &mut out);
        }
        out
    }

    #[test]
    fn output_length_follows_ratio() {
        let input = vec![0.5; 48_000];
        let out = run(48_000, 16_000, &input, 512);
        assert!((out.len() as i64 - 16_000).abs() <= 1, "len = {}", out.len());

        let out = run(44_100, 16_000, &vec![0.5; 44_100], 441);
        assert!((out.len() as i64 - 16_000).abs() <= 1, "len = {}", out.len());
    }

    #[test]
    fn chunk_size_does_not_change_result() {
        let input: Vec<f32> = (0..9_600).map(|i| (i as f32 * 0.01).sin()).collect();
        let a = run(48_000, 16_000, &input, 480);
        let b = run(48_000, 16_000, &input, 7);
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            assert!((x - y).abs() < 1e-5);
        }
    }

    #[test]
    fn same_rate_is_passthrough() {
        let input: Vec<f32> = (0..100).map(|i| i as f32 / 100.0).collect();
        let out = run(16_000, 16_000, &input, 33);
        // A ultima amostra fica retida ate o proximo bloco (necessaria para interpolar).
        assert_eq!(out, input[..input.len() - 1]);
    }

    #[test]
    fn constant_signal_keeps_level() {
        let out = run(48_000, 16_000, &vec![0.25; 4_800], 256);
        // Ignora o inicio, onde o filtro ainda esta enchendo.
        for &s in &out[10..] {
            assert!((s - 0.25).abs() < 1e-4);
        }
    }
}
