use serde::Serialize;
use tauri::State;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

use crate::commands::capture::CaptureState;
use crate::error::{AppError, AppResult};
use crate::session::{self, Project, Session};
use crate::settings::SettingsState;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputDir {
    pub path: String,
    pub is_default: bool,
}

fn output_dir_view(settings: &SettingsState) -> AppResult<OutputDir> {
    Ok(OutputDir {
        path: settings.output_dir()?.to_string_lossy().into_owned(),
        is_default: settings.is_default_output_dir(),
    })
}

#[tauri::command]
pub fn list_sessions(settings: State<SettingsState>) -> AppResult<Vec<Session>> {
    session::list(&settings.output_dir()?)
}

#[tauri::command]
pub fn list_projects(settings: State<SettingsState>) -> AppResult<Vec<Project>> {
    session::projects(&settings.output_dir()?)
}

#[tauri::command]
pub fn create_project(settings: State<SettingsState>, name: String) -> AppResult<Project> {
    let root = settings.output_dir()?;
    std::fs::create_dir_all(&root)?;
    session::create_project(&root, &name)
}

#[tauri::command]
pub fn get_session(settings: State<SettingsState>, id: String) -> AppResult<Session> {
    session::get(&settings.output_dir()?, &id)
}

#[tauri::command]
pub fn open_session_folder(
    app: tauri::AppHandle,
    settings: State<SettingsState>,
    id: String,
) -> AppResult<()> {
    let session = session::get(&settings.output_dir()?, &id)?;
    open_path(&app, &session.path)
}

/// Abre um arquivo da sessao no app padrao do sistema. So nomes conhecidos, para nao abrir caminhos arbitrarios.
#[tauri::command]
pub fn open_session_file(
    app: tauri::AppHandle,
    settings: State<SettingsState>,
    id: String,
    file: String,
) -> AppResult<()> {
    const ALLOWED: &[&str] = &["transcricao.md", "analise.md", "acoes.md", "transcricao-pt.md", "audio.wav"];
    let session = session::get(&settings.output_dir()?, &id)?;
    if !ALLOWED.contains(&file.as_str()) || !session.path.join(&file).is_file() {
        return Err(AppError::SessionNotFound(format!("{id}/{file}")));
    }
    open_path(&app, &session.path.join(file))
}

/// Copia o caminho da transcricao para colar na IA. Feito no Rust: o clipboard do WebView nao e confiavel.
#[tauri::command]
pub fn copy_transcript_path(
    app: tauri::AppHandle,
    settings: State<SettingsState>,
    id: String,
) -> AppResult<String> {
    let session = session::get(&settings.output_dir()?, &id)?;
    let path = session.path.join("transcricao.md").to_string_lossy().into_owned();
    app.clipboard().write_text(path.clone()).map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(path)
}

#[tauri::command]
pub fn get_output_dir(settings: State<SettingsState>) -> AppResult<OutputDir> {
    output_dir_view(&settings)
}

/// Abre o seletor de pastas do sistema. Devolve `None` se o usuario cancelar.
/// Async porque o seletor bloqueia ate o usuario escolher.
#[tauri::command]
pub async fn choose_output_dir(
    app: tauri::AppHandle,
    settings: State<'_, SettingsState>,
    capture: State<'_, CaptureState>,
) -> AppResult<Option<OutputDir>> {
    if capture.is_recording() {
        return Err(AppError::ChangeDirWhileRecording);
    }
    let current = settings.output_dir()?;
    let picked = app
        .dialog()
        .file()
        .set_title("Pasta para salvar as gravações e transcrições")
        .set_directory(if current.exists() { current } else { dirs::home_dir().unwrap_or_default() })
        .blocking_pick_folder();

    let Some(picked) = picked else { return Ok(None) };
    let dir = picked.into_path().map_err(|e| std::io::Error::other(e.to_string()))?;
    settings.set_output_dir(Some(dir))?;
    output_dir_view(&settings).map(Some)
}

#[tauri::command]
pub fn reset_output_dir(
    settings: State<SettingsState>,
    capture: State<CaptureState>,
) -> AppResult<OutputDir> {
    if capture.is_recording() {
        return Err(AppError::ChangeDirWhileRecording);
    }
    settings.set_output_dir(None)?;
    output_dir_view(&settings)
}

#[tauri::command]
pub fn open_output_dir(app: tauri::AppHandle, settings: State<SettingsState>) -> AppResult<()> {
    let dir = settings.output_dir()?;
    std::fs::create_dir_all(&dir)?;
    open_path(&app, &dir)
}

fn open_path(app: &tauri::AppHandle, path: &std::path::Path) -> AppResult<()> {
    app.opener()
        .open_path(path.to_string_lossy(), None::<&str>)
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(())
}
