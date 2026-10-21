//! Organizacao das sessoes na pasta de saida (padrao: ~/ReCapt), TASK-09 e TASK-10:
//!
//! ```text
//! <raiz>/<Projeto>/<Frente>/<Tema>/AAAA-MM-DD_HHhMM Assunto/   sessoes organizadas
//! <raiz>/<Projeto>/<Frente>/AAAA-MM-DD_HHhMM/                   gravadas antes do Tema
//! <raiz>/AAAA-MM/AAAA-MM-DD_HHhMM/                               sessoes antigas, sem projeto
//! ```
//!
//! O assunto e opcional e fica no nome da pasta da reuniao, depois da data.
//!
//! Projetos sao pastas com o arquivo `.recapt-projeto`, para que outras pastas da raiz
//! nao virem projetos por engano. Frentes sao criadas na primeira gravacao.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::error::{AppError, AppResult};

const PROJECT_MARKER: &str = ".recapt-projeto";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// Caminho relativo a raiz, ex.: "Cliente X/Discovery/2026-10-20_15h40".
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    /// `None` nas sessoes antigas, gravadas antes da TASK-09.
    pub project: Option<String>,
    pub front: Option<String>,
    /// `None` nas sessoes gravadas antes da TASK-10, direto na frente.
    pub theme: Option<String>,
    /// Assunto opcional, lido do nome da pasta ("2026-10-21_14h30 Mapeamento de campos").
    pub subject: Option<String>,
    pub has_audio: bool,
    pub has_video: bool,
    pub has_transcript: bool,
    pub has_analysis: bool,
    /// Duracao do audio.wav, lida do cabecalho; `None` sem audio.
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    /// Em ordem alfabetica.
    pub fronts: Vec<Front>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Front {
    pub name: String,
    /// Em ordem alfabetica.
    pub themes: Vec<String>,
}

/// Onde a sessao esta na hierarquia; tudo `None` nas sessoes antigas por mes.
#[derive(Default)]
struct Place {
    project: Option<String>,
    front: Option<String>,
    theme: Option<String>,
}

/// Remove o que nao pode ir em nome de pasta e normaliza espacos. Acentos ficam.
pub fn clean_name(raw: &str) -> AppResult<String> {
    let cleaned: String = raw
        .chars()
        .map(|c| if matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') || c.is_control() { ' ' } else { c })
        .collect();
    let cleaned = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
    // Ponto no inicio esconde a pasta no Finder; ".." sairia da raiz.
    let cleaned = cleaned.trim_start_matches('.').trim().to_string();
    if cleaned.is_empty() {
        return Err(AppError::InvalidName(raw.trim().to_string()));
    }
    if cleaned.chars().count() > 80 {
        return Err(AppError::NameTooLong);
    }
    Ok(cleaned)
}

pub fn create_project(root: &Path, name: &str) -> AppResult<Project> {
    let name = clean_name(name)?;
    // Comparacao sem diferenciar maiusculas: o sistema de arquivos do macOS tambem nao diferencia.
    if projects(root)?.iter().any(|p| p.name.to_lowercase() == name.to_lowercase()) {
        return Err(AppError::ProjectExists(name));
    }
    let dir = root.join(&name);
    fs::create_dir_all(&dir)?;
    fs::write(dir.join(PROJECT_MARKER), b"")?;
    Ok(Project { name, fronts: Vec::new() })
}

pub fn projects(root: &Path) -> AppResult<Vec<Project>> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for dir in subdirs(root)?.into_iter().filter(|d| is_project_dir(d)) {
        let mut fronts = Vec::new();
        for front in subdirs(&dir)? {
            // Pastas de data direto na frente sao sessoes antigas, nao temas.
            let mut themes: Vec<String> =
                subdirs(&front)?.iter().filter(|d| !is_session_dir(d)).filter_map(|d| dir_name(d)).collect();
            themes.sort_by_key(|t| t.to_lowercase());
            fronts.push(Front { name: dir_name(&front).unwrap_or_default(), themes });
        }
        fronts.sort_by_key(|f| f.name.to_lowercase());
        out.push(Project { name: dir_name(&dir).unwrap_or_default(), fronts });
    }
    out.sort_by_key(|p| p.name.to_lowercase());
    Ok(out)
}

