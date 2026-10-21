import { useCallback, useEffect, useState } from "react";
import { AddProjectDialog } from "../components/AddProjectDialog";
import { CaptureCard } from "../components/CaptureCard";
import { Icon } from "../components/Icon";
import { Notices } from "../components/Notices";
import { SessionList } from "../components/SessionList";
import { SettingsPanel } from "../components/SettingsPanel";
import { matches, Selection, Sidebar } from "../components/Sidebar";
import { useInputDevices } from "../hooks/useInputDevices";
import { useLastTarget } from "../hooks/useLastTarget";
import { useLevels } from "../hooks/useLevels";
import { useErrorNotice, useNotices } from "../hooks/useNotices";
import { useOutputDir } from "../hooks/useOutputDir";
import { useProjects } from "../hooks/useProjects";
import { useRecording } from "../hooks/useRecording";
import { useSessions } from "../hooks/useSessions";
import { useTranscription } from "../hooks/useTranscription";
import { useTranscriptionLanguage } from "../hooks/useTranscriptionLanguage";
import { api } from "../lib/tauri";

export function Home() {
  const notices = useNotices();
  const sessions = useSessions();
  const projects = useProjects();
  const refreshAll = useCallback(() => {
    sessions.refresh();
    // Frentes novas aparecem na barra lateral assim que a primeira sessao e criada.
    projects.refresh();
  }, [sessions.refresh, projects.refresh]);
  const transcription = useTranscription(sessions.refresh);
  // Ao parar, a sessao aparece na lista e a transcricao comeca automaticamente.
  const rec = useRecording((id) => {
    refreshAll();
    transcription.transcribe(id);
  });
  const levels = useLevels(rec.status.recording && !rec.status.paused);
  // Trocar a pasta muda quais projetos e sessoes aparecem.
  const outputDir = useOutputDir(refreshAll);
  const language = useTranscriptionLanguage();
  const mics = useInputDevices();
  const [target, setTarget] = useLastTarget();
  const [selection, setSelection] = useState<Selection>({ kind: "all" });
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [addProjectOpen, setAddProjectOpen] = useState(false);
  const closeSettings = useCallback(() => setSettingsOpen(false), []);
  const closeAddProject = useCallback(() => setAddProjectOpen(false), []);

  useErrorNotice(rec.error, notices.push);
  useErrorNotice(transcription.error, notices.push);
  useErrorNotice(outputDir.error, notices.push);
  useErrorNotice(language.error, notices.push);
  useErrorNotice(mics.error, notices.push);
  useErrorNotice(sessions.error, notices.push);
  useErrorNotice(projects.error, notices.push);

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

  // Escolher um projeto ou frente na barra lateral ja prepara a proxima gravacao.
  const select = (sel: Selection) => {
    setSelection(sel);
    if (rec.status.recording) return;
    if (sel.kind === "project" && sel.project !== target.project) setTarget({ project: sel.project, front: "" });
    if (sel.kind === "front") setTarget({ project: sel.project, front: sel.front });
  };

  const createProject = async (name: string) => {
    const project = await projects.create(name);
    setTarget({ project: project.name, front: "" });
    setSelection({ kind: "project", project: project.name });
    notices.push("success", `Projeto "${project.name}" criado.`);
  };

  const visible = sessions.sessions.filter((s) => matches(s, selection));
  const title =
    selection.kind === "all"
      ? "Todas as sessões"
      : selection.kind === "none"
        ? "Sem projeto"
        : selection.kind === "project"
          ? selection.project
          : `${selection.project} / ${selection.front}`;

  return (
    <div className="app">
      <Sidebar
        projects={projects.projects}
        sessions={sessions.sessions}
        selection={selection}
        onSelect={select}
        onAddProject={() => setAddProjectOpen(true)}
      />

      <div className="main">
        <header className="topbar">
          {/* Mesma largura do conteudo: titulo e engrenagem alinhados com o cartao. */}
          <div className="topbar-inner">
            <h1 title={title}>{title}</h1>
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
            projects={projects.projects}
            target={target}
            onTarget={setTarget}
            onAddProject={() => setAddProjectOpen(true)}
            onStart={() => rec.start(target.project, target.front)}
            onStop={rec.stop}
            onPause={rec.pause}
            onResume={rec.resume}
          />

          <section className="sessions" aria-labelledby="sessions-title">
            <h2 id="sessions-title">Sessões</h2>
            <SessionList
              sessions={visible}
              transcribing={transcription.progress}
              onTranscribe={transcription.transcribe}
              onCopied={(text) => notices.push("success", text)}
              onError={(text) => notices.push("error", text)}
              context={selection.kind === "all" ? "full" : selection.kind === "project" ? "front" : "none"}
              emptyText={
                selection.kind === "all"
                  ? "Escolha o projeto, informe a frente e clique no botão vermelho para gravar sua primeira reunião."
                  : "As reuniões gravadas aqui aparecem nesta lista, agrupadas por dia."
              }
            />
          </section>
        </main>
      </div>

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

      <AddProjectDialog open={addProjectOpen} onClose={closeAddProject} onCreate={createProject} />

      <Notices notices={notices.notices} onDismiss={notices.dismiss} />
    </div>
  );
}
