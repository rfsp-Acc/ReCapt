//! Relogio da gravacao: tempo ativo, sem contar as pausas.
//! Usado pelo cronometro e para manter as faixas alinhadas.

use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct Clock(Mutex<Inner>);

struct Inner {
    started: Instant,
    paused_at: Option<Instant>,
    paused_total: Duration,
}

impl Clock {
    pub fn start() -> Self {
        Self(Mutex::new(Inner { started: Instant::now(), paused_at: None, paused_total: Duration::ZERO }))
    }

    pub fn elapsed(&self) -> Duration {
        let i = self.0.lock().unwrap();
        let end = i.paused_at.unwrap_or_else(Instant::now);
        end.duration_since(i.started).saturating_sub(i.paused_total)
    }

    pub fn is_paused(&self) -> bool {
        self.0.lock().unwrap().paused_at.is_some()
    }

    pub fn pause(&self) {
        let mut i = self.0.lock().unwrap();
        if i.paused_at.is_none() {
            i.paused_at = Some(Instant::now());
        }
    }

    pub fn resume(&self) {
        let mut i = self.0.lock().unwrap();
        if let Some(at) = i.paused_at.take() {
            i.paused_total += at.elapsed();
        }
    }
}

/// "HH:MM:SS", mesmo formato da transcricao.
pub fn format(d: Duration) -> String {
    let s = d.as_secs();
    format!("{:02}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn pause_is_not_counted() {
        let clock = Clock::start();
        sleep(Duration::from_millis(30));
        clock.pause();
        let at_pause = clock.elapsed();
        sleep(Duration::from_millis(50));
        assert_eq!(clock.elapsed(), at_pause);
        clock.resume();
        sleep(Duration::from_millis(30));
        let total = clock.elapsed();
        assert!(total >= Duration::from_millis(60) && total < Duration::from_millis(75), "{total:?}");
    }

    #[test]
    fn formats_like_transcript() {
        assert_eq!(format(Duration::from_secs(3_725)), "01:02:05");
    }
}
