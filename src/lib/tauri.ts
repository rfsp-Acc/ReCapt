// Cliente tipado dos commands do backend (src-tauri/src/commands).
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface Session {
  id: string;
  name: string;
  path: string;
  /** `null` nas sessoes antigas, sem projeto. */
  project: string | null;
  front: string | null;
  /** `null` nas sessoes gravadas direto na frente, antes do nivel Tema. */
  theme: string | null;
  /** Assunto opcional da reuniao, parte do nome da pasta. */
  subject: string | null;
  hasAudio: boolean;
  hasVideo: boolean;
  hasTranscript: boolean;
  hasAnalysis: boolean;
  /** `null` quando a sessao nao tem audio.wav. */
  durationMs: number | null;
}

export interface Front {
  name: string;
  /** Pasta propria da frente; `null` grava dentro da pasta de salvamento. */
  dir: string | null;
  themes: string[];
}

export interface Project {
  name: string;
  fronts: Front[];
}

/** Projeto, frente e tema so quando a sessao nao os tem; `null` mantem o atual. */
export interface RenameTarget {
  project: string | null;
  front: string | null;
  theme: string | null;
  subject: string;
}

export interface Levels {
  /** Pico de 0 a 1 desde a ultima leitura. */
  microphone: number;
  system: number;
}

export type SessionFile = "transcricao.md" | "analise.md" | "acoes.md" | "transcricao-pt.md";

export type AudioSource = "microphone" | "system";

export interface RecordingStatus {
  recording: boolean;
  paused: boolean;
  sessionId: string | null;
  /** Tempo gravado no momento da consulta, sem contar pausas. */
  elapsedMs: number;
  screen: boolean;
  sources: AudioSource[];
  warnings: string[];
}

export interface TranscriptionProgress {
  sessionId: string;
  percent: number;
}

export interface InputDevice {
  id: string;
  name: string;
  isDefault: boolean;
}

export interface InputDevices {
  devices: InputDevice[];
  /** `null` segue o padrao do sistema. */
  selected: string | null;
}

export type TranscriptionLanguage = "auto" | "pt" | "en";

export interface OutputDir {
  path: string;
  isDefault: boolean;
}

export const api = {
  startRecording: (project: string, front: string, theme: string, subject: string) =>
    invoke<RecordingStatus>("start_recording", { project, front, theme, subject }),
  pauseRecording: () => invoke<RecordingStatus>("pause_recording"),
  resumeRecording: () => invoke<RecordingStatus>("resume_recording"),
  stopRecording: () => invoke<string>("stop_recording"),
  recordingStatus: () => invoke<RecordingStatus>("recording_status"),
  recordingLevels: () => invoke<Levels>("recording_levels"),
  listInputDevices: () => invoke<InputDevices>("list_input_devices"),
  setInputDevice: (id: string | null) => invoke<void>("set_input_device", { id }),
  /** Ids das sessoes interrompidas que ganharam audio.wav. */
  recoverInterrupted: () => invoke<string[]>("recover_interrupted"),
  listProjects: () => invoke<Project[]>("list_projects"),
  createProject: (name: string) => invoke<Project>("create_project", { name }),
  listSessions: () => invoke<Session[]>("list_sessions"),
  getSession: (id: string) => invoke<Session>("get_session", { id }),
  /** Abre o Finder para escolher a pasta; `null` quando o usuario cancela. */
  moveSession: (id: string) => invoke<Session | null>("move_session", { id }),
  renameSession: (id: string, target: RenameTarget) => invoke<Session>("rename_session", { id, target }),
  deleteSession: (id: string) => invoke<void>("delete_session", { id }),
  openSessionFolder: (id: string) => invoke<void>("open_session_folder", { id }),
  copyTranscriptPath: (id: string) => invoke<string>("copy_transcript_path", { id }),
  openSessionFile: (id: string, file: SessionFile) => invoke<void>("open_session_file", { id, file }),
  getOutputDir: () => invoke<OutputDir>("get_output_dir"),
  /** `null` quando o usuario cancela o seletor. */
  chooseOutputDir: () => invoke<OutputDir | null>("choose_output_dir"),
  resetOutputDir: () => invoke<OutputDir>("reset_output_dir"),
  openOutputDir: () => invoke<void>("open_output_dir"),
  /** Devolve o nome da frente com a grafia existente, ou `null` se o usuario cancelar. */
  chooseFrontDir: (project: string, front: string) => invoke<string | null>("choose_front_dir", { project, front }),
  resetFrontDir: (project: string, front: string) => invoke<string>("reset_front_dir", { project, front }),
  getTranscriptionLanguage: () => invoke<TranscriptionLanguage>("get_transcription_language"),
  setTranscriptionLanguage: (language: TranscriptionLanguage) =>
    invoke<TranscriptionLanguage>("set_transcription_language", { language }),
  transcribe: (id: string) => invoke<Session>("transcribe", { id }),
  onTranscriptionProgress: (cb: (p: TranscriptionProgress) => void) =>
    listen<TranscriptionProgress>("transcription-progress", (e) => cb(e.payload)),
};
