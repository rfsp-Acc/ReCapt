import { useCallback, useEffect, useState } from "react";
import { api, RecordingStatus } from "../lib/tauri";

const idle: RecordingStatus = {
  recording: false,
  paused: false,
  sessionId: null,
  elapsedMs: 0,
  screen: false,
  sources: [],
  warnings: [],
};

export function useRecording(onStopped?: (sessionId: string) => void) {
  const [status, setStatus] = useState<RecordingStatus>(idle);
  const [elapsed, setElapsed] = useState(0);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Recupera o estado se a janela for recarregada durante uma gravacao.
  useEffect(() => {
    api.recordingStatus().then(setStatus).catch((e) => setError(String(e)));
  }, []);

  // Durante a gravacao, busca o estado periodicamente: trocas de dispositivo geram avisos.
  useEffect(() => {
    if (!status.recording) return;
    const id = setInterval(() => {
      api.recordingStatus().then((next) => {
        setStatus((prev) =>
          prev.warnings.length !== next.warnings.length || prev.sources.length !== next.sources.length
            ? next
            : prev,
        );
      });
    }, 2000);
    return () => clearInterval(id);
  }, [status.recording]);

  // O backend informa o tempo gravado; a interface so continua contando enquanto nao esta pausada.
  useEffect(() => {
    const base = Math.floor(status.elapsedMs / 1000);
    setElapsed(base);
    if (!status.recording || status.paused) return;
    const t0 = Date.now() - (status.elapsedMs % 1000);
    const id = setInterval(() => setElapsed(base + Math.floor((Date.now() - t0) / 1000)), 250);
    return () => clearInterval(id);
  }, [status]);

  const run = useCallback(async (action: () => Promise<void>) => {
    setError(null);
    try {
      await action();
    } catch (e) {
      setError(String(e));
    }
  }, []);

  const start = (project: string, front: string) =>
    run(async () => setStatus(await api.startRecording(project, front)));
  const pause = () => run(async () => setStatus(await api.pauseRecording()));
  const resume = () => run(async () => setStatus(await api.resumeRecording()));

  const stop = () =>
    run(async () => {
      setSaving(true);
      try {
        const sessionId = await api.stopRecording();
        setStatus(idle);
        onStopped?.(sessionId);
      } finally {
        setSaving(false);
      }
    });

  return { status, elapsed, saving, error, start, pause, resume, stop };
}
