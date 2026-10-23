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
//!
//! Uma frente pode gravar numa pasta propria (TASK-11), ex.: `.ai/docs/reunioes` de um repositorio.
//! O caminho fica no arquivo `.recapt-pasta` dentro da pasta da frente na raiz, e as reunioes vao para
//! `<pasta da frente>/<Tema>/AAAA-MM-DD_HHhMM Assunto/`. O id continua `<Projeto>/<Frente>/<Tema>/<pasta>`,
//! entao a interface nao precisa saber onde a sessao esta. Sessoes gravadas antes na raiz continuam listadas.
//!
//! Uma sessao movida para fora da estrutura deixa na pasta do tema, na raiz, o atalho oculto
//! `.<pasta>.recapt-local` com o caminho real; ela continua no mesmo projeto, frente e tema no app.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::error::{AppError, AppResult};

const PROJECT_MARKER: &str = ".recapt-projeto";
const FRONT_LINK: &str = ".recapt-pasta";
const POINTER_SUFFIX: &str = ".recapt-local";

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
    /// Pasta propria da frente; `None` grava dentro da raiz.
    pub dir: Option<PathBuf>,
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
            let linked = linked_dir(&front);
            // Pastas de data direto na frente sao sessoes antigas, nao temas.
            let mut themes: Vec<String> = Vec::new();
            for base in std::iter::once(&front).chain(linked.iter().filter(|d| d.is_dir())) {
                for theme in subdirs(base)?.iter().filter(|d| !is_session_dir(d)).filter_map(|d| dir_name(d)) {
                    if !themes.iter().any(|t| t.to_lowercase() == theme.to_lowercase()) {
                        themes.push(theme);
                    }
                }
            }
            themes.sort_by_key(|t| t.to_lowercase());
            fronts.push(Front { name: dir_name(&front).unwrap_or_default(), dir: linked, themes });
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

/// Onde uma sessao vai ficar: projeto, frente e tema com a grafia existente.
struct Destination {
    project: String,
    front: String,
    theme: String,
    /// Dentro da raiz ou da pasta propria da frente.
    theme_dir: PathBuf,
    subject: Option<String>,
}

impl Destination {
    /// O projeto precisa existir; frente e tema sao criados se forem novos, reaproveitando a grafia existente.
    fn resolve(root: &Path, new: &NewSession) -> AppResult<Self> {
        let project_dir = root.join(clean_name(new.project)?);
        if !is_project_dir(&project_dir) {
            return Err(AppError::ProjectNotFound(new.project.to_string()));
        }
        let front = existing_or(&project_dir, clean_name(new.front)?)?;
        let front_dir = project_dir.join(&front);
        let base_dir = linked_dir(&front_dir).unwrap_or(front_dir);
        let theme = existing_or(&base_dir, clean_name(new.theme)?)?;
        let subject = match new.subject.trim() {
            "" => None,
            raw => Some(clean_name(raw)?),
        };
        Ok(Self {
            project: dir_name(&project_dir).unwrap_or_default(),
            front,
            theme_dir: base_dir.join(&theme),
            theme,
            subject,
        })
    }

    /// Duas reunioes no mesmo minuto recebem sufixo -2, -3... antes do assunto.
    /// `current` e a propria sessao sendo movida: manter o mesmo nome nao e conflito.
    fn folder_name(&self, stamp: &str, current: Option<&Path>) -> String {
        let with_subject = |stamp: &str| match &self.subject {
            Some(s) => format!("{stamp} {s}"),
            None => stamp.to_string(),
        };
        let free = |name: &str| {
            let path = self.theme_dir.join(name);
            // macOS nao diferencia maiusculas: "assunto" -> "Assunto" e a mesma pasta.
            !path.exists() || current.is_some_and(|c| c.to_string_lossy().to_lowercase() == path.to_string_lossy().to_lowercase())
        };
        let mut name = with_subject(stamp);
        let mut n = 2;
        while !free(&name) {
            name = with_subject(&format!("{stamp}-{n}"));
            n += 1;
        }
        name
    }

    fn session(self, path: &Path, name: &str) -> Session {
        let id = format!("{}/{}/{}/{name}", self.project, self.front, self.theme);
        read(path, id, Place { project: Some(self.project), front: Some(self.front), theme: Some(self.theme) })
    }
}

