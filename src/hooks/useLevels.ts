import { useEffect, useState } from "react";
import { api, Levels } from "../lib/tauri";

const silent: Levels = { microphone: 0, system: 0 };

/** Niveis para os medidores: consulta o backend ~10 vezes por segundo enquanto grava. */
export function useLevels(active: boolean) {
  const [levels, setLevels] = useState<Levels>(silent);

  useEffect(() => {
    if (!active) {
      setLevels(silent);
      return;
    }
    let cancelled = false;
    const id = setInterval(() => {
      api
        .recordingLevels()
        .then((next) => {
          if (cancelled) return;
          // Sobe na hora e desce devagar: o medidor fica legivel em vez de piscar.
          setLevels((prev) => ({
            microphone: Math.max(next.microphone, prev.microphone * 0.7),
            system: Math.max(next.system, prev.system * 0.7),
          }));
        })
        .catch(() => {});
    }, 100);
    return () => {
      cancelled = true;
      clearInterval(id);
    };
  }, [active]);

  return levels;
}
