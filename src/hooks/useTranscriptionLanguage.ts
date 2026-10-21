import { useEffect, useState } from "react";
import { api, TranscriptionLanguage } from "../lib/tauri";

export function useTranscriptionLanguage() {
  const [language, setLanguage] = useState<TranscriptionLanguage>("auto");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api.getTranscriptionLanguage().then(setLanguage).catch((e) => setError(String(e)));
  }, []);

  const change = async (next: TranscriptionLanguage) => {
    setError(null);
    try {
      setLanguage(await api.setTranscriptionLanguage(next));
    } catch (e) {
      setError(String(e));
    }
  };

  return { language, error, change };
}
