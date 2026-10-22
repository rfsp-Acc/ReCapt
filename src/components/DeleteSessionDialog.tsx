import { useEffect, useState } from "react";
import { formatSessionName, shortPath } from "../lib/format";
import { Session } from "../lib/tauri";
import { Icon } from "./Icon";

interface Props {
  /** `null` fecha a janela. */
  session: Session | null;
  onClose: () => void;
  /** Lanca erro se o backend recusar; a mensagem aparece na janela. */
  onDelete: (session: Session) => Promise<void>;
}

/** Confirmacao antes de apagar a pasta da sessao: nao da para desfazer. */
export function DeleteSessionDialog({ session, onClose, onDelete }: Props) {
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    if (!session) return;
    setError(null);
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && onClose();
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [session, onClose]);

  if (!session) return null;

  const confirm = async () => {
    setBusy(true);
    setError(null);
    try {
      await onDelete(session);
      onClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="overlay is-centered" onMouseDown={(e) => e.target === e.currentTarget && onClose()}>
      <div className="dialog" role="alertdialog" aria-modal="true" aria-labelledby="delete-session-title" aria-describedby="delete-session-text">
        <header className="settings-header">
          <h2 id="delete-session-title">Excluir gravação?</h2>
          <button type="button" className="icon-button" onClick={onClose} aria-label="Cancelar">
            <Icon name="close" />
          </button>
        </header>
        <div id="delete-session-text" className="dialog-text">
          <p>
            <strong>{formatSessionName(session.name)}</strong>
          </p>
          <p>
            A pasta será apagada com o áudio, a transcrição e a análise. Não vai para a Lixeira e não dá para desfazer.
          </p>
          <code className="path" title={session.path}>
            {shortPath(session.path)}
          </code>
          {error && <p className="field-hint is-error">{error}</p>}
        </div>
        <div className="dialog-actions">
          <button type="button" className="button" onClick={onClose} autoFocus>
            Cancelar
          </button>
          <button type="button" className="button is-danger" onClick={confirm} disabled={busy}>
            {busy ? "Excluindo..." : "Excluir"}
          </button>
        </div>
      </div>
    </div>
  );
}