/// Cria a pasta da sessao: <raiz>/<projeto>/<frente>/<tema>/AAAA-MM-DD_HHhMM Assunto.
pub fn create(root: &Path, new: &NewSession, started_at: DateTime<Local>) -> AppResult<Session> {
    let dest = Destination::resolve(root, new)?;
    let name = dest.folder_name(&started_at.format("%Y-%m-%d_%Hh%M").to_string(), None);
    let path = dest.theme_dir.join(&name);
    fs::create_dir_all(&path)?;
    Ok(dest.session(&path, &name))
}

/// Para onde vai a pasta de uma sessao movida.
pub enum Location<'a> {
    /// Pasta do tema na estrutura (raiz ou pasta propria da frente).
    Structure,
    /// Fora da estrutura: a sessao vai para la e continua no mesmo projeto, frente e tema no app.
    Outside(&'a Path),
    /// Onde ela ja esta; usado para trocar so o assunto.
    Keep,
}

/// Move a sessao para outro projeto, frente ou tema, ou troca o assunto. A pasta e movida com
/// todos os arquivos, mantendo data e hora no nome; o cabecalho do transcricao.md acompanha.
pub fn move_to(root: &Path, id: &str, new: &NewSession, place: Location) -> AppResult<Session> {
    let current = get(root, id)?;
    let old_pointer = pointer_of(root, &current).filter(|p| p.is_file());
    let mut dest = Destination::resolve(root, new)?;
    let is_outside = match place {
        Location::Structure => false,
        Location::Outside(dir) => {
            dest.theme_dir = dir.to_path_buf();
            true
        }
        // Sessao fora da estrutura: renomeia onde ela esta.
        Location::Keep if old_pointer.is_some() => {
            dest.theme_dir = current.path.parent().map(Path::to_path_buf).unwrap_or_default();
            true
        }
        Location::Keep => false,
    };
    let name = dest.folder_name(base_stamp(stamp_of(&current.name)), Some(&current.path));
    let path = dest.theme_dir.join(&name);

    if path != current.path {
        fs::create_dir_all(&dest.theme_dir)?;
        move_dir(&current.path, &path)?;
    }
    if let Some(old) = &old_pointer {
        fs::remove_file(old)?;
    }
    if is_outside {
        let pointer = pointer_path(root, &dest.project, &dest.front, &dest.theme, &name);
        if let Some(parent) = pointer.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(pointer, path.to_string_lossy().as_bytes())?;
    }
    // Tema que ficou vazio deixaria de ser util na barra lateral.
    tidy(root, &current);

    let transcript = path.join("transcricao.md");
    if let Ok(md) = fs::read_to_string(&transcript) {
        let context = crate::whisper::transcript::Context {
            project: Some(&dest.project),
            front: Some(&dest.front),
            theme: Some(&dest.theme),
            subject: dest.subject.as_deref(),
        };
        let title = crate::whisper::transcript::display_name(stamp_of(&name));
        fs::write(&transcript, crate::whisper::transcript::retitle(&md, &title, &context))?;
    }
    Ok(dest.session(&path, &name))
}

/// Move para a pasta escolhida no Finder, mantendo o assunto.
/// Pasta de um tema da estrutura: projeto, frente e tema saem dela. Pasta fora da estrutura: a sessao
/// vai para la e continua no mesmo projeto, frente e tema no app.
pub fn move_to_folder(root: &Path, id: &str, picked: &Path) -> AppResult<Session> {
    let current = get(root, id)?;
    let subject = current.subject.clone().unwrap_or_default();
    let same = |a: &Path, b: &Path| a.to_string_lossy().to_lowercase() == b.to_string_lossy().to_lowercase();
    let inside = |a: &Path, b: &Path| a.to_string_lossy().to_lowercase().starts_with(&b.to_string_lossy().to_lowercase());

    if inside(picked, &current.path) {
        return Err(AppError::InvalidDestination("a pasta escolhida está dentro da própria reunião".into()));
    }
    let mut structure = vec![root.to_path_buf()];
    for project in projects(root)? {
        for front in &project.fronts {
            let bases = std::iter::once(root.join(&project.name).join(&front.name)).chain(front.dir.clone());
            for base in bases {
                for theme in &front.themes {
                    if same(picked, &base.join(theme)) {
                        let new = NewSession { project: &project.name, front: &front.name, theme, subject: &subject };
                        return move_to(root, id, &new, Location::Structure);
                    }
                }
                structure.push(base);
            }
        }
    }
    if structure.iter().any(|dir| inside(picked, dir)) {
        return Err(AppError::InvalidDestination(
            "dentro das pastas do ReCapt, escolha a pasta de um tema".into(),
        ));
    }
    let (Some(project), Some(front), Some(theme)) = (&current.project, &current.front, &current.theme) else {
        return Err(AppError::InvalidDestination(
            "esta reunião não tem projeto, frente ou tema; use Renomear para completar e depois mova".into(),
        ));
    };
    let new = NewSession { project, front, theme, subject: &subject };
    move_to(root, id, &new, Location::Outside(picked))
}

