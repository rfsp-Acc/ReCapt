use std::collections::HashSet;
use std::fs;
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::error::{AppError, AppResult};
use crate::session::{self, Session};
use crate::settings::SettingsState;
use crate::whisper::language::Language;
use crate::whisper::{engine, models, transcript};

/// Sessoes sendo transcritas agora; evita rodar duas vezes a mesma.
#[derive(Default)]
pub struct TranscribeState(Mutex<HashSet<String>>);

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Progress {
    session_id: String,
    percent: i32,
}

/// Emite `transcription-progress` durante o processamento e devolve a sessao atualizada.
#[tauri::command]
pub async fn transcribe(
    app: AppHandle,
    state: State<'_, TranscribeState>,
    settings: State<'_, SettingsState>,
    id: String,
) -> AppResult<Session> {
    // A sessao fica na pasta escolhida no inicio, mesmo que ela mude durante a transcricao.
    let root = settings.output_dir()?;
    let language = settings.transcription_language();
    let session = session::get(&root, &id)?;
    let audio = session.path.join("audio.wav");
    if !audio.is_file() {
        return Err(AppError::NoAudio);
    }

    let app_data = app.path().app_data_dir().map_err(|e| std::io::Error::other(e.to_string()))?;
    let model = models::find(&app_data)
        .ok_or_else(|| AppError::NoWhisperModel(models::models_dir(&app_data)))?;

    if !state.0.lock().unwrap().insert(id.clone()) {
        return Err(AppError::AlreadyTranscribing);
    }

    let emitter = app.clone();
    let progress_id = id.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        engine::transcribe(&model, &audio, language, move |percent| {
            let _ = emitter.emit(
                "transcription-progress",
                Progress { session_id: progress_id.clone(), percent },
            );
        })
    })
    .await;
    state.0.lock().unwrap().remove(&id);

    let result = result.map_err(|e| std::io::Error::other(e.to_string()))??;
    let context = transcript::Context {
        project: session.project.as_deref(),
        front: session.front.as_deref(),
        theme: session.theme.as_deref(),
        subject: session.subject.as_deref(),
    };
    // O titulo usa so data e hora; o assunto ja entra pelo contexto.
    let title = transcript::display_name(session::stamp_of(&session.name));
    let md = transcript::to_markdown(&title, &context, &result);
    fs::write(session.path.join("transcricao.md"), md)?;

    session::get(&root, &id)
}

#[tauri::command]
pub fn get_transcription_language(settings: State<SettingsState>) -> Language {
    settings.transcription_language()
}

#[tauri::command]
pub fn set_transcription_language(
    settings: State<SettingsState>,
    language: Language,
) -> AppResult<Language> {
    settings.set_transcription_language(language)?;
    Ok(language)
}
