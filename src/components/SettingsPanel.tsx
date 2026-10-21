import { useEffect } from "react";
import { InputDevice, OutputDir, TranscriptionLanguage } from "../lib/tauri";
import { Icon } from "./Icon";

interface Props {
  open: boolean;
  onClose: () => void;
  recording: boolean;
  outputDir: OutputDir | null;
  onChooseDir: () => void;
  onResetDir: () => void;
  onOpenDir: () => void;
  mics: InputDevice[];
  selectedMic: string | null;
  onSelectMic: (id: string | null) => void;
  onRefreshMics: () => void;
  language: TranscriptionLanguage;
  onLanguage: (language: TranscriptionLanguage) => void;
}

const DEFAULT_MIC = "__default__";
const LANGUAGES: { value: TranscriptionLanguage; label: string }[] = [
  { value: "auto", label: "Automático" },
  { value: "pt", label: "Português" },
  { value: "en", label: "Inglês" },
];

export function SettingsPanel(p: Props) {
  useEffect(() => {
    if (!p.open) return;
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && p.onClose();
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [p.open, p.onClose]);

  if (!p.open) return null;
  const missingMic = p.selectedMic !== null && !p.mics.some((d) => d.id === p.selectedMic);
  const defaultMic = p.mics.find((d) => d.isDefault);

  return (
    <div className="overlay" onMouseDown={(e) => e.target === e.currentTarget && p.onClose()}>
      <aside className="settings" role="dialog" aria-modal="true" aria-labelledby="settings-title">
        <header className="settings-header">
          <h2 id="settings-title">Configurações</h2>
          <button className="icon-button" onClick={p.onClose} aria-label="Fechar configurações">
            <Icon name="close" />
          </button>
        </header>

        <div className="field">
          <span className="field-label">Pasta das gravações e transcrições</span>
          <code className="path" title={p.outputDir?.path}>{p.outputDir?.path ?? "..."}</code>
          <div className="field-actions">
            <button className="button" onClick={p.onChooseDir} disabled={p.recording}>
              Alterar pasta
            </button>
            {p.outputDir && !p.outputDir.isDefault && (
              <button className="button is-quiet" onClick={p.onResetDir} disabled={p.recording}>
                Usar padrão
              </button>
            )}
            <button className="button is-quiet" onClick={p.onOpenDir}>
              <Icon name="folder" size={16} />
              Abrir
            </button>
          </div>
          <span className="field-hint">
            {p.recording
              ? "Não é possível trocar a pasta durante uma gravação."
              : "Sessões antigas não são movidas ao trocar a pasta."}
          </span>
        </div>

        <div className="field">
          <label className="field-label" htmlFor="mic-select">Microfone</label>
          <div className="field-row">
            <select
              id="mic-select"
              value={p.selectedMic ?? DEFAULT_MIC}
              onChange={(e) => p.onSelectMic(e.target.value === DEFAULT_MIC ? null : e.target.value)}
            >
              <option value={DEFAULT_MIC}>Padrão{defaultMic ? ` (${defaultMic.name})` : " do sistema"}</option>
              {p.mics.map((d) => (
                <option key={d.id} value={d.id}>
                  {d.name}
                </option>
              ))}
              {missingMic && <option value={p.selectedMic!}>Dispositivo desconectado</option>}
            </select>
            <button className="icon-button" onClick={p.onRefreshMics} aria-label="Atualizar lista de microfones" title="Atualizar lista">
              <Icon name="refresh" size={16} />
            </button>
          </div>
          <span className="field-hint">
            {missingMic
              ? "O microfone escolhido não está conectado; o padrão do sistema será usado."
              : "Com fone Bluetooth, o microfone do computador mantém o áudio do fone em alta qualidade."}
          </span>
        </div>

        <div className="field">
          <span className="field-label" id="language-label">Idioma das reuniões</span>
          <div className="segmented" role="radiogroup" aria-labelledby="language-label">
            {LANGUAGES.map((l) => (
              <button
                key={l.value}
                role="radio"
                aria-checked={p.language === l.value}
                className={p.language === l.value ? "is-selected" : ""}
                onClick={() => p.onLanguage(l.value)}
              >
                {l.label}
              </button>
            ))}
          </div>
          <span className="field-hint">
            {p.language === "auto"
              ? "Detectado nos primeiros 30 s. Em reuniões mistas, escolha o idioma predominante."
              : "Vale para as próximas transcrições. Use \"Transcrever novamente\" para refazer uma sessão."}
          </span>
        </div>
      </aside>
    </div>
  );
}
