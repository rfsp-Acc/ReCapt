import { OutputDir } from "../lib/tauri";

interface Props {
  dir: OutputDir | null;
  /** Trocar a pasta durante uma gravacao separaria os arquivos da sessao. */
  disabled: boolean;
  onChoose: () => void;
  onReset: () => void;
  onOpen: () => void;
}

export function OutputDirSetting({ dir, disabled, onChoose, onReset, onOpen }: Props) {
  return (
    <div className="output-dir">
      <div className="output-dir-path">
        <span className="output-dir-label">Gravações e transcrições salvas em</span>
        <code title={dir?.path}>{dir?.path ?? "..."}</code>
      </div>
      <div className="output-dir-actions">
        <button className="link" onClick={onChoose} disabled={disabled}>
          Alterar pasta
        </button>
        {dir && !dir.isDefault && (
          <button className="link" onClick={onReset} disabled={disabled}>
            Usar padrão
          </button>
        )}
        <button className="link" onClick={onOpen}>
          Abrir
        </button>
      </div>
    </div>
  );
}