pub struct NewSession<'a> {
    pub project: &'a str,
    pub front: &'a str,
    pub theme: &'a str,
    /// Opcional; vazio fica so com data e hora.
    pub subject: &'a str,
}

/// Cria a pasta da sessao: <raiz>/<projeto>/<frente>/<tema>/AAAA-MM-DD_HHhMM Assunto.
/// O projeto precisa existir; frente e tema sao criados se forem novos, reaproveitando a grafia existente.
pub fn create(root: &Path, new: &NewSession, started_at: DateTime<Local>) -> AppResult<Session> {
    let project_dir = root.join(clean_name(new.project)?);
    if !is_project_dir(&project_dir) {
        return Err(AppError::ProjectNotFound(new.project.to_string()));
    }
    let front = existing_or(&project_dir, clean_name(new.front)?)?;
    let front_dir = project_dir.join(&front);
    let theme = existing_or(&front_dir, clean_name(new.theme)?)?;
    let theme_dir = front_dir.join(&theme);
    let subject = match new.subject.trim() {
        "" => None,
        raw => Some(clean_name(raw)?),
    };

    let base = started_at.format("%Y-%m-%d_%Hh%M").to_string();
    let with_subject = |stamp: &str| match &subject {
        Some(s) => format!("{stamp} {s}"),
        None => stamp.to_string(),
    };
    // Duas gravacoes no mesmo minuto recebem sufixo -2, -3... antes do assunto.
    let mut name = with_subject(&base);
    let mut n = 2;
    while theme_dir.join(&name).exists() {
        name = with_subject(&format!("{base}-{n}"));
        n += 1;
    }

    let path = theme_dir.join(&name);
    fs::create_dir_all(&path)?;
    let place = Place { project: dir_name(&project_dir), front: Some(front), theme: Some(theme) };
    Ok(read(root, &path, place))
}

/// "discovery" reaproveita a pasta "Discovery" em vez de criar outra.
fn existing_or(parent: &Path, name: String) -> AppResult<String> {
    if !parent.exists() {
        return Ok(name);
    }
    Ok(subdirs(parent)?
        .iter()
        .filter_map(|d| dir_name(d))
        .find(|f| f.to_lowercase() == name.to_lowercase())
        .unwrap_or(name))
}

pub fn list(root: &Path) -> AppResult<Vec<Session>> {
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();
    for dir in subdirs(root)? {
        if is_project_dir(&dir) {
            for front in subdirs(&dir)? {
                for child in subdirs(&front)? {
                    let place = |theme: Option<String>| Place { project: dir_name(&dir), front: dir_name(&front), theme };
                    if is_session_dir(&child) {
                        // Gravada antes da TASK-10, direto na frente.
                        sessions.push(read(root, &child, place(None)));
                    } else {
                        for session in subdirs(&child)?.into_iter().filter(|d| is_session_dir(d)) {
                            sessions.push(read(root, &session, place(dir_name(&child))));
                        }
                    }
                }
            }
        } else if is_month_dir(&dir) {
            for session in subdirs(&dir)?.into_iter().filter(|d| is_session_dir(d)) {
                sessions.push(read(root, &session, Place::default()));
            }
        }
        // Outras pastas da raiz sao ignoradas.
    }
    // Nomes comecam com a data: ordenar pelo nome deixa as mais recentes primeiro em todos os projetos.
    sessions.sort_by(|a, b| b.name.cmp(&a.name).then_with(|| a.id.cmp(&b.id)));
    Ok(sessions)
}

