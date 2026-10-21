import { useCallback, useState } from "react";

const KEY = "recapt:last-target";

export interface Target {
  project: string;
  front: string;
  theme: string;
}

const EMPTY: Target = { project: "", front: "", theme: "" };

/** Ultimo projeto, frente e tema usados: a proxima gravacao ja comeca preenchida.
 * O assunto nao entra: muda a cada reuniao. */
export function useLastTarget() {
  const [target, setTarget] = useState<Target>(() => {
    try {
      return { ...EMPTY, ...JSON.parse(localStorage.getItem(KEY) ?? "{}") };
    } catch {
      return EMPTY;
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
