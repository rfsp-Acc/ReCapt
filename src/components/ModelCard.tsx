import { ModelDownloadProgress, ModelStatus } from "../lib/tauri";

interface Props {
  status: ModelStatus;
  progress: ModelDownloadProgress | null;
  onDownload: () => void;
}

function formatSize(mb: number): string {
  return mb >= 1000 ? `${(mb / 1000).toLocaleString("pt-BR", { maximumFractionDigits: 1 })} GB` : `${mb} MB`;
}

/** Aparece enquanto nao ha modelo do Whisper: a gravacao funciona, a transcricao nao. */
export function ModelCard({ status, progress, onDownload }: Props) {
  const percent = progress?.total ? Math.floor((progress.downloaded / progress.total) * 100) : 0;
  return (
    <section className="model-card" aria-labelledby="model-title">
      <div className="model-text">
        <h2 id="model-title">Baixe o modelo de transcrição</h2>
        <p>
          O ReCapt transcreve tudo no seu computador, sem enviar áudio para a internet. Para isso, precisa baixar uma vez
          o modelo <strong>{status.recommended}</strong> ({formatSize(status.recommendedSizeMb)}).
        </p>
      </div>
      {progress ? (
        <span className="progress" role="progressbar" aria-valuenow={percent} aria-valuemin={0} aria-valuemax={100}>
          <span className="progress-bar" style={{ width: `${percent}%` }} />
          <span className="progress-label">
            {progress.total ? `Baixando ${percent}%` : `Baixando ${formatSize(Math.round(progress.downloaded / 1e6))}`}
          </span>
        </span>
      ) : (
        <button className="button is-primary" onClick={onDownload}>
          Baixar modelo
        </button>
      )}
    </section>
  );
}
