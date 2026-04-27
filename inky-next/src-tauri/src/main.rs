// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::Command;
use tauri::api::process::CommandChild;
use tauri::api::process::CommandEvent;
use std::path::PathBuf;
use std::sync::Mutex;
use uuid::Uuid;

struct AppState {
    active_process: Mutex<Option<CommandChild>>,
}

#[tauri::command]
async fn get_temp_project_dir() -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir().join("inky_next").join(&session_id);
    std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
    Ok(temp_dir.to_str().unwrap().to_string())
}

#[tauri::command]
async fn compile_ink(
    window: tauri::Window,
    state: tauri::State<'_, AppState>,
    main_path: String,
) -> Result<(), String> {
    let mut lock = state.active_process.lock().unwrap();
    if let Some(child) = lock.take() {
        child.kill().ok();
    }

    let (mut rx, child) = Command::new_sidecar("inklecate")
        .map_err(|e| e.to_string())?
        .args(["-p", "-j", "-k", "-c", &main_path])
        .spawn()
        .map_err(|e| e.to_string())?;

    *lock = Some(child);

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
async fn choose_path(state: tauri::State<'_, AppState>, choice_index: usize) -> Result<(), String> {
    let mut lock = state.active_process.lock().unwrap();
    if let Some(child) = lock.as_mut() {
        child
            .write(format!("{}\n", choice_index).as_bytes())
            .map_err(|e| e.to_string())?;
    }
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

#[tauri::command]
async fn list_project_files(project_path: String) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    let dir = PathBuf::from(&project_path);
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ink") {
                if let Some(p) = path.to_str() {
                    files.push(p.to_string());
                }
            }
        }
    }
    Ok(files)
}

#[tauri::command]
async fn create_file(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if p.exists() {
        return Err("File already exists".to_string());
    }
    // Ensure parent directory exists
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, "").map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn delete_file(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err("File does not exist".to_string());
    }
    std::fs::remove_file(p).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn save_project(source_path: String, target_path: String) -> Result<(), String> {
    let source = PathBuf::from(&source_path);
    let target = PathBuf::from(&target_path);

    if !source.exists() {
        return Err("Source project directory does not exist".to_string());
    }

    if !target.exists() {
        std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    }

    for entry in std::fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            let dest_path = target.join(file_name);
            std::fs::copy(&path, &dest_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn cleanup_temp() {
    let temp_dir = std::env::temp_dir().join("inky_next");
    if temp_dir.exists() {
        std::fs::remove_dir_all(temp_dir).ok();
    }
}

fn main() {
    cleanup_temp();
    tauri::Builder::default()
        .manage(AppState {
            active_process: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_temp_project_dir,
            compile_ink,
            choose_path,
            open_file,
            save_file,
            list_project_files,
            create_file,
            delete_file,
            save_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
