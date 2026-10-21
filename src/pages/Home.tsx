import { useCallback, useEffect, useState } from "react";
import { CaptureCard } from "../components/CaptureCard";
import { Icon } from "../components/Icon";
import { Notices } from "../components/Notices";
import { SessionList } from "../components/SessionList";
import { SettingsPanel } from "../components/SettingsPanel";
import { useInputDevices } from "../hooks/useInputDevices";
import { useLevels } from "../hooks/useLevels";
import { useErrorNotice, useNotices } from "../hooks/useNotices";
import { useOutputDir } from "../hooks/useOutputDir";
import { useRecording } from "../hooks/useRecording";
import { useSessions } from "../hooks/useSessions";
import { useTranscription } from "../hooks/useTranscription";
import { useTranscriptionLanguage } from "../hooks/useTranscriptionLanguage";
import { api } from "../lib/tauri";

export function Home() {
  const notices = useNotices();
  const sessions = useSessions();
  const transcription = useTranscription(sessions.refresh);
  // Ao parar, a sessao aparece na lista e a transcricao comeca automaticamente.
  const rec = useRecording((id) => {
    sessions.refresh();
    transcription.transcribe(id);
  });
  const levels = useLevels(rec.status.recording && !rec.status.paused);
  // Trocar a pasta muda quais sessoes aparecem na lista.
  const outputDir = useOutputDir(sessions.refresh);
  const language = useTranscriptionLanguage();
  const mics = useInputDevices();
  const [settingsOpen, setSettingsOpen] = useState(false);
  const closeSettings = useCallback(() => setSettingsOpen(false), []);

  useErrorNotice(rec.error, notices.push);
  useErrorNotice(transcription.error, notices.push);
  useErrorNotice(outputDir.error, notices.push);
  useErrorNotice(language.error, notices.push);
  useErrorNotice(mics.error, notices.push);
  useErrorNotice(sessions.error, notices.push);

  // Gravacoes interrompidas (app fechado no meio) ganham audio.wav e podem ser transcritas.
  useEffect(() => {
    api
      .recoverInterrupted()
      .then((ids) => {
        if (ids.length === 0) return;
        sessions.refresh();
        notices.push(
          "warning",
          ids.length === 1
            ? "1 gravação interrompida foi recuperada. Ela está na lista, pronta para transcrever."
            : `${ids.length} gravações interrompidas foram recuperadas. Elas estão na lista, prontas para transcrever.`,
        );
      })
      .catch(() => {});
  }, [sessions.refresh, notices.push]);

  return (
    <div className="app">
      <header className="topbar">
        {/* Mesma largura do conteudo: titulo e engrenagem alinhados com o cartao. */}
        <div className="topbar-inner">
          <h1>ReCapt</h1>
          <button className="icon-button" onClick={() => setSettingsOpen(true)} aria-label="Configurações" title="Configurações">
            <Icon name="settings" />
          </button>
        </div>
      </header>

      <main className="content">
        <CaptureCard
          status={rec.status}
          elapsed={rec.elapsed}
          saving={rec.saving}
          levels={levels}
          mics={mics.devices}
          selectedMic={mics.selected}
          onSelectMic={mics.select}
          onRefreshMics={mics.refresh}
          onStart={rec.start}
          onStop={rec.stop}
          onPause={rec.pause}
          onResume={rec.resume}
        />

        <section className="sessions" aria-labelledby="sessions-title">
          <h2 id="sessions-title">Sessões</h2>
          <SessionList
            sessions={sessions.sessions}
            transcribing={transcription.progress}
            onTranscribe={transcription.transcribe}
            onCopied={(text) => notices.push("success", text)}
            onError={(text) => notices.push("error", text)}
          />
        </section>
      </main>

      <SettingsPanel
        open={settingsOpen}
        onClose={closeSettings}
        recording={rec.status.recording || rec.saving}
        outputDir={outputDir.dir}
        onChooseDir={outputDir.choose}
        onResetDir={outputDir.reset}
        onOpenDir={outputDir.open}
        mics={mics.devices}
        selectedMic={mics.selected}
        onSelectMic={mics.select}
        onRefreshMics={mics.refresh}
        language={language.language}
        onLanguage={language.change}
      />

      <Notices notices={notices.notices} onDismiss={notices.dismiss} />
    </div>
  );
}
