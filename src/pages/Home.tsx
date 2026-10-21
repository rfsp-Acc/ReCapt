import { useEffect, useState } from "react";
import { LanguageSetting } from "../components/LanguageSetting";
import { MicrophoneSetting } from "../components/MicrophoneSetting";
import { OutputDirSetting } from "../components/OutputDirSetting";
import { RecordButton } from "../components/RecordButton";
import { ScreenToggle } from "../components/ScreenToggle";
import { SessionList } from "../components/SessionList";
import { useInputDevices } from "../hooks/useInputDevices";
import { useOutputDir } from "../hooks/useOutputDir";
import { useRecording } from "../hooks/useRecording";
import { useSessions } from "../hooks/useSessions";
import { useTranscription } from "../hooks/useTranscription";
import { useTranscriptionLanguage } from "../hooks/useTranscriptionLanguage";
import { api } from "../lib/tauri";

export function Home() {
  const sessions = useSessions();
  const transcription = useTranscription(sessions.refresh);
  // Ao parar, a sessao aparece na lista e a transcricao comeca automaticamente.
  const rec = useRecording((id) => {
    sessions.refresh();
    transcription.transcribe(id);
  });
  // Trocar a pasta muda quais sessoes aparecem na lista.
  const outputDir = useOutputDir(sessions.refresh);
  const language = useTranscriptionLanguage();
  const mics = useInputDevices();
  const [recovered, setRecovered] = useState<string[]>([]);
  const error =
    rec.error ?? transcription.error ?? outputDir.error ?? language.error ?? mics.error ?? sessions.error;

  // Gravacoes interrompidas (app fechado no meio) ganham audio.wav e podem ser transcritas.
  useEffect(() => {
    api
      .recoverInterrupted()
      .then((ids) => {
        if (ids.length === 0) return;
        setRecovered(ids);
        sessions.refresh();
      })
      .catch(() => {});
  }, [sessions.refresh]);

  return (
    <main className="home">
      <header>
        <h1>ReCapt</h1>
      </header>

      <section className="capture">
        <RecordButton
          recording={rec.status.recording}
          paused={rec.status.paused}
          saving={rec.saving}
          elapsed={rec.elapsed}
          onStart={rec.start}
          onStop={rec.stop}
          onPause={rec.pause}
          onResume={rec.resume}
        />
        <ScreenToggle
          enabled={rec.status.screen}
          disabled={!rec.status.recording}
          onToggle={rec.toggleScreen}
        />
        {rec.status.recording && (
          <p className="sources">
            Gravando: {rec.status.sources.map((s) => (s === "microphone" ? "microfone" : "áudio do sistema")).join(" + ")}
          </p>
        )}
        {/* So os avisos mais recentes: trocas de dispositivo podem se acumular numa reuniao longa. */}
        {rec.status.warnings.slice(-3).map((w) => (
          <p key={w} className="warning">{w}</p>
        ))}
        {recovered.length > 0 && (
          <p className="warning">
            {recovered.length === 1
              ? "1 gravação interrompida foi recuperada."
              : `${recovered.length} gravações interrompidas foram recuperadas.`}{" "}
            Use "Transcrever" na lista abaixo.
          </p>
        )}
        {error && <p className="error">{error}</p>}
      </section>

      <section>
        <h2>Sessões</h2>
        <OutputDirSetting
          dir={outputDir.dir}
          disabled={rec.status.recording || rec.saving}
          onChoose={outputDir.choose}
          onReset={outputDir.reset}
          onOpen={outputDir.open}
        />
        <MicrophoneSetting
          devices={mics.devices}
          selected={mics.selected}
          onSelect={mics.select}
          onRefresh={mics.refresh}
        />
        <LanguageSetting language={language.language} onChange={language.change} />
        <SessionList
          sessions={sessions.sessions}
          transcribing={transcription.progress}
          onTranscribe={transcription.transcribe}
        />
      </section>
    </main>
  );
}
