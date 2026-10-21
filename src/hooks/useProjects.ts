import { useCallback, useEffect, useState } from "react";
import { api, Project } from "../lib/tauri";

export function useProjects() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(() => {
    api.listProjects().then(setProjects).catch((e) => setError(String(e)));
  }, []);

  useEffect(refresh, [refresh]);

  /** Lanca o erro do backend (nome invalido, duplicado) para a janela mostrar. */
  const create = useCallback(
    async (name: string) => {
      const project = await api.createProject(name);
      refresh();
      return project;
    },
    [refresh],
  );

  return { projects, error, refresh, create };
}
