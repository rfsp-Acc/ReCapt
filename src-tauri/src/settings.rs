//! Configuracoes do usuario, salvas em <app_config>/settings.json.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
    /// Pasta das sessoes; `None` usa ~/ReCapt.
    output_dir: Option<PathBuf>,
}

pub struct SettingsState {
    file: PathBuf,
    settings: Mutex<Settings>,
}

impl SettingsState {
    /// Arquivo ausente ou invalido volta para o padrao, sem impedir o app de abrir.
    pub fn load(config_dir: &Path) -> Self {
        let file = config_dir.join("settings.json");
        let settings = fs::read_to_string(&file)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Self { file, settings: Mutex::new(settings) }
    }

    pub fn output_dir(&self) -> AppResult<PathBuf> {
        match &self.settings.lock().unwrap().output_dir {
            Some(dir) => Ok(dir.clone()),
            None => default_output_dir(),
        }
    }

    pub fn is_default_output_dir(&self) -> bool {
        self.settings.lock().unwrap().output_dir.is_none()
    }

    /// `None` volta para a pasta padrao.
    pub fn set_output_dir(&self, dir: Option<PathBuf>) -> AppResult<()> {
        if let Some(dir) = &dir {
            ensure_writable(dir)?;
        }
        let mut settings = self.settings.lock().unwrap();
        settings.output_dir = dir;
        if let Some(parent) = self.file.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&*settings).map_err(std::io::Error::other)?;
        fs::write(&self.file, json)?;
        Ok(())
    }
}

pub fn default_output_dir() -> AppResult<PathBuf> {
    Ok(dirs::home_dir().ok_or(AppError::NoHomeDir)?.join("ReCapt"))
}

/// Falha cedo, na escolha da pasta, em vez de no fim de uma gravacao.
fn ensure_writable(dir: &Path) -> AppResult<()> {
    let probe = dir.join(".recapt-teste-escrita");
    fs::write(&probe, b"")
        .and_then(|_| fs::remove_file(&probe))
        .map_err(|_| AppError::NotWritable(dir.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("recapt-settings-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn persists_and_resets_output_dir() {
        let config = temp("config");
        let out = temp("out");

        let state = SettingsState::load(&config);
        assert!(state.is_default_output_dir());

        state.set_output_dir(Some(out.clone())).unwrap();
        let reloaded = SettingsState::load(&config);
        assert_eq!(reloaded.output_dir().unwrap(), out);

        reloaded.set_output_dir(None).unwrap();
        assert!(SettingsState::load(&config).is_default_output_dir());

        fs::remove_dir_all(&config).unwrap();
        fs::remove_dir_all(&out).unwrap();
    }

    #[test]
    fn rejects_missing_dir_and_keeps_previous() {
        let config = temp("config2");
        let state = SettingsState::load(&config);
        assert!(state.set_output_dir(Some(config.join("nao-existe"))).is_err());
        assert!(state.is_default_output_dir());
        fs::remove_dir_all(&config).unwrap();
    }

    #[test]
    fn invalid_file_falls_back_to_default() {
        let config = temp("config3");
        fs::write(config.join("settings.json"), "{ invalido").unwrap();
        assert!(SettingsState::load(&config).is_default_output_dir());
        fs::remove_dir_all(&config).unwrap();
    }
}
