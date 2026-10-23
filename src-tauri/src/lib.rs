mod audio;
mod commands;
mod error;
mod session;
mod settings;
mod whisper;

use commands::{capture, files, model, transcribe};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let config_dir = app.path().app_config_dir()?;
            app.manage(settings::SettingsState::load(&config_dir));
            Ok(())
        })
        .manage(capture::CaptureState::default())
        .manage(transcribe::TranscribeState::default())
        .manage(model::ModelState::default())
        .invoke_handler(tauri::generate_handler![
            capture::start_recording,
            capture::pause_recording,
            capture::resume_recording,
            capture::stop_recording,
            capture::toggle_screen_capture,
            capture::recording_status,
            capture::recording_levels,
            capture::list_input_devices,
            capture::set_input_device,
            capture::recover_interrupted,
            files::list_sessions,
            files::list_projects,
            files::create_project,
            files::get_session,
            files::move_session,
            files::rename_session,
            files::delete_session,
            files::open_session_folder,
            files::open_session_file,
            files::copy_transcript_path,
            files::get_output_dir,
            files::choose_output_dir,
            files::reset_output_dir,
            files::open_output_dir,
            files::choose_front_dir,
            files::reset_front_dir,
            model::model_status,
            model::download_model,
            transcribe::transcribe,
            transcribe::get_transcription_language,
            transcribe::set_transcription_language,
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o ReCapt");
}
