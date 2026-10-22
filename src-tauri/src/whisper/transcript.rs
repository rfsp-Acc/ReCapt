//! Gera o transcricao.md da sessao.

use super::engine::Transcript;
use super::language;

/// Lugar da sessao na hierarquia, quando houver: da contexto para a IA na analise.
pub struct Context<'a> {
    pub project: Option<&'a str>,
    pub front: Option<&'a str>,
    pub theme: Option<&'a str>,
    pub subject: Option<&'a str>,
}

/// `title` ja vem formatado para exibicao (data em DD-MM-AAAA).
pub fn to_markdown(title: &str, context: &Context, transcript: &Transcript) -> String {
    let segments = &transcript.segments;
    let duration = segments.last().map(|s| s.end_ms).unwrap_or(0);
    let origin = if transcript.detected { "detectado automaticamente" } else { "escolhido nas configurações" };
    let mut md = header(title, context);
    md.push_str(&format!(
        "Duração: {} | Idioma: {} ({origin})\n\n",
        timestamp(duration),
        language::display_name(&transcript.language),
    ));

    if segments.is_empty() {
        md.push_str("_Nenhuma fala reconhecida._\n");
        return md;
    }
    for s in segments {
        md.push_str(&format!("**[{}]** {}\n\n", timestamp(s.start_ms), s.text));
    }
    md
}

/// Titulo e linha "Projeto | Frente | Tema".
fn header(title: &str, context: &Context) -> String {
    // O nome mais especifico disponivel: assunto, tema ou frente.
    let heading = match context.subject.or(context.theme).or(context.front) {
        Some(name) => format!("{name}, {title}"),
        None => title.to_string(),
    };
    let mut md = format!("# Transcrição: {heading}\n\n");
    let place: Vec<String> = [("Projeto", context.project), ("Frente", context.front), ("Tema", context.theme)]
        .iter()
        .filter_map(|(label, value)| value.map(|v| format!("{label}: {v}")))
        .collect();
    if !place.is_empty() {
        md.push_str(&format!("{}\n\n", place.join(" | ")));
    }
    md
}

/// Troca o cabecalho de um transcricao.md ja gerado (sessao movida), mantendo as falas.
/// Arquivo sem o cabecalho esperado volta sem mudanca.
pub fn retitle(md: &str, title: &str, context: &Context) -> String {
    if !md.starts_with("# Transcrição:") {
        return md.to_string();
    }
    match md.find("\nDuração:") {
        Some(i) => format!("{}{}", header(title, context), &md[i + 1..]),
        None => md.to_string(),
    }
}

/// Nome da pasta "AAAA-MM-DD_resto" para exibicao "DD-MM-AAAA resto",
/// mesma regra de src/lib/format.ts.
pub fn display_name(folder: &str) -> String {
    let bytes = folder.as_bytes();
    let is_date = folder.len() >= 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && folder[..4].bytes().chain(folder[5..7].bytes()).chain(folder[8..10].bytes()).all(|b| b.is_ascii_digit());
    if !is_date {
        return folder.to_string();
    }
    let date = format!("{}-{}-{}", &folder[8..10], &folder[5..7], &folder[..4]);
    match folder[10..].strip_prefix('_') {
        Some(rest) if !rest.is_empty() => format!("{date} {rest}"),
        _ => date,
    }
}

fn timestamp(ms: i64) -> String {
    let total = ms.max(0) / 1000;
    format!("{:02}:{:02}:{:02}", total / 3600, (total % 3600) / 60, total % 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::whisper::engine::Segment;

    #[test]
    fn display_name_matches_frontend_rule() {
        assert_eq!(display_name("2026-10-20_15h40"), "20-10-2026 15h40");
        assert_eq!(display_name("2026-09-21_alinhamento-produto"), "21-09-2026 alinhamento-produto");
        assert_eq!(display_name("2026-10-20"), "20-10-2026");
        assert_eq!(display_name("reuniao"), "reuniao");
    }

    #[test]
    fn markdown_lists_segments_with_timestamps() {
        let transcript = Transcript {
            segments: vec![
                Segment { start_ms: 0, end_ms: 4_000, text: "Bom dia.".into() },
                Segment { start_ms: 3_725_000, end_ms: 3_730_500, text: "Encerrando.".into() },
            ],
            language: "en".into(),
            detected: true,
        };
        let ctx = Context {
            project: Some("Cliente X"),
            front: Some("Discovery"),
            theme: Some("Integração com SAP"),
            subject: Some("Mapeamento de campos"),
        };
        let md = to_markdown("20-10-2026 15h40", &ctx, &transcript);
        assert!(md.starts_with("# Transcrição: Mapeamento de campos, 20-10-2026 15h40\n"));
        assert!(md.contains("Projeto: Cliente X | Frente: Discovery | Tema: Integração com SAP"));
        assert!(md.contains("Duração: 01:02:10 | Idioma: inglês (detectado automaticamente)"));
        assert!(md.contains("**[00:00:00]** Bom dia."));
        assert!(md.contains("**[01:02:05]** Encerrando."));
    }

    #[test]
    fn empty_transcript_says_so() {
        let empty = Transcript { segments: vec![], language: "pt".into(), detected: false };
        let none = Context { project: None, front: None, theme: None, subject: None };
        let md = to_markdown("x", &none, &empty);
        assert!(md.starts_with("# Transcrição: x\n\nDuração"));
        assert!(md.contains("Nenhuma fala reconhecida"));
        assert!(md.contains("Idioma: português (escolhido nas configurações)"));
    }
}