pub fn get(root: &Path, id: &str) -> AppResult<Session> {
    let path = root.join(id);
    // Impede ids como "../../etc" de sair da raiz.
    if id.split(['/', '\\']).any(|part| part == "..") || !path.is_dir() {
        return Err(AppError::SessionNotFound(id.to_string()));
    }
    let parts: Vec<&str> = id.split('/').collect();
    let owned = |s: &&str| Some(s.to_string());
    let place = match parts.as_slice() {
        [project, front, theme, _] if is_project_dir(&root.join(project)) => {
            Place { project: owned(project), front: owned(front), theme: owned(theme) }
        }
        [project, front, _] if is_project_dir(&root.join(project)) => {
            Place { project: owned(project), front: owned(front), theme: None }
        }
        _ => Place::default(),
    };
    Ok(read(root, &path, place))
}

/// So le o cabecalho: barato mesmo para gravacoes longas.
fn audio_duration_ms(path: &Path) -> Option<u64> {
    let reader = hound::WavReader::open(path).ok()?;
    let rate = reader.spec().sample_rate as u64;
    (rate > 0).then(|| reader.duration() as u64 * 1000 / rate)
}

fn is_project_dir(dir: &Path) -> bool {
    dir.join(PROJECT_MARKER).is_file()
}

fn is_month_dir(dir: &Path) -> bool {
    let Some(name) = dir.file_name().and_then(|n| n.to_str()) else { return false };
    let b = name.as_bytes();
    b.len() == 7 && b[4] == b'-' && b[..4].iter().chain(&b[5..]).all(u8::is_ascii_digit)
}

/// Pastas de sessao comecam com AAAA-MM-DD.
fn is_session_dir(dir: &Path) -> bool {
    let Some(name) = dir.file_name().and_then(|n| n.to_str()) else { return false };
    let b = name.as_bytes();
    b.len() >= 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b[..4].iter().chain(&b[5..7]).chain(&b[8..10]).all(u8::is_ascii_digit)
}

/// "2026-10-21_14h30-2 Mapeamento de campos" -> "Mapeamento de campos".
pub fn subject_of(folder: &str) -> Option<String> {
    folder.split_once(' ').map(|(_, s)| s.trim().to_string()).filter(|s| !s.is_empty())
}

/// "2026-10-21_14h30 Mapeamento de campos" -> "2026-10-21_14h30".
pub fn stamp_of(folder: &str) -> &str {
    folder.split_once(' ').map_or(folder, |(stamp, _)| stamp)
}

fn dir_name(dir: &Path) -> Option<String> {
    dir.file_name().map(|n| n.to_string_lossy().into_owned())
}

fn subdirs(dir: &Path) -> AppResult<Vec<PathBuf>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        // Pastas ocultas (".algo") nao sao projetos, frentes nem sessoes.
        let hidden = path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with('.'));
        if path.is_dir() && !hidden {
            out.push(path);
        }
    }
    Ok(out)
}

