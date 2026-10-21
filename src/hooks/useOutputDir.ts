import { useCallback, useEffect, useState } from "react";
import { api, OutputDir } from "../lib/tauri";

export function useOutputDir(onChanged?: () => void) {
  const [dir, setDir] = useState<OutputDir | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api.getOutputDir().then(setDir).catch((e) => setError(String(e)));
  }, []);

  const update = useCallback(
    async (action: () => Promise<OutputDir | null>) => {
      setError(null);
      try {
        const next = await action();
        if (next) {
          setDir(next);
          onChanged?.();
        }
      } catch (e) {
        setError(String(e));
      }
    },
    [onChanged],
  );

  return {
    dir,
    error,
    choose: () => update(api.chooseOutputDir),
    reset: () => update(api.resetOutputDir),
    open: () => api.openOutputDir().catch((e) => setError(String(e))),
  };
}
