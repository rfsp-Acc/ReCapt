mod audio;
mod commands;
mod error;
mod session;
mod settings;
mod whisper;

use commands::{capture, files, transcribe};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let config_dir = app.path().app_config_dir()?;
            app.manage(settings::SettingsState::load(&config_dir));
            Ok(())
        })
        .manage(capture::CaptureState::default())
        .manage(transcribe::TranscribeState::default())
        .invoke_handler(tauri::generate_handler![
            capture::start_recording,
            capture::pause_recording,
            capture::resume_recording,
            capture::stop_recording,
            capture::toggle_screen_capture,
            capture::recording_status,
            files::list_sessions,
            files::get_session,
            files::open_session_folder,
            files::get_output_dir,
            files::choose_output_dir,
            files::reset_output_dir,
            files::open_output_dir,
            transcribe::transcribe,
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o ReCapt");
}