fn read(root: &Path, path: &Path, place: Place) -> Session {
    let id = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    let name = dir_name(path).unwrap_or_default();
    Session {
        id,
        subject: subject_of(&name),
        name,
        path: path.to_path_buf(),
        project: place.project,
        front: place.front,
        theme: place.theme,
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
    use chrono::TimeZone;

    fn temp_root(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("recapt-session-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn at(h: u32, m: u32) -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 10, 21, h, m, 0).unwrap()
    }

    #[test]
    fn clean_name_keeps_accents_and_removes_forbidden_chars() {
        assert_eq!(clean_name("  Migração: Fase 1/2  ").unwrap(), "Migração Fase 1 2");
        assert_eq!(clean_name("..oculto").unwrap(), "oculto");
        assert!(clean_name(" / : ").is_err());
        assert!(clean_name(&"a".repeat(81)).is_err());
    }

    #[test]
    fn creates_sessions_inside_project_and_front() {
        let root = temp_root("create");
        create_project(&root, "Cliente X").unwrap();
        assert!(matches!(create_project(&root, "cliente x"), Err(AppError::ProjectExists(_))));

        let new = |front, theme, subject| NewSession { project: "Cliente X", front, theme, subject };

        let s = create(&root, &new("Discovery", "Integração com SAP", "Mapeamento de campos"), at(10, 27)).unwrap();
        assert_eq!(s.id, "Cliente X/Discovery/Integração com SAP/2026-10-21_10h27 Mapeamento de campos");
        assert_eq!(s.theme.as_deref(), Some("Integração com SAP"));
        assert_eq!(s.subject.as_deref(), Some("Mapeamento de campos"));

        // Mesma frente e tema com outra grafia, mesmo minuto, sem assunto: reaproveita as pastas.
        let s2 = create(&root, &new("discovery", "integração com sap", ""), at(10, 27)).unwrap();
        assert_eq!(s2.id, "Cliente X/Discovery/Integração com SAP/2026-10-21_10h27");
        assert_eq!(s2.subject, None);
        let s3 = create(&root, &new("Discovery", "Integração com SAP", ""), at(10, 27)).unwrap();
        assert_eq!(s3.id, "Cliente X/Discovery/Integração com SAP/2026-10-21_10h27-2");

        let other = NewSession { project: "Nao existe", front: "F", theme: "T", subject: "" };
        assert!(matches!(create(&root, &other, at(9, 0)), Err(AppError::ProjectNotFound(_))));

        let projects = projects(&root).unwrap();
        assert_eq!(projects[0].fronts.len(), 1);
        assert_eq!(projects[0].fronts[0].name, "Discovery");
        assert_eq!(projects[0].fronts[0].themes, ["Integração com SAP"]);

        let got = get(&root, &s.id).unwrap();
        assert_eq!(got.theme.as_deref(), Some("Integração com SAP"));
        assert_eq!(got.subject.as_deref(), Some("Mapeamento de campos"));
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn lists_projects_and_legacy_sessions_ignoring_other_folders() {
        let root = temp_root("list");
        create_project(&root, "Cliente X").unwrap();
        let new = NewSession { project: "Cliente X", front: "Discovery", theme: "SAP", subject: "" };
        create(&root, &new, at(10, 27)).unwrap();
        // Gravada antes da TASK-10, direto na frente.
        fs::create_dir_all(root.join("Cliente X/Discovery/2026-10-20_09h00")).unwrap();
        fs::create_dir_all(root.join("2026-09/2026-09-30_09h00")).unwrap();
        fs::create_dir_all(root.join("Pasta qualquer/sub/2026-10-01_10h00")).unwrap();

        let ids: Vec<_> = list(&root).unwrap().into_iter().map(|s| (s.id, s.theme)).collect();
        assert_eq!(
            ids,
            [
                ("Cliente X/Discovery/SAP/2026-10-21_10h27".to_string(), Some("SAP".to_string())),
                ("Cliente X/Discovery/2026-10-20_09h00".to_string(), None),
                ("2026-09/2026-09-30_09h00".to_string(), None),
            ]
        );
        // A sessao antiga direto na frente nao vira tema.
        assert_eq!(projects(&root).unwrap()[0].fronts[0].themes, ["SAP"]);
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn splits_stamp_and_subject() {
        assert_eq!(subject_of("2026-10-21_14h30-2 Mapeamento de campos").as_deref(), Some("Mapeamento de campos"));
        assert_eq!(subject_of("2026-10-21_14h30"), None);
        assert_eq!(stamp_of("2026-10-21_14h30 Mapeamento"), "2026-10-21_14h30");
    }

    #[test]
    fn get_rejects_paths_outside_root() {
        let root = temp_root("get");
        assert!(get(&root, "../etc").is_err());
        fs::remove_dir_all(&root).unwrap();
    }
}
