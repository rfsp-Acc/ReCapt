import { FormEvent, useEffect, useRef, useState } from "react";
import { formatSessionName } from "../lib/format";
import { Project, RenameTarget, Session } from "../lib/tauri";
import { Icon } from "./Icon";

interface Props {
  /** `null` fecha a janela. */
  session: Session | null;
  projects: Project[];
  onClose: () => void;
  /** Lanca erro se o backend recusar; a mensagem aparece na janela. */
  onRename: (session: Session, target: RenameTarget) => Promise<void>;
}

const EMPTY = { project: "", front: "", theme: "", subject: "" };
const sameName = (a: string, b: string) => a.trim().toLowerCase() === b.trim().toLowerCase();

/** Troca o assunto da reuniao; a pasta e renomeada onde estiver.
 * Reunioes antigas pedem tambem o projeto, a frente ou o tema que faltam. */
export function RenameSessionDialog({ session, projects, onClose, onRename }: Props) {
  const [form, setForm] = useState(EMPTY);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const first = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!session) return;
    setForm({ ...EMPTY, subject: session.subject ?? "" });
    setError(null);
    setTimeout(() => first.current?.focus(), 0);
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && onClose();
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [session, onClose]);

  if (!session) return null;

  const needsProject = session.project === null;
  const needsFront = session.front === null;
  const needsTheme = session.theme === null;
  const incomplete = needsProject || needsFront || needsTheme;
  const projectName = session.project ?? form.project;
  const project = projects.find((p) => p.name === projectName);
  const frontName = session.front ?? form.front;
  const front = project?.fronts.find((f) => sameName(f.name, frontName));
  const ready = (!needsProject || project !== undefined) && (!needsFront || form.front.trim() !== "") && (!needsTheme || form.theme.trim() !== "");
  const unchanged = !incomplete && form.subject.trim() === (session.subject ?? "");
  const set = (next: Partial<typeof form>) => setForm((f) => ({ ...f, ...next }));

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    if (!ready || unchanged) return;
    setBusy(true);
    setError(null);
    try {
      await onRename(session, {
        project: needsProject ? form.project : null,
        front: needsFront ? form.front : null,
        theme: needsTheme ? form.theme : null,
        subject: form.subject,
      });
      onClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const date = formatSessionName(session.name).split(" ")[0];

  return (
    <div className="overlay is-centered" onMouseDown={(e) => e.target === e.currentTarget && onClose()}>
      <form className="dialog" role="dialog" aria-modal="true" aria-labelledby="rename-session-title" onSubmit={submit}>
        <header className="settings-header">
          <h2 id="rename-session-title">Renomear reunião</h2>
          <button type="button" className="icon-button" onClick={onClose} aria-label="Cancelar">
            <Icon name="close" />
          </button>
        </header>

        {incomplete && (
          <p className="dialog-text">
            Esta reunião de {date} foi gravada antes de {needsProject ? "existirem projetos" : "existir o nível Tema"}. Complete
            as informações que faltam: ela vai para a pasta do tema.
          </p>
        )}

        {needsProject && (
          <label className="field">
            <span className="field-label">Projeto</span>
            <select value={project ? project.name : ""} onChange={(e) => set({ project: e.target.value })} aria-label="Projeto">
              <option value="" disabled>
                {projects.length === 0 ? "Nenhum projeto" : "Escolha um projeto"}
              </option>
              {projects.map((p) => (
                <option key={p.name} value={p.name}>
                  {p.name}
                </option>
              ))}
            </select>
          </label>
        )}
        {needsFront && (
          <label className="field">
            <span className="field-label">Frente</span>
            <input
              className="text-input"
              list="rename-fronts"
              value={form.front}
              onChange={(e) => set({ front: e.target.value })}
              placeholder={project?.fronts[0] ? `Ex.: ${project.fronts[0].name}` : "Ex.: Discovery"}
              maxLength={80}
              disabled={!project}
            />
            <datalist id="rename-fronts">
              {project?.fronts.map((f) => <option key={f.name} value={f.name} />)}
            </datalist>
          </label>
        )}
        {needsTheme && (
          <label className="field">
            <span className="field-label">Tema</span>
            <input
              ref={needsProject ? undefined : first}
              className="text-input"
              list="rename-themes"
              value={form.theme}
              onChange={(e) => set({ theme: e.target.value })}
              placeholder={front?.themes[0] ? `Ex.: ${front.themes[0]}` : "Ex.: Integração com SAP"}
              maxLength={80}
              disabled={!project}
            />
            <datalist id="rename-themes">
              {front?.themes.map((t) => <option key={t} value={t} />)}
            </datalist>
          </label>
        )}
        <label className="field">
          <span className="field-label">
            Assunto <span className="field-optional">(opcional)</span>
          </span>
          <input
            ref={incomplete ? undefined : first}
            className="text-input"
            value={form.subject}
            onChange={(e) => set({ subject: e.target.value })}
            placeholder="Ex.: Mapeamento de campos"
            maxLength={80}
            aria-invalid={error !== null}
          />
          <span className={`field-hint ${error ? "is-error" : ""}`}>
            {error ?? `Reunião de ${date}. A data e a hora continuam no nome da pasta.`}
          </span>
        </label>
        <div className="dialog-actions">
          <button type="button" className="button" onClick={onClose}>
            Cancelar
          </button>
          <button type="submit" className="button is-primary" disabled={!ready || unchanged || busy}>
            {incomplete ? "Salvar" : "Renomear"}
          </button>
        </div>
      </form>
    </div>
  );
}
