import { OutputDirSetting } from "../components/OutputDirSetting";
import { RecordButton } from "../components/RecordButton";
import { ScreenToggle } from "../components/ScreenToggle";
import { SessionList } from "../components/SessionList";
import { useOutputDir } from "../hooks/useOutputDir";
import { useRecording } from "../hooks/useRecording";
import { useSessions } from "../hooks/useSessions";
import { useTranscription } from "../hooks/useTranscription";

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
  const error = rec.error ?? transcription.error ?? outputDir.error ?? sessions.error;

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
        {rec.status.warnings.map((w) => (
          <p key={w} className="warning">{w}</p>
        ))}
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
        <SessionList
          sessions={sessions.sessions}
          transcribing={transcription.progress}
          onTranscribe={transcription.transcribe}
        />
      </section>
    </main>
  );
}
