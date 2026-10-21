// Cliente tipado dos commands do backend (src-tauri/src/commands).
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface Session {
  id: string;
  name: string;
  path: string;
  hasAudio: boolean;
  hasVideo: boolean;
  hasTranscript: boolean;
  hasAnalysis: boolean;
  /** `null` quando a sessao nao tem audio.wav. */
  durationMs: number | null;
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
  startRecording: () => invoke<RecordingStatus>("start_recording"),
  pauseRecording: () => invoke<RecordingStatus>("pause_recording"),
  resumeRecording: () => invoke<RecordingStatus>("resume_recording"),
  stopRecording: () => invoke<string>("stop_recording"),
  recordingStatus: () => invoke<RecordingStatus>("recording_status"),
  recordingLevels: () => invoke<Levels>("recording_levels"),
  listInputDevices: () => invoke<InputDevices>("list_input_devices"),
  setInputDevice: (id: string | null) => invoke<void>("set_input_device", { id }),
  /** Ids das sessoes interrompidas que ganharam audio.wav. */
  recoverInterrupted: () => invoke<string[]>("recover_interrupted"),
  listSessions: () => invoke<Session[]>("list_sessions"),
  getSession: (id: string) => invoke<Session>("get_session", { id }),
  openSessionFolder: (id: string) => invoke<void>("open_session_folder", { id }),
  copyTranscriptPath: (id: string) => invoke<string>("copy_transcript_path", { id }),
  openSessionFile: (id: string, file: SessionFile) => invoke<void>("open_session_file", { id, file }),
  getOutputDir: () => invoke<OutputDir>("get_output_dir"),
  /** `null` quando o usuario cancela o seletor. */
  chooseOutputDir: () => invoke<OutputDir | null>("choose_output_dir"),
  resetOutputDir: () => invoke<OutputDir>("reset_output_dir"),
  openOutputDir: () => invoke<void>("open_output_dir"),
  getTranscriptionLanguage: () => invoke<TranscriptionLanguage>("get_transcription_language"),
  setTranscriptionLanguage: (language: TranscriptionLanguage) =>
    invoke<TranscriptionLanguage>("set_transcription_language", { language }),
  transcribe: (id: string) => invoke<Session>("transcribe", { id }),
  onTranscriptionProgress: (cb: (p: TranscriptionProgress) => void) =>
    listen<TranscriptionProgress>("transcription-progress", (e) => cb(e.payload)),
};
