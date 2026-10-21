import { useCallback, useEffect, useState } from "react";
import { api } from "../lib/tauri";

/** Progresso (0 a 100) das sessoes sendo transcritas, por id. */
export function useTranscription(onDone?: () => void) {
  const [progress, setProgress] = useState<Record<string, number>>({});
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const unlisten = api.onTranscriptionProgress(({ sessionId, percent }) =>
      setProgress((p) => (sessionId in p ? { ...p, [sessionId]: percent } : p)),
    );
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const transcribe = useCallback(
    async (id: string) => {
      setError(null);
      setProgress((p) => ({ ...p, [id]: 0 }));
      try {
        await api.transcribe(id);
      } catch (e) {
        setError(String(e));
      } finally {
        setProgress(({ [id]: _, ...rest }) => rest);
        onDone?.();
      }
    },
    [onDone],
  );

  return { progress, error, transcribe };
}
