// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::Command;
use tauri::api::process::CommandEvent;

#[tauri::command]
async fn compile_ink(window: tauri::Window, content: String) -> Result<(), String> {
    // Write content to temp file
    let temp_path = std::env::temp_dir().join("compile.ink");
    std::fs::write(&temp_path, content).map_err(|e| e.to_string())?;

    let (mut rx, _child) = Command::new_sidecar("inklecate")
        .expect("failed to setup sidecar")
        .args(["-j", "-c", temp_path.to_str().unwrap()])
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compile_ink])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
