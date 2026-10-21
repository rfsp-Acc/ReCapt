interface Props {
  recording: boolean;
  paused: boolean;
  saving: boolean;
  elapsed: number;
  onStart: () => void;
  onStop: () => void;
  onPause: () => void;
  onResume: () => void;
}

function formatElapsed(total: number) {
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  const mmss = `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
  return h > 0 ? `${h}:${mmss}` : mmss;
}

export function RecordButton(p: Props) {
  const label = p.saving
    ? "Salvando áudio..."
    : p.recording
      ? `${formatElapsed(p.elapsed)}${p.paused ? " (pausado)" : ""}`
      : "Pronto para gravar";

  return (
    <div className="record">
      <div className="record-controls">
        <button
          className={`record-button ${p.recording ? "is-recording" : ""}`}
          onClick={p.recording ? p.onStop : p.onStart}
          disabled={p.saving}
          aria-label={p.recording ? "Parar gravação" : "Iniciar gravação"}
        >
          <span className="record-icon" />
        </button>
        {p.recording && (
          <button className="pause-button" onClick={p.paused ? p.onResume : p.onPause} disabled={p.saving}>
            {p.paused ? "Retomar" : "Pausar"}
          </button>
        )}
      </div>
      <span className={`record-time ${p.paused ? "is-paused" : ""}`}>{label}</span>
    </div>
  );
}
