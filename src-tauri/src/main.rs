// 防止在 Windows 发布版本中出现额外的控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod video;
mod error;
mod utils;
mod media;

use video::VideoInfo;

#[tauri::command]
fn check_ffmpeg() -> Result<bool, String> {
    media::check_ffmpeg_installed()
}

#[tauri::command]
fn get_video_info(path: String) -> Result<VideoInfo, String> {
    media::get_video_info(&path)
}

#[tauri::command]
fn cut_video(input: String, start: f64, end: f64, notes: Option<String>) -> Result<String, String> {
    media::cut_video(&input, start, end, notes.as_deref())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_ffmpeg,
            get_video_info,
            cut_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
