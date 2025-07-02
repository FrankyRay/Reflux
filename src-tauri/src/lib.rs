mod utils;

use crate::utils::{
    download::download_video,
    info::{get_video_data, VideoInfo},
    ytdlp::get_ytdlp_version,
};
use tauri::{AppHandle, Manager};

#[tauri::command]
async fn get_ytdlp_ver() -> Result<String, String> {
    get_ytdlp_version()
}

#[tauri::command]
async fn get_video_info(video_link: String) -> Result<VideoInfo, String> {
    get_video_data(video_link)
}

#[tauri::command]
async fn execute_video_download(app: AppHandle, args: Vec<String>, video_link: String) {
    let mut arguments = args.clone();
    arguments.push(video_link);
    download_video(app, arguments);
}

// Registering command
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        // .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            execute_video_download,
            get_video_info,
            get_ytdlp_ver,
        ])
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = _app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
