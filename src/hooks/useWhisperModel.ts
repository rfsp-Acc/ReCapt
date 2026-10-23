import { useCallback, useEffect, useState } from "react";
import { api, ModelDownloadProgress, ModelStatus } from "../lib/tauri";

/** Modelo do Whisper: sem ele nao ha transcricao, entao a primeira execucao oferece o download. */
export function useWhisperModel(onInstalled?: () => void) {
  const [status, setStatus] = useState<ModelStatus | null>(null);
  const [progress, setProgress] = useState<ModelDownloadProgress | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api.modelStatus().then(setStatus).catch((e) => setError(String(e)));
    const unlisten = api.onModelDownloadProgress(setProgress);
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const download = useCallback(async () => {
    setError(null);
    setProgress({ downloaded: 0, total: null });
    try {
      setStatus(await api.downloadModel());
      onInstalled?.();
    } catch (e) {
      setError(String(e));
    } finally {
      setProgress(null);
    }
  }, [onInstalled]);

  return { status, progress, error, download };
}
