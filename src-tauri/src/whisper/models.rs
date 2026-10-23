//! Localiza o modelo do Whisper na pasta de dados do app e diz qual baixar.

use std::path::{Path, PathBuf};

/// Do mais preciso ao mais leve; usa o primeiro que estiver baixado.
/// Ver tabela de modelos em .ai/Plan/estrutura-tecnica-tauri.md.
const PREFERENCE: &[&str] = &["large-v3-turbo", "large-v3", "small"];

/// Sugerido na primeira execucao: o turbo so e rapido com Metal; na CPU, o small.
pub const RECOMMENDED: &str =
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) { "large-v3-turbo" } else { "small" };

/// Tamanho aproximado, para avisar antes do download.
pub fn size_mb(name: &str) -> u32 {
    match name {
        "large-v3-turbo" => 1_620,
        "large-v3" => 3_100,
        _ => 488,
    }
}

pub fn models_dir(app_data: &Path) -> PathBuf {
    app_data.join("models").join("whisper")
}

pub fn file_name(name: &str) -> String {
    format!("ggml-{name}.bin")
}

pub fn url(name: &str) -> String {
    format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}", file_name(name))
}

pub fn find(app_data: &Path) -> Option<PathBuf> {
    let dir = models_dir(app_data);
    PREFERENCE.iter().map(|name| dir.join(file_name(name))).find(|path| path.is_file())
}

/// Nome do modelo instalado (ex.: "small"), pelo arquivo encontrado.
pub fn installed(app_data: &Path) -> Option<String> {
    let path = find(app_data)?;
    let stem = path.file_stem()?.to_str()?;
    stem.strip_prefix("ggml-").map(str::to_string)
}
