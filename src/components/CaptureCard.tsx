import { formatClock } from "../lib/format";
import { Target } from "../hooks/useLastTarget";
import { InputDevice, Levels, Project, RecordingStatus } from "../lib/tauri";
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
  projects: Project[];
  target: Target;
  onTarget: (t: Partial<Target>) => void;
  onAddProject: () => void;
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
  const project = p.projects.find((x) => x.name === p.target.project);
  const ready = project !== undefined && p.target.front.trim() !== "";
  // Ultimo aviso de dispositivo: os anteriores ja foram resolvidos ou substituidos.
  const lastWarning = p.status.warnings[p.status.warnings.length - 1];

  return (
    <section className={`capture-card is-${state}`} aria-label="Gravação">
      <div className="capture-main">
        <button
          className="record-button"
          onClick={recording ? p.onStop : p.onStart}
          disabled={p.saving || (!recording && !ready)}
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
            {state === "idle" &&
              (ready
                ? "Grava o microfone e o áudio do computador. A transcrição começa ao parar."
                : p.projects.length === 0
                  ? "Crie um projeto para começar a gravar."
                  : "Escolha o projeto e informe a frente ou o tema da reunião.")}
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

      {!recording && (
        <div className="capture-target">
          <label className="field">
            <span className="field-label">Projeto</span>
            <div className="field-row">
              <select
                value={project ? project.name : ""}
                onChange={(e) => p.onTarget({ project: e.target.value })}
                disabled={p.saving}
                aria-label="Projeto"
              >
                <option value="" disabled>
                  {p.projects.length === 0 ? "Nenhum projeto" : "Escolha um projeto"}
                </option>
                {p.projects.map((x) => (
                  <option key={x.name} value={x.name}>
                    {x.name}
                  </option>
                ))}
              </select>
              <button className="icon-button" onClick={p.onAddProject} aria-label="Adicionar projeto" title="Adicionar projeto">
                <Icon name="plus" size={16} />
              </button>
            </div>
          </label>
          <label className="field">
            <span className="field-label">Frente ou tema da reunião</span>
            <input
              className="text-input"
              list="fronts"
              value={p.target.front}
              onChange={(e) => p.onTarget({ front: e.target.value })}
              placeholder={project?.fronts[0] ? `Ex.: ${project.fronts[0]}` : "Ex.: Discovery, Status semanal"}
              maxLength={80}
              disabled={!project || p.saving}
            />
            {/* Sugere as frentes que ja existem no projeto. */}
            <datalist id="fronts">
              {project?.fronts.map((f) => <option key={f} value={f} />)}
            </datalist>
          </label>
        </div>
      )}

      {recording && p.status.sessionId && (
        <p className="capture-path">{p.status.sessionId.split("/").slice(0, -1).join(" / ")}</p>
      )}

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