/// Apaga a pasta da sessao com todos os arquivos, sem passar pela Lixeira.
pub fn delete(root: &Path, id: &str) -> AppResult<()> {
    let current = get(root, id)?;
    fs::remove_dir_all(&current.path)?;
    if let Some(pointer) = pointer_of(root, &current).filter(|p| p.is_file()) {
        fs::remove_file(pointer)?;
    }
    tidy(root, &current);
    Ok(())
}

/// Sessao guardada fora da estrutura: atalho oculto na pasta do tema dentro da raiz, com o caminho real.
fn pointer_path(root: &Path, project: &str, front: &str, theme: &str, folder: &str) -> PathBuf {
    root.join(project).join(front).join(theme).join(format!(".{folder}{POINTER_SUFFIX}"))
}

fn pointer_of(root: &Path, s: &Session) -> Option<PathBuf> {
    Some(pointer_path(root, s.project.as_deref()?, s.front.as_deref()?, s.theme.as_deref()?, &s.name))
}

/// Sessoes fora da estrutura apontadas por um tema; as indisponiveis (disco externo) ficam de fora.
fn pointed(theme_dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(theme_dir) else { return Vec::new() };
    entries
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().ends_with(POINTER_SUFFIX))
        .filter_map(|e| fs::read_to_string(e.path()).ok())
        .map(|raw| PathBuf::from(raw.trim()))
        .filter(|p| p.is_absolute() && p.is_dir())
        .collect()
}

/// Remove as pastas do tema da sessao (na raiz e na pasta propria da frente) se ficaram vazias.
/// Nunca toca pastas fora da estrutura.
fn tidy(root: &Path, s: &Session) {
    let (Some(project), Some(front), Some(theme)) = (&s.project, &s.front, &s.theme) else { return };
    let front_dir = root.join(project).join(front);
    if let Some(linked) = linked_dir(&front_dir) {
        remove_if_empty(&linked.join(theme));
    }
    remove_if_empty(&front_dir.join(theme));
}

/// "2026-10-21_14h30-2" -> "2026-10-21_14h30": o sufixo de desempate e refeito no destino.
fn base_stamp(stamp: &str) -> &str {
    match stamp.rsplit_once('-') {
        Some((base, n)) if base.len() == 16 && n.bytes().all(|b| b.is_ascii_digit()) => base,
        _ => stamp,
    }
}

/// `rename` nao atravessa discos (ex.: raiz no Mac e pasta da frente num disco externo); nesse caso copia e apaga.
fn move_dir(from: &Path, to: &Path) -> AppResult<()> {
    if fs::rename(from, to).is_ok() {
        return Ok(());
    }
    if let Err(e) = copy_dir(from, to) {
        let _ = fs::remove_dir_all(to);
        return Err(e.into());
    }
    fs::remove_dir_all(from)?;
    Ok(())
}

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

/// Apaga a pasta se so tiver arquivos do Finder (`.DS_Store`) ou do Explorer (`Thumbs.db`, `desktop.ini`).
fn remove_if_empty(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    if entries.flatten().all(|e| matches!(e.file_name().to_str(), Some(".DS_Store" | "Thumbs.db" | "desktop.ini"))) {
        let _ = fs::remove_dir_all(dir);
    }
}

/// Define a pasta propria da frente (`None` volta a gravar na raiz). Cria a frente se for nova.
/// Devolve o nome da frente com a grafia existente.
pub fn set_front_dir(root: &Path, project: &str, front: &str, dir: Option<&Path>) -> AppResult<String> {
    let project_dir = root.join(clean_name(project)?);
    if !is_project_dir(&project_dir) {
        return Err(AppError::ProjectNotFound(project.to_string()));
    }
    let front = existing_or(&project_dir, clean_name(front)?)?;
    let front_dir = project_dir.join(&front);
    let link = front_dir.join(FRONT_LINK);
    match dir {
        Some(dir) => {
            fs::create_dir_all(&front_dir)?;
            fs::write(link, dir.to_string_lossy().as_bytes())?;
        }
        None if link.exists() => fs::remove_file(link)?,
        None => {}
    }
    Ok(front)
}

