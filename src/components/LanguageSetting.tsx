import { TranscriptionLanguage } from "../lib/tauri";

interface Props {
  language: TranscriptionLanguage;
  onChange: (language: TranscriptionLanguage) => void;
}

const OPTIONS: { value: TranscriptionLanguage; label: string }[] = [
  { value: "auto", label: "Automático" },
  { value: "pt", label: "Português" },
  { value: "en", label: "Inglês" },
];

export function LanguageSetting({ language, onChange }: Props) {
  return (
    <label className="language-setting">
      <span className="output-dir-label">Idioma da transcrição</span>
      <select value={language} onChange={(e) => onChange(e.target.value as TranscriptionLanguage)}>
        {OPTIONS.map((o) => (
          <option key={o.value} value={o.value}>
            {o.label}
          </option>
        ))}
      </select>
      {language === "auto" && (
        <span className="hint">Detectado nos primeiros 30 s. Em reuniões mistas, escolha o idioma predominante.</span>
      )}
    </label>
  );
}
