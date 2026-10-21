import { useCallback, useState } from "react";

const KEY = "recapt:last-target";

export interface Target {
  project: string;
  front: string;
}

/** Ultimo projeto e frente usados: a proxima gravacao ja comeca preenchida. */
export function useLastTarget() {
  const [target, setTarget] = useState<Target>(() => {
    try {
      return { project: "", front: "", ...JSON.parse(localStorage.getItem(KEY) ?? "{}") };
    } catch {
      return { project: "", front: "" };
    }
  });

  const update = useCallback((next: Partial<Target>) => {
    setTarget((prev) => {
      const merged = { ...prev, ...next };
      localStorage.setItem(KEY, JSON.stringify(merged));
      return merged;
    });
  }, []);

  return [target, update] as const;
}
