import { formatClock } from "../lib/format";
import { InputDevice, Levels, RecordingStatus } from "../lib/tauri";
import { Icon } from "./Icon";
import { LevelMeter } from "./LevelMeter";

interface Props {
  status: RecordingStatus;
  elapsed: number;
  saving: boolean;
  levels: Levels;
  mics: InputDevice[];
  /** `null` segue o padrao do sistema. */
  selectedMic: string | null;
  onSelectMic: (id: string | null) => void;
  onRefreshMics: () => void;
  onStart: () => void;
  onStop: () => void;
  onPause: () => void;
  onResume: () => void;
}

const DEFAULT_MIC = "__default__";

export function CaptureCard(p: Props) {
  const { recording, paused } = p.status;
  const state = p.saving ? "saving" : !recording ? "idle" : paused ? "paused" : "recording";
  const label = {
    idle: "Pronto para gravar",
    recording: "Gravando",
    paused: "Pausado",
    saving: "Salvando áudio...",
  }[state];
  const micName =
    p.mics.find((d) => d.id === p.selectedMic)?.name ?? p.mics.find((d) => d.isDefault)?.name ?? "Microfone";
  // Ultimo aviso de dispositivo: os anteriores ja foram resolvidos ou substituidos.
  const lastWarning = p.status.warnings[p.status.warnings.length - 1];

  return (
    <section className={`capture-card is-${state}`} aria-label="Gravação">
      <div className="capture-main">
        <button
          className="record-button"
          onClick={recording ? p.onStop : p.onStart}
          disabled={p.saving}
          aria-label={recording ? "Parar gravação" : "Iniciar gravação"}
          title={recording ? "Parar e transcrever" : "Iniciar gravação"}
        >
          <span className="record-icon" />
        </button>

        <div className="capture-info">
          <span className="capture-state" aria-live="polite">
            {state === "recording" && <span className="live-dot" />}
            {label}
          </span>
          <span className="capture-clock">{formatClock(p.elapsed)}</span>
          <span className="capture-hint">
            {state === "idle" && "Grava o microfone e o áudio do computador. A transcrição começa ao parar."}
            {state === "recording" && "Clique no botão vermelho para parar e transcrever."}
            {state === "paused" && "O trecho pausado não entra na gravação."}
            {state === "saving" && "Juntando as faixas de áudio."}
          </span>
        </div>

        {recording && (
          <button className="button" onClick={paused ? p.onResume : p.onPause} disabled={p.saving}>
            <Icon name={paused ? "play" : "pause"} size={16} />
            {paused ? "Retomar" : "Pausar"}
          </button>
        )}
      </div>

      <div className="capture-sources">
        {recording ? (
          <>
            <LevelMeter
              icon="mic"
              label={micName}
              level={p.levels.microphone}
              offline={!p.status.sources.includes("microphone")}
            />
            <LevelMeter
              icon="speaker"
              label="Áudio do computador"
              level={p.levels.system}
              offline={!p.status.sources.includes("system")}
            />
          </>
        ) : (
          <label className="mic-picker">
            <Icon name="mic" size={16} />
            <select
              value={p.selectedMic ?? DEFAULT_MIC}
              onFocus={p.onRefreshMics}
              onChange={(e) => p.onSelectMic(e.target.value === DEFAULT_MIC ? null : e.target.value)}
              aria-label="Microfone"
            >
              <option value={DEFAULT_MIC}>
                Padrão{p.mics.find((d) => d.isDefault) ? ` (${p.mics.find((d) => d.isDefault)!.name})` : " do sistema"}
              </option>
              {p.mics.map((d) => (
                <option key={d.id} value={d.id}>
                  {d.name}
                </option>
              ))}
            </select>
          </label>
        )}
        {recording && lastWarning && <p className="capture-warning">{lastWarning}</p>}
      </div>
    </section>
  );
}
