//! Controle da gravacao: iniciar, pausar, retomar e parar.

use std::sync::Mutex;

use chrono::Local;
use serde::Serialize;
use tauri::State;

use crate::audio::recorder::{self, InputDevice, Recorder, SourceKind};
use crate::error::{AppError, AppResult};
use crate::session::{self, Session};
use crate::settings::SettingsState;

#[derive(Default)]
pub struct CaptureState(Mutex<Option<ActiveRecording>>, RecoveryLock);

/// Uma recuperacao por vez: o React em modo dev executa os efeitos duas vezes.
#[derive(Default)]
struct RecoveryLock(std::sync::Arc<Mutex<()>>);

struct ActiveRecording {
    session: Session,
    recorder: Recorder,
    screen: bool,
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

    pub fn active_session_id(&self) -> Option<String> {
        self.0.lock().unwrap().as_ref().map(|r| r.session.id.clone())
    }

    fn status(&self) -> RecordingStatus {
        match &*self.0.lock().unwrap() {
            Some(r) => RecordingStatus {
                recording: true,
                paused: r.recorder.is_paused(),
                session_id: Some(r.session.id.clone()),
                elapsed_ms: r.recorder.elapsed().as_millis() as u64,
                screen: r.screen,
                sources: r.recorder.sources(),
                warnings: r.recorder.warnings(),
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
        let recorder = match Recorder::start(&session.path, settings.mic_choice()) {
            Ok(r) => r,
            Err(e) => {
                // Nao deixa pasta vazia para tras quando o microfone nao abre.
                let _ = std::fs::remove_dir(&session.path);
                return Err(e.into());
            }
        };
        *active = Some(ActiveRecording { session, recorder, screen: false });
    }
    Ok(state.status())
}

#[tauri::command]
pub fn pause_recording(state: State<CaptureState>) -> AppResult<RecordingStatus> {
    state.with_active(|r| r.recorder.set_paused(true))?;
    Ok(state.status())
}

#[tauri::command]
pub fn resume_recording(state: State<CaptureState>) -> AppResult<RecordingStatus> {
    state.with_active(|r| r.recorder.set_paused(false))?;
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputDevices {
    pub devices: Vec<InputDevice>,
    /// `None` segue o padrao do sistema.
    pub selected: Option<String>,
}

/// Lista os microfones conectados no momento, incluindo Bluetooth.
#[tauri::command]
pub async fn list_input_devices(settings: State<'_, SettingsState>) -> AppResult<InputDevices> {
    // Enumerar dispositivos pode demorar alguns instantes com Bluetooth; fora da thread principal.
    let devices = tauri::async_runtime::spawn_blocking(recorder::input_devices)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(InputDevices { devices, selected: settings.input_device() })
}

/// Vale tambem durante a gravacao: o vigia troca o microfone em ate 1 s.
#[tauri::command]
pub fn set_input_device(settings: State<SettingsState>, id: Option<String>) -> AppResult<()> {
    settings.set_input_device(id)
}

/// Gera `audio.wav` das sessoes interrompidas (app fechado durante a gravacao).
#[tauri::command]
pub async fn recover_interrupted(
    settings: State<'_, SettingsState>,
    state: State<'_, CaptureState>,
) -> AppResult<Vec<String>> {
    let root = settings.output_dir()?;
    // A sessao em andamento tem temporarios abertos; nao pode ser tocada.
    let active = state.active_session_id();
    let lock = state.1 .0.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = lock.lock().unwrap();
        let mut recovered = Vec::new();
        for s in session::list(&root)? {
            if Some(&s.id) == active.as_ref() {
                continue;
            }
            match recorder::recover(&s.path) {
                Ok(true) => recovered.push(s.id),
                Ok(false) => {}
                Err(e) => eprintln!("[audio] falha ao recuperar {}: {e}", s.id),
            }
        }
        Ok(recovered)
    })
    .await
    .map_err(|e| std::io::Error::other(e.to_string()))?
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Levels {
    pub microphone: f32,
    pub system: f32,
}

/// Pico de cada fonte desde a ultima chamada; a interface consulta ~10 vezes por segundo.
#[tauri::command]
pub fn recording_levels(state: State<CaptureState>) -> Levels {
    let active = state.0.lock().unwrap();
    let mut levels = Levels::default();
    if let Some(r) = active.as_ref() {
        for (kind, level) in r.recorder.take_levels() {
            match kind {
                SourceKind::Microphone => levels.microphone = level,
                SourceKind::System => levels.system = level,
            }
        }
    }
    levels
}
