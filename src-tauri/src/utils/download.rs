use crate::utils::{shell::process_command, ytdlp::get_ytdlp};
use tauri::AppHandle;

pub fn download_video(app: AppHandle, args: Vec<String>) {
    let ytdlp = get_ytdlp().unwrap();
    process_command(app, ytdlp, args);
}
