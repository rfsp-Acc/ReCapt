import { useCallback, useEffect, useState } from "react";
import { AddProjectDialog } from "../components/AddProjectDialog";
import { CaptureCard } from "../components/CaptureCard";
import { DeleteSessionDialog } from "../components/DeleteSessionDialog";
import { Icon } from "../components/Icon";
import { Notices } from "../components/Notices";
import { RenameSessionDialog } from "../components/RenameSessionDialog";
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
import { shortPath } from "../lib/format";
import { api, RenameTarget, Session } from "../lib/tauri";

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
  const [subject, setSubject] = useState("");
  const [selection, setSelection] = useState<Selection>({ kind: "all" });
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [addProjectOpen, setAddProjectOpen] = useState(false);
  const closeSettings = useCallback(() => setSettingsOpen(false), []);
  const closeAddProject = useCallback(() => setAddProjectOpen(false), []);
  const [renaming, setRenaming] = useState<Session | null>(null);
  const closeRename = useCallback(() => setRenaming(null), []);
  const [deleting, setDeleting] = useState<Session | null>(null);
  const closeDelete = useCallback(() => setDeleting(null), []);

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
    if (sel.kind === "project" && sel.project !== target.project) setTarget({ project: sel.project, front: "", theme: "" });
    if (sel.kind === "front" && (sel.project !== target.project || sel.front !== target.front))
      setTarget({ project: sel.project, front: sel.front, theme: "" });
    if (sel.kind === "theme" && sel.theme) setTarget({ project: sel.project, front: sel.front, theme: sel.theme });
  };

  const createProject = async (name: string) => {
    const project = await projects.create(name);
    setTarget({ project: project.name, front: "", theme: "" });
    setSelection({ kind: "project", project: project.name });
    notices.push("success", `Projeto "${project.name}" criado.`);
  };

  // A pasta vale para a frente: todas as proximas reunioes dela vao para la.
  const changeFrontDir = async (reset: boolean) => {
    try {
      const front = reset
        ? await api.resetFrontDir(target.project, target.front)
        : await api.chooseFrontDir(target.project, target.front);
      if (!front) return;
      setTarget({ front });
      refreshAll();
      notices.push(
        "success",
        reset
          ? `A frente "${front}" volta a gravar na pasta de salvamento.`
          : `As próximas reuniões da frente "${front}" serão salvas na pasta escolhida.`,
      );
    } catch (e) {
      notices.push("error", String(e));
    }
  };

  const moveSession = async (session: Session) => {
    try {
      const moved = await api.moveSession(session.id);
      if (!moved) return;
      refreshAll();
      const place = [moved.project, moved.front, moved.theme].join(" / ");
      const parent = moved.path.slice(0, moved.path.lastIndexOf("/"));
      notices.push(
        "success",
        place === [session.project, session.front, session.theme].join(" / ")
          ? `Reunião movida para ${shortPath(parent)}. No app, ela continua em ${place}.`
          : `Reunião movida para ${place}.`,
      );
    } catch (e) {
      notices.push("error", String(e));
    }
  };

  const renameSession = async (session: Session, target: RenameTarget) => {
    const renamed = await api.renameSession(session.id, target);
    refreshAll();
    notices.push(
      "success",
      session.theme === null ? `Reunião organizada em ${[renamed.project, renamed.front, renamed.theme].join(" / ")}.` : "Assunto atualizado.",
    );
  };

  const deleteSession = async (session: Session) => {
    await api.deleteSession(session.id);
    refreshAll();
    notices.push("success", "Gravação excluída.");
  };

  const visible = sessions.sessions.filter((s) => matches(s, selection));
  const title =
    selection.kind === "all"
      ? "Todas as sessões"
      : selection.kind === "none"
        ? "Sem projeto"
        : selection.kind === "project"
          ? selection.project
          : selection.kind === "front"
            ? `${selection.project} / ${selection.front}`
            : `${selection.project} / ${selection.front} / ${selection.theme ?? "Sem tema"}`;

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
            outputDir={outputDir.dir?.path ?? null}
            onChooseFrontDir={() => changeFrontDir(false)}
            onResetFrontDir={() => changeFrontDir(true)}
            subject={subject}
            onSubject={setSubject}
            onStart={() => {
              rec.start(target.project, target.front, target.theme, subject);
              // O assunto vale so para esta reuniao.
              setSubject("");
            }}
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
              onMove={moveSession}
              onRename={setRenaming}
              onDelete={setDeleting}
              context={
                { all: "full", none: "none", project: "front", front: "theme", theme: "none" }[selection.kind] as
                  | "full"
                  | "front"
                  | "theme"
                  | "none"
              }
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

      <RenameSessionDialog session={renaming} projects={projects.projects} onClose={closeRename} onRename={renameSession} />

      <DeleteSessionDialog session={deleting} onClose={closeDelete} onDelete={deleteSession} />

      <AddProjectDialog open={addProjectOpen} onClose={closeAddProject} onCreate={createProject} />

      <Notices notices={notices.notices} onDismiss={notices.dismiss} />
    </div>
  );
}
