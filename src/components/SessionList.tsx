import { formatSessionName } from "../lib/format";
import { api, Session } from "../lib/tauri";

interface Props {
  sessions: Session[];
  /** Progresso das transcricoes em andamento, por id da sessao. */
  transcribing: Record<string, number>;
  onTranscribe: (id: string) => void;
}

export function SessionList({ sessions, transcribing, onTranscribe }: Props) {
  if (sessions.length === 0) {
    return <p className="empty">Nenhuma sessão gravada ainda.</p>;
  }

  return (
    <ul className="session-list">
      {sessions.map((s) => {
        const percent = transcribing[s.id];
        return (
          <li key={s.id}>
            <span className="session-name">{formatSessionName(s.name)}</span>
            <span className="session-tags">
              {s.hasAudio && <span>áudio</span>}
              {s.hasVideo && <span>vídeo</span>}
              {s.hasTranscript && <span>transcrição</span>}
              {s.hasAnalysis && <span>análise</span>}
            </span>
            {percent !== undefined ? (
              <span className="progress" title="Transcrevendo">
                <span className="progress-bar" style={{ width: `${percent}%` }} />
                <span className="progress-label">Transcrevendo {percent}%</span>
              </span>
            ) : (
              s.hasAudio && (
                <button
                  className="link"
                  onClick={() => onTranscribe(s.id)}
                  title={s.hasTranscript ? "Substitui o transcricao.md atual" : undefined}
                >
                  {s.hasTranscript ? "Transcrever novamente" : "Transcrever"}
                </button>
              )
            )}
            <button className="link" onClick={() => api.openSessionFolder(s.id)}>
              Abrir pasta
            </button>
          </li>
        );
      })}
    </ul>
  );
}