/// Caminho gravado em `.recapt-pasta`, se a frente tiver pasta propria.
fn linked_dir(front_dir: &Path) -> Option<PathBuf> {
    let raw = fs::read_to_string(front_dir.join(FRONT_LINK)).ok()?;
    let path = PathBuf::from(raw.trim());
    path.is_absolute().then_some(path)
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
                let place = |theme: Option<String>| Place { project: dir_name(&dir), front: dir_name(&front), theme };
                for child in subdirs(&front)? {
                    if is_session_dir(&child) {
                        // Gravada antes da TASK-10, direto na frente.
                        sessions.push(read(&child, rel_id(root, &child), place(None)));
                    } else {
                        for session in subdirs(&child)?.into_iter().filter(|d| is_session_dir(d)) {
                            sessions.push(read(&session, rel_id(root, &session), place(dir_name(&child))));
                        }
                        for session in pointed(&child) {
                            let id = format!("{}/{}", rel_id(root, &child), dir_name(&session).unwrap_or_default());
                            sessions.push(read(&session, id, place(dir_name(&child))));
                        }
                    }
                }
                // Pasta propria da frente; se estiver indisponivel (disco externo), so nao aparece.
                let Some(linked) = linked_dir(&front).filter(|d| d.is_dir()) else { continue };
                let prefix = rel_id(root, &front);
                for theme in subdirs(&linked)?.into_iter().filter(|d| !is_session_dir(d)) {
                    for session in subdirs(&theme)?.into_iter().filter(|d| is_session_dir(d)) {
                        let id = format!("{prefix}/{}/{}", dir_name(&theme).unwrap_or_default(), dir_name(&session).unwrap_or_default());
                        sessions.push(read(&session, id, place(dir_name(&theme))));
                    }
                }
            }
        } else if is_month_dir(&dir) {
            for session in subdirs(&dir)?.into_iter().filter(|d| is_session_dir(d)) {
                sessions.push(read(&session, rel_id(root, &session), Place::default()));
            }
        }
        // Outras pastas da raiz sao ignoradas.
    }
    // Nomes comecam com a data: ordenar pelo nome deixa as mais recentes primeiro em todos os projetos.
    sessions.sort_by(|a, b| b.name.cmp(&a.name).then_with(|| a.id.cmp(&b.id)));
    Ok(sessions)
}

