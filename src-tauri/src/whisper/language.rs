use serde::{Deserialize, Serialize};

/// Idioma pedido ao Whisper. Ver TASK-06.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// Detecta nos primeiros 30 s e mantem ate o fim.
    #[default]
    Auto,
    Pt,
    En,
}

impl Language {
    pub fn whisper_code(self) -> &'static str {
        match self {
            Language::Auto => "auto",
            Language::Pt => "pt",
            Language::En => "en",
        }
    }
}

/// Nome em portugues para o cabecalho da transcricao; codigos sem traducao aparecem como estao.
pub fn display_name(code: &str) -> String {
    match code {
        "pt" => "português",
        "en" => "inglês",
        "es" => "espanhol",
        "fr" => "francês",
        "de" => "alemão",
        "it" => "italiano",
        "ja" => "japonês",
        "zh" => "chinês",
        other => return other.to_string(),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_as_lowercase_codes() {
        assert_eq!(serde_json::to_string(&Language::En).unwrap(), "\"en\"");
        assert_eq!(serde_json::from_str::<Language>("\"auto\"").unwrap(), Language::Auto);
    }

    #[test]
    fn display_names() {
        assert_eq!(display_name("en"), "inglês");
        assert_eq!(display_name("nl"), "nl");
    }
}
