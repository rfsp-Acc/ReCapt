//! Controle da gravacao: iniciar, pausar, retomar e parar.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use chrono::Local;
use serde::Serialize;
use tauri::State;

use crate::audio::recorder::{Recorder, SourceKind};
use crate::error::{AppError, AppResult};
use crate::session::{self, Session};
use crate::settings::SettingsState;

#[derive(Default)]
pub struct CaptureState(Mutex<Option<ActiveRecording>>);

struct ActiveRecording {
    session: Session,
    recorder: Recorder,
    screen: bool,
    started: Instant,
    paused_at: Option<Instant>,
    paused_total: Duration,
}

impl ActiveRecording {
    /// Tempo gravado, sem contar as pausas.
    fn elapsed(&self) -> Duration {
        let end = self.paused_at.unwrap_or_else(Instant::now);
        end.duration_since(self.started).saturating_sub(self.paused_total)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingStatus {
    pub recording: bool,
    pub paused: bool,
    pub session_id: Option<String>,
    /// Tempo gravado no momento da consulta; a interface continua contando a partir dele.
    pub elapsed_ms: u64,
    pub screen: bool,
    pub sources: Vec<SourceKind>,
    pub warnings: Vec<String>,
}

impl CaptureState {
    pub fn is_recording(&self) -> bool {
        self.0.lock().unwrap().is_some()
    }

    fn status(&self) -> RecordingStatus {
        match &*self.0.lock().unwrap() {
            Some(r) => RecordingStatus {
                recording: true,
                paused: r.paused_at.is_some(),
                session_id: Some(r.session.id.clone()),
                elapsed_ms: r.elapsed().as_millis() as u64,
                screen: r.screen,
                sources: r.recorder.sources(),
                warnings: r.recorder.warnings.clone(),
            },
            None => RecordingStatus {
                recording: false,
                paused: false,
                session_id: None,
                elapsed_ms: 0,
                screen: false,
                sources: Vec::new(),
                warnings: Vec::new(),
            },
        }
    }

    fn with_active<T>(&self, f: impl FnOnce(&mut ActiveRecording) -> T) -> AppResult<T> {
        let mut active = self.0.lock().unwrap();
        Ok(f(active.as_mut().ok_or(AppError::NotRecording)?))
    }
}

#[tauri::command]
pub fn start_recording(
    state: State<CaptureState>,
    settings: State<SettingsState>,
) -> AppResult<RecordingStatus> {
    {
        let mut active = state.0.lock().unwrap();
        if active.is_some() {
            return Err(AppError::AlreadyRecording);
        }
        let session = session::create(&settings.output_dir()?, Local::now())?;
        let recorder = match Recorder::start(&session.path) {
            Ok(r) => r,
            Err(e) => {
                // Nao deixa pasta vazia para tras quando o microfone nao abre.
                let _ = std::fs::remove_dir(&session.path);
                return Err(e.into());
            }
        };
        *active = Some(ActiveRecording {
            session,
            recorder,
            screen: false,
            started: Instant::now(),
            paused_at: None,
            paused_total: Duration::ZERO,
        });
    }
    Ok(state.status())
}

#[tauri::command]
pub fn pause_recording(state: State<CaptureState>) -> AppResult<RecordingStatus> {
    state.with_active(|r| {
        if r.paused_at.is_none() {
            r.recorder.set_paused(true);
            r.paused_at = Some(Instant::now());
        }
    })?;
    Ok(state.status())
}

#[tauri::command]
pub fn resume_recording(state: State<CaptureState>) -> AppResult<RecordingStatus> {
    state.with_active(|r| {
        if let Some(at) = r.paused_at.take() {
            r.paused_total += at.elapsed();
            r.recorder.set_paused(false);
        }
    })?;
    Ok(state.status())
}

/// Async para a mixagem rodar fora da thread principal e nao travar a interface.
#[tauri::command]
pub async fn stop_recording(state: State<'_, CaptureState>) -> AppResult<String> {
    let recording = state.0.lock().unwrap().take().ok_or(AppError::NotRecording)?;
    let session_id = recording.session.id;
    tauri::async_runtime::spawn_blocking(move || recording.recorder.finish())
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))??;
    Ok(session_id)
}

#[tauri::command]
pub fn toggle_screen_capture(state: State<CaptureState>) -> AppResult<bool> {
    state.with_active(|r| {
        r.screen = !r.screen;
        r.screen
    })
}

#[tauri::command]
pub fn recording_status(state: State<CaptureState>) -> RecordingStatus {
    state.status()
}