pub fn get(root: &Path, id: &str) -> AppResult<Session> {
    // Impede ids como "../../etc" de sair da raiz ou da pasta da frente.
    if id.split(['/', '\\']).any(|part| part == "..") {
        return Err(AppError::SessionNotFound(id.to_string()));
    }
    let parts: Vec<&str> = id.split('/').collect();
    let linked = match parts.as_slice() {
        [project, front, theme, folder] => {
            let pointer = fs::read_to_string(pointer_path(root, project, front, theme, folder))
                .ok()
                .map(|raw| PathBuf::from(raw.trim()))
                .filter(|p| p.is_absolute());
            pointer
                .or_else(|| linked_dir(&root.join(project).join(front)).map(|d| d.join(theme).join(folder)))
                .filter(|p| p.is_dir())
        }
        _ => None,
    };
    let path = linked.unwrap_or_else(|| root.join(id));
    if !path.is_dir() {
        return Err(AppError::SessionNotFound(id.to_string()));
    }
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
    Ok(read(&path, id.to_string(), place))
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

/// Id de uma pasta dentro da raiz, com "/" tambem no Windows.
fn rel_id(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

fn read(path: &Path, id: String, place: Place) -> Session {
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
    fn front_with_own_folder_records_lists_and_resets() {
        let root = temp_root("linked");
        let repo = temp_root("linked-repo").join(".ai/docs/reunioes");
        fs::create_dir_all(&repo).unwrap();
        create_project(&root, "ReCapt").unwrap();
        let new = |theme| NewSession { project: "ReCapt", front: "Produto", theme, subject: "Pasta por frente" };

        // Gravada antes de a frente ter pasta propria: continua na raiz e na lista.
        let old = create(&root, &new("Gravação"), at(9, 0)).unwrap();
        assert_eq!(set_front_dir(&root, "ReCapt", "produto", Some(&repo)).unwrap(), "Produto");

        let s = create(&root, &new("Interface"), at(10, 0)).unwrap();
        assert_eq!(s.id, "ReCapt/Produto/Interface/2026-10-21_10h00 Pasta por frente");
        assert_eq!(s.path, repo.join("Interface/2026-10-21_10h00 Pasta por frente"));
        assert_eq!(get(&root, &s.id).unwrap().path, s.path);
        assert_eq!(get(&root, &old.id).unwrap().path, old.path);

        let ids: Vec<_> = list(&root).unwrap().into_iter().map(|s| s.id).collect();
        assert_eq!(ids, [s.id.clone(), old.id.clone()]);
        let front = &projects(&root).unwrap()[0].fronts[0];
        assert_eq!(front.dir.as_deref(), Some(repo.as_path()));
        assert_eq!(front.themes, ["Gravação", "Interface"]);

        // Tema existente na pasta propria reaproveita a grafia.
        let s2 = create(&root, &new("interface"), at(11, 0)).unwrap();
        assert_eq!(s2.theme.as_deref(), Some("Interface"));

        set_front_dir(&root, "ReCapt", "Produto", None).unwrap();
        assert_eq!(projects(&root).unwrap()[0].fronts[0].dir, None);
        assert!(get(&root, &s.id).is_err());
        fs::remove_dir_all(&root).unwrap();
        fs::remove_dir_all(repo.ancestors().nth(3).unwrap()).unwrap();
    }

    #[test]
    fn moves_sessions_between_projects_keeping_files() {
        let root = temp_root("move");
        let linked = temp_root("move-linked");
        create_project(&root, "Cliente X").unwrap();
        create_project(&root, "ReCapt").unwrap();
        set_front_dir(&root, "ReCapt", "Produto", Some(&linked)).unwrap();
        let new = |project, front, theme, subject| NewSession { project, front, theme, subject };

        let s = create(&root, &new("Cliente X", "Discovery", "SAP", ""), at(10, 0)).unwrap();
        fs::write(s.path.join("audio.wav"), b"x").unwrap();
        fs::write(s.path.join("transcricao.md"), "# Transcrição: SAP, 21-10-2026 10h00\n\nProjeto: Cliente X | Frente: Discovery | Tema: SAP\n\nDuração: 00:00:01\n\n**[00:00:00]** oi\n").unwrap();

        // Para a pasta propria de outra frente, com assunto novo.
        let moved = move_to(&root, &s.id, &new("ReCapt", "produto", "Interface", "Revisão"), Location::Structure).unwrap();
        assert_eq!(moved.id, "ReCapt/Produto/Interface/2026-10-21_10h00 Revisão");
        assert_eq!(moved.path, linked.join("Interface/2026-10-21_10h00 Revisão"));
        assert!(moved.path.join("audio.wav").is_file());
        assert!(!s.path.exists());
        // O tema que ficou vazio some.
        assert!(!root.join("Cliente X/Discovery/SAP").exists());
        let md = fs::read_to_string(moved.path.join("transcricao.md")).unwrap();
        assert!(md.starts_with("# Transcrição: Revisão, 21-10-2026 10h00\n\nProjeto: ReCapt | Frente: Produto | Tema: Interface\n\nDuração"));
        assert!(md.ends_with("**[00:00:00]** oi\n"));

        // Mesmo minuto ja ocupado no destino: ganha sufixo; tirar o assunto mantem a data.
        let other = create(&root, &new("ReCapt", "Produto", "Interface", ""), at(10, 0)).unwrap();
        let back = move_to(&root, &moved.id, &new("ReCapt", "Produto", "Interface", ""), Location::Structure).unwrap();
        assert_eq!(other.name, "2026-10-21_10h00");
        assert_eq!(back.name, "2026-10-21_10h00-2");

        // Sem mudanca: nada acontece.
        let same = move_to(&root, &back.id, &new("ReCapt", "Produto", "Interface", ""), Location::Structure).unwrap();
        assert_eq!(same.path, back.path);
        fs::remove_dir_all(&root).unwrap();
        fs::remove_dir_all(&linked).unwrap();
    }

    #[test]
    fn moves_to_picked_folder_inside_or_outside_structure() {
        let root = temp_root("pick");
        let outside = temp_root("pick-outside");
        create_project(&root, "Cliente X").unwrap();
        create_project(&root, "ReCapt").unwrap();
        let new = |project, front, theme| NewSession { project, front, theme, subject: "Kickoff" };
        let s = create(&root, &new("Cliente X", "Discovery", "SAP"), at(10, 0)).unwrap();
        create(&root, &new("ReCapt", "Produto", "Interface"), at(9, 0)).unwrap();

        // Pasta de um tema: projeto, frente e tema saem dela; o assunto fica.
        let inside = move_to_folder(&root, &s.id, &root.join("ReCapt/Produto/Interface")).unwrap();
        assert_eq!(inside.id, "ReCapt/Produto/Interface/2026-10-21_10h00 Kickoff");
        // Pastas da estrutura que nao sao tema sao recusadas.
        assert!(matches!(move_to_folder(&root, &inside.id, &root.join("ReCapt")), Err(AppError::InvalidDestination(_))));
        assert!(matches!(move_to_folder(&root, &inside.id, &inside.path), Err(AppError::InvalidDestination(_))));

        // Fora da estrutura: continua em ReCapt / Produto / Interface no app.
        let out = move_to_folder(&root, &inside.id, &outside).unwrap();
        assert_eq!(out.id, inside.id);
        assert_eq!(out.path, outside.join("2026-10-21_10h00 Kickoff"));
        assert_eq!(get(&root, &out.id).unwrap().path, out.path);
        assert!(list(&root).unwrap().iter().any(|x| x.id == out.id && x.path == out.path));

        // Trocar o assunto renomeia onde ela esta.
        let renamed = move_to(&root, &out.id, &NewSession { subject: "Retro", ..new("ReCapt", "Produto", "Interface") }, Location::Keep).unwrap();
        assert_eq!(renamed.path, outside.join("2026-10-21_10h00 Retro"));
        assert_eq!(get(&root, &renamed.id).unwrap().path, renamed.path);
        assert!(get(&root, &out.id).is_err());

        // Voltar para um tema da estrutura apaga o atalho; excluir tambem.
        let back = move_to_folder(&root, &renamed.id, &root.join("ReCapt/Produto/Interface")).unwrap();
        assert!(back.path.starts_with(&root));
        assert_eq!(list(&root).unwrap().iter().filter(|x| x.id == back.id).count(), 1);
        let gone = move_to_folder(&root, &back.id, &outside).unwrap();
        delete(&root, &gone.id).unwrap();
        assert!(!gone.path.exists());
        assert!(get(&root, &gone.id).is_err());
        assert!(outside.exists());
        fs::remove_dir_all(&root).unwrap();
        fs::remove_dir_all(&outside).unwrap();
    }

    #[test]
    fn completes_legacy_sessions_without_theme_or_project() {
        let root = temp_root("legacy-move");
        create_project(&root, "Loreal").unwrap();
        fs::create_dir_all(root.join("Loreal/Kshu - Willy/2026-10-20_09h00")).unwrap();
        fs::create_dir_all(root.join("2026-09/2026-09-30_09h00")).unwrap();

        let new = |theme, subject| NewSession { project: "Loreal", front: "Kshu - Willy", theme, subject };
        let a = move_to(&root, "Loreal/Kshu - Willy/2026-10-20_09h00", &new("Onboarding", "Kickoff"), Location::Structure).unwrap();
        assert_eq!(a.id, "Loreal/Kshu - Willy/Onboarding/2026-10-20_09h00 Kickoff");
        assert!(!root.join("Loreal/Kshu - Willy/2026-10-20_09h00").exists());
        let b = move_to(&root, "2026-09/2026-09-30_09h00", &new("Onboarding", ""), Location::Structure).unwrap();
        assert_eq!(b.id, "Loreal/Kshu - Willy/Onboarding/2026-09-30_09h00");
        assert_eq!(list(&root).unwrap().len(), 2);
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn deletes_session_folder_and_empty_theme() {
        let root = temp_root("delete");
        create_project(&root, "Cliente X").unwrap();
        let new = |theme| NewSession { project: "Cliente X", front: "Discovery", theme, subject: "" };
        let a = create(&root, &new("SAP"), at(9, 0)).unwrap();
        let b = create(&root, &new("SAP"), at(10, 0)).unwrap();
        fs::write(a.path.join("audio.wav"), b"x").unwrap();

        delete(&root, &a.id).unwrap();
        assert!(!a.path.exists());
        assert!(b.path.exists());
        delete(&root, &b.id).unwrap();
        assert!(!root.join("Cliente X/Discovery/SAP").exists());
        assert!(root.join("Cliente X/Discovery").exists());
        assert!(delete(&root, &a.id).is_err());
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
