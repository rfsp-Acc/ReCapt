use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("ja existe uma gravacao em andamento")]
    AlreadyRecording,
    #[error("nenhuma gravacao em andamento")]
    NotRecording,
    #[error("sessao nao encontrada: {0}")]
    SessionNotFound(String),
    #[error("a sessao nao tem audio gravado")]
    NoAudio,
    #[error("a sessao ja esta sendo transcrita")]
    AlreadyTranscribing,
    #[error("modelo do Whisper nao encontrado em {}", .0.display())]
    NoWhisperModel(std::path::PathBuf),
    #[error(transparent)]
    Transcribe(#[from] crate::whisper::engine::TranscribeError),
    #[error("sem permissao de escrita em {}", .0.display())]
    NotWritable(std::path::PathBuf),
    #[error("nao e possivel trocar a pasta durante uma gravacao")]
    ChangeDirWhileRecording,
    #[error("nome invalido: \"{0}\"")]
    InvalidName(String),
    #[error("o nome pode ter no maximo 80 caracteres")]
    NameTooLong,
    #[error("ja existe um projeto chamado \"{0}\"")]
    ProjectExists(String),
    #[error("a sessao esta sendo gravada ou transcrita; aguarde terminar")]
    SessionBusy,
    #[error("não é possível mover para esta pasta: {0}")]
    InvalidDestination(String),
    #[error("projeto nao encontrado: \"{0}\"")]
    ProjectNotFound(String),
    #[error("pasta pessoal do usuario nao encontrada")]
    NoHomeDir,
    #[error(transparent)]
    Recorder(#[from] crate::audio::recorder::RecorderError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// Tauri exige que o erro de um command seja serializavel; enviamos a mensagem.
impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
