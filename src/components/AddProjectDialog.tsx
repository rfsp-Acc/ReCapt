import { FormEvent, useEffect, useRef, useState } from "react";
import { Icon } from "./Icon";

interface Props {
  open: boolean;
  onClose: () => void;
  /** Lanca erro se o backend recusar o nome; a mensagem aparece na janela. */
  onCreate: (name: string) => Promise<void>;
}

export function AddProjectDialog({ open, onClose, onCreate }: Props) {
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const input = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!open) return;
    setName("");
    setError(null);
    setTimeout(() => input.current?.focus(), 0);
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && onClose();
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open, onClose]);

  if (!open) return null;

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    setBusy(true);
    setError(null);
    try {
      await onCreate(name);
      onClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="overlay is-centered" onMouseDown={(e) => e.target === e.currentTarget && onClose()}>
      <form className="dialog" role="dialog" aria-modal="true" aria-labelledby="add-project-title" onSubmit={submit}>
        <header className="settings-header">
          <h2 id="add-project-title">Novo projeto</h2>
          <button type="button" className="icon-button" onClick={onClose} aria-label="Cancelar">
            <Icon name="close" />
          </button>
        </header>
        <label className="field">
          <span className="field-label">Nome do projeto</span>
          <input
            ref={input}
            className="text-input"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Ex.: Cliente X, Migração do CRM"
            maxLength={80}
            aria-invalid={error !== null}
          />
          <span className={`field-hint ${error ? "is-error" : ""}`}>
            {error ?? "Vira uma pasta na sua pasta de salvamento. As frentes ficam dentro dela."}
          </span>
        </label>
        <div className="dialog-actions">
          <button type="button" className="button" onClick={onClose}>
            Cancelar
          </button>
          <button type="submit" className="button is-primary" disabled={!name.trim() || busy}>
            Criar projeto
          </button>
        </div>
      </form>
    </div>
  );
}
