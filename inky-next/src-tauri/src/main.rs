// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::Command;
use tauri::api::process::CommandEvent;
use uuid::Uuid;

#[tauri::command]
async fn compile_ink(window: tauri::Window, content: String) -> Result<(), String> {
    let session_id = Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir().join("inky_next").join(&session_id);
    std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
    let temp_path = temp_dir.join("main.ink");
    std::fs::write(&temp_path, content).map_err(|e| e.to_string())?;

    let (mut rx, _child) = Command::new_sidecar("inklecate")
        .map_err(|e| e.to_string())?
        .args(["-j", "-p", "-c", temp_path.to_str().unwrap()])
        .spawn()
        .map_err(|e| e.to_string())?;

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                window.emit("ink-output", line).unwrap();
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn open_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(path, content).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compile_ink, open_file, save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
