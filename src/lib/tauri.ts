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
}

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

export interface OutputDir {
  path: string;
  isDefault: boolean;
}

export const api = {
  startRecording: () => invoke<RecordingStatus>("start_recording"),
  pauseRecording: () => invoke<RecordingStatus>("pause_recording"),
  resumeRecording: () => invoke<RecordingStatus>("resume_recording"),
  stopRecording: () => invoke<string>("stop_recording"),
  toggleScreenCapture: () => invoke<boolean>("toggle_screen_capture"),
  recordingStatus: () => invoke<RecordingStatus>("recording_status"),
  listSessions: () => invoke<Session[]>("list_sessions"),
  getSession: (id: string) => invoke<Session>("get_session", { id }),
  openSessionFolder: (id: string) => invoke<void>("open_session_folder", { id }),
  getOutputDir: () => invoke<OutputDir>("get_output_dir"),
  /** `null` quando o usuario cancela o seletor. */
  chooseOutputDir: () => invoke<OutputDir | null>("choose_output_dir"),
  resetOutputDir: () => invoke<OutputDir>("reset_output_dir"),
  openOutputDir: () => invoke<void>("open_output_dir"),
  transcribe: (id: string) => invoke<Session>("transcribe", { id }),
  onTranscriptionProgress: (cb: (p: TranscriptionProgress) => void) =>
    listen<TranscriptionProgress>("transcription-progress", (e) => cb(e.payload)),
};
