//! Download do modelo do Whisper na primeira execucao, com progresso para a interface.

use std::fs::{self, File};
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::error::{AppError, AppResult};
use crate::whisper::models;

/// Impede dois downloads ao mesmo tempo.
#[derive(Default)]
pub struct ModelState(AtomicBool);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStatus {
    /// `None` enquanto nenhum modelo foi baixado.
    installed: Option<String>,
    recommended: &'static str,
    recommended_size_mb: u32,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Progress {
    downloaded: u64,
    /// `None` se o servidor nao informar o tamanho.
    total: Option<u64>,
}

fn app_data(app: &AppHandle) -> AppResult<std::path::PathBuf> {
    app.path().app_data_dir().map_err(|e| std::io::Error::other(e.to_string()).into())
}

#[tauri::command]
pub fn model_status(app: AppHandle) -> AppResult<ModelStatus> {
    Ok(ModelStatus {
        installed: models::installed(&app_data(&app)?),
        recommended: models::RECOMMENDED,
        recommended_size_mb: models::size_mb(models::RECOMMENDED),
    })
}

/// Baixa o modelo recomendado emitindo `model-download-progress`. Grava num `.part` e so
/// renomeia no fim: um download interrompido nunca vira um modelo corrompido.
#[tauri::command]
pub async fn download_model(app: AppHandle, state: State<'_, ModelState>) -> AppResult<ModelStatus> {
    if state.0.swap(true, Ordering::SeqCst) {
        return Err(AppError::AlreadyDownloading);
    }
    let result = download(&app).await;
    state.0.store(false, Ordering::SeqCst);
    result?;
    model_status(app)
}

async fn download(app: &AppHandle) -> AppResult<()> {
    let dir = models::models_dir(&app_data(app)?);
    fs::create_dir_all(&dir)?;
    let target = dir.join(models::file_name(models::RECOMMENDED));
    let part = target.with_extension("bin.part");

    let result = async {
        let mut response = reqwest::get(models::url(models::RECOMMENDED)).await?.error_for_status()?;
        let total = response.content_length();
        let mut file = File::create(&part)?;
        let mut downloaded = 0u64;
        let mut last_emit = 0u64;
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            // Um evento a cada ~4 MB basta para a barra andar sem inundar a interface.
            if downloaded - last_emit >= 4 << 20 {
                last_emit = downloaded;
                let _ = app.emit("model-download-progress", Progress { downloaded, total });
            }
        }
        file.sync_all()?;
        if total.is_some_and(|t| t != downloaded) {
            return Err(AppError::DownloadIncomplete);
        }
        fs::rename(&part, &target)?;
        Ok(())
    }
    .await;

    if result.is_err() {
        let _ = fs::remove_file(&part);
    }
    result
}
