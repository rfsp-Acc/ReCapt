//! Pastas de sessao em <pasta de saida>/AAAA-MM/AAAA-MM-DD_HHhMM (padrao: ~/ReCapt).
//! Ver .ai/Plan/nomenclatura-sessoes.md.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// Caminho relativo a raiz, ex.: "2026-10/2026-10-20_15h40".
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub has_audio: bool,
    pub has_video: bool,
    pub has_transcript: bool,
    pub has_analysis: bool,
    /// Duracao do audio.wav, lida do cabecalho; `None` sem audio.
    pub duration_ms: Option<u64>,
}

/// Cria a pasta da sessao com nome temporario baseado na data e hora.
pub fn create(root: &Path, started_at: DateTime<Local>) -> AppResult<Session> {
    let month = started_at.format("%Y-%m").to_string();
    let base = started_at.format("%Y-%m-%d_%Hh%M").to_string();

    // Duas gravacoes no mesmo minuto recebem sufixo -2, -3...
    let mut name = base.clone();
    let mut n = 2;
    while root.join(&month).join(&name).exists() {
        name = format!("{base}-{n}");
        n += 1;
    }

    let path = root.join(&month).join(&name);
    fs::create_dir_all(&path)?;
    Ok(read(root, &path))
}

pub fn list(root: &Path) -> AppResult<Vec<Session>> {
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();
    // A pasta de saida pode ter outros conteudos; so entram as pastas de mes AAAA-MM.
    for month in subdirs(root)?.into_iter().filter(|d| is_month_dir(d)) {
        for dir in subdirs(&month)? {
            sessions.push(read(root, &dir));
        }
    }
    // Nomes comecam com a data, entao a ordem alfabetica inversa deixa as mais recentes primeiro.
    sessions.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(sessions)
}

pub fn get(root: &Path, id: &str) -> AppResult<Session> {
    let path = root.join(id);
    // Impede ids como "../../etc" de sair da raiz.
    if id.contains("..") || !path.is_dir() {
        return Err(AppError::SessionNotFound(id.to_string()));
    }
    Ok(read(root, &path))
}

/// So le o cabecalho: barato mesmo para gravacoes longas.
fn audio_duration_ms(path: &Path) -> Option<u64> {
    let reader = hound::WavReader::open(path).ok()?;
    let rate = reader.spec().sample_rate as u64;
    (rate > 0).then(|| reader.duration() as u64 * 1000 / rate)
}

fn is_month_dir(dir: &Path) -> bool {
    let Some(name) = dir.file_name().and_then(|n| n.to_str()) else { return false };
    let b = name.as_bytes();
    b.len() == 7 && b[4] == b'-' && b[..4].iter().chain(&b[5..]).all(u8::is_ascii_digit)
}

fn subdirs(dir: &Path) -> AppResult<Vec<PathBuf>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            out.push(path);
        }
    }
    Ok(out)
}

fn read(root: &Path, path: &Path) -> Session {
    let id = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();

    Session {
        id,
        name,
        path: path.to_path_buf(),
        has_audio: path.join("audio.wav").exists() || path.join("audio.mp3").exists(),
        has_video: path.join("gravacao.mp4").exists(),
        has_transcript: path.join("transcricao.md").exists(),
        has_analysis: path.join("analise.md").exists(),
        duration_ms: audio_duration_ms(&path.join("audio.wav")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_only_sessions_inside_month_dirs() {
        let root = std::env::temp_dir().join(format!("recapt-session-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("2026-10/2026-10-21_10h27")).unwrap();
        fs::create_dir_all(root.join("2026-09/2026-09-30_09h00")).unwrap();
        fs::create_dir_all(root.join("Projetos/qualquer-coisa")).unwrap();

        let ids: Vec<_> = list(&root).unwrap().into_iter().map(|s| s.id).collect();
        assert_eq!(ids, ["2026-10/2026-10-21_10h27", "2026-09/2026-09-30_09h00"]);
        fs::remove_dir_all(&root).unwrap();
    }
}
