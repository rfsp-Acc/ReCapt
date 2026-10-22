import { useEffect, useRef, useState } from "react";
import { dayLabel, formatDuration, parseSessionName } from "../lib/format";
import { api, Session } from "../lib/tauri";
import { Icon } from "./Icon";

interface Props {
  sessions: Session[];
  /** Progresso das transcricoes em andamento, por id da sessao. */
  transcribing: Record<string, number>;
  onTranscribe: (id: string) => void;
  onCopied: (text: string) => void;
  onError: (text: string) => void;
  onMove: (session: Session) => void;
  onRename: (session: Session) => void;
  onDelete: (session: Session) => void;
  /** Quanto do caminho mostrar em cada sessao; o que ja esta no titulo da tela fica de fora. */
  context: "full" | "front" | "theme" | "none";
  emptyText: string;
}

export function SessionList({ sessions, transcribing, onTranscribe, onCopied, onError, onMove, onRename, onDelete, context, emptyText }: Props) {
  if (sessions.length === 0) {
    return (
      <div className="empty">
        <p className="empty-title">Nenhuma sessão aqui</p>
        <p>{emptyText}</p>
      </div>
    );
  }

  // A lista ja vem da mais recente para a mais antiga; agrupa mantendo a ordem.
  const groups: { label: string; items: Session[] }[] = [];
  for (const s of sessions) {
    const label = dayLabel(parseSessionName(s.name).date);
    const group = groups[groups.length - 1];
    if (group?.label === label) group.items.push(s);
    else groups.push({ label, items: [s] });
  }

  return (
    <div className="session-groups">
      {groups.map((g) => (
        <section key={g.label} className="session-group">
          <h3>{g.label}</h3>
          <ul className="session-list">
            {g.items.map((s) => (
              <SessionRow
                key={s.id}
                session={s}
                percent={transcribing[s.id]}
                onTranscribe={onTranscribe}
                onCopied={onCopied}
                onError={onError}
                onMove={onMove}
                onRename={onRename}
                onDelete={onDelete}
                context={context}
              />
            ))}
          </ul>
        </section>
      ))}
    </div>
  );
}

interface RowProps {
  session: Session;
  percent: number | undefined;
  onTranscribe: (id: string) => void;
  onCopied: (text: string) => void;
  onError: (text: string) => void;
  onMove: (session: Session) => void;
  onRename: (session: Session) => void;
  onDelete: (session: Session) => void;
  context: Props["context"];
}

function SessionRow({ session: s, percent, onTranscribe, onCopied, onError, onMove, onRename, onDelete, context }: RowProps) {
  const { time } = parseSessionName(s.name);
  // Sem assunto, o titulo e o nivel mais especifico (tema, frente ou projeto), que sai do caminho
  // para nao aparecer duas vezes.
  const levels = [s.project, s.front, s.theme];
  const titleLevel = s.subject ? -1 : levels.reduce((last, x, i) => (x !== null ? i : last), -1);
  const title = s.subject ?? levels[titleLevel] ?? null;
  const shown = levels.map((x, i) => (i === titleLevel ? null : x));
  const place = { full: shown, front: shown.slice(1), theme: shown.slice(2), none: [] }[context].filter(Boolean);
  const transcribing = percent !== undefined;
  const fail = (e: unknown) => onError(String(e));

  const copyPath = async () => {
    try {
      await api.copyTranscriptPath(s.id);
      onCopied("Caminho da transcrição copiado. Cole na sua IA junto com o prompt de análise.");
    } catch {
      onError(`Não foi possível copiar. Caminho: ${s.path}/transcricao.md`);
    }
  };

  return (
    <li className="session-row">
      <div className="session-text">
        <span className="session-title">
          <span className="session-time">{time || s.name}</span>
          {title ? <span>{title}</span> : <span className="session-no-subject">sem assunto</span>}
          {place.length > 0 && <span className="session-context">{place.join(" / ")}</span>}
        </span>
        <span className="session-meta">
          {s.durationMs !== null ? formatDuration(s.durationMs) : "sem áudio"}
          {s.hasTranscript && <span className="badge is-done">Transcrita</span>}
          {s.hasAnalysis && <span className="badge">Analisada</span>}
          {s.hasAudio && !s.hasTranscript && !transcribing && <span className="badge is-pending">Não transcrita</span>}
        </span>
      </div>

      <div className="session-actions">
        {transcribing ? (
          <span className="progress" role="progressbar" aria-valuenow={percent} aria-valuemin={0} aria-valuemax={100}>
            <span className="progress-bar" style={{ width: `${percent}%` }} />
            <span className="progress-label">Transcrevendo {percent}%</span>
          </span>
        ) : s.hasTranscript ? (
          <>
            <button className="button" onClick={() => api.openSessionFile(s.id, "transcricao.md").catch(fail)}>
              <Icon name="file" size={16} />
              Abrir transcrição
            </button>
            <button className="icon-button" onClick={copyPath} aria-label="Copiar caminho da transcrição" title="Copiar caminho da transcrição">
              <Icon name="copy" size={16} />
            </button>
          </>
        ) : (
          s.hasAudio && (
            <button className="button is-primary" onClick={() => onTranscribe(s.id)}>
              Transcrever
            </button>
          )
        )}
        <RowMenu
          items={[
            { label: "Abrir pasta", onSelect: () => api.openSessionFolder(s.id).catch(fail) },
            ...(!transcribing ? [{ label: "Mover para outra pasta...", onSelect: () => onMove(s) }] : []),
            ...(!transcribing ? [{ label: "Renomear...", onSelect: () => onRename(s) }] : []),
            ...(!transcribing ? [{ label: "Excluir...", onSelect: () => onDelete(s), danger: true }] : []),
            ...(s.hasTranscript && !transcribing
              ? [{ label: "Transcrever novamente", onSelect: () => onTranscribe(s.id) }]
              : []),
          ]}
        />
      </div>
    </li>
  );
}

interface MenuItem {
  label: string;
  onSelect: () => void;
  /** Acao destrutiva: vermelha e separada das outras. */
  danger?: boolean;
}

function RowMenu({ items }: { items: MenuItem[] }) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  // Fecha ao clicar fora ou apertar Esc.
  useEffect(() => {
    if (!open) return;
    const onClick = (e: MouseEvent) => ref.current?.contains(e.target as Node) || setOpen(false);
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && setOpen(false);
    document.addEventListener("mousedown", onClick);
    document.addEventListener("keydown", onKey);
    return () => {
      document.removeEventListener("mousedown", onClick);
      document.removeEventListener("keydown", onKey);
    };
  }, [open]);

  return (
    <div className="menu" ref={ref}>
      <button className="icon-button" onClick={() => setOpen((o) => !o)} aria-label="Mais ações" aria-expanded={open}>
        <Icon name="more" size={16} />
      </button>
      {open && (
        <div className="menu-popover" role="menu">
          {items.map((item, i) => (
            <div key={item.label}>
              {item.danger && i > 0 && <div className="menu-separator" role="separator" />}
              <button
                role="menuitem"
                className={item.danger ? "is-danger" : undefined}
                onClick={() => {
                  setOpen(false);
                  item.onSelect();
                }}
              >
                {item.label}
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
