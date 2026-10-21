//! Localiza o modelo do Whisper na pasta de dados do app.
//! O download e a escolha por hardware ficam para a task de primeira execucao.

use std::path::{Path, PathBuf};

/// Do mais preciso ao mais leve; usa o primeiro que estiver baixado.
/// Ver tabela de modelos em .ai/Plan/estrutura-tecnica-tauri.md.
const PREFERENCE: &[&str] = &["large-v3-turbo", "large-v3", "small"];

pub fn models_dir(app_data: &Path) -> PathBuf {
    app_data.join("models").join("whisper")
}

pub fn find(app_data: &Path) -> Option<PathBuf> {
    let dir = models_dir(app_data);
    PREFERENCE
        .iter()
        .map(|name| dir.join(format!("ggml-{name}.bin")))
        .find(|path| path.is_file())
}
