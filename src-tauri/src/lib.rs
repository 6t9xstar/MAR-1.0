use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Emitter, State};
use tracing::info;

mod commands;
mod tray;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_url: String,
    pub theme: String,
    pub language: String,
}

pub struct AppState {
    pub api_url: String,
    pub token: Mutex<Option<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:8080".into(),
            token: Mutex::new(None),
        }
    }
}

#[tauri::command]
async fn get_app_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    Ok(AppConfig {
        api_url: state.api_url.clone(),
        theme: "system".into(),
        language: "en".into(),
    })
}

#[tauri::command]
async fn set_auth_token(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    token: String,
) -> Result<(), String> {
    *state.token.lock().unwrap() = Some(token);
    app.emit("auth-changed", true).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_auth_token(state: State<'_, AppState>) -> Result<Option<String>, String> {
    Ok(state.token.lock().unwrap().clone())
}

#[tauri::command]
async fn clear_auth_token(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    *state.token.lock().unwrap() = None;
    app.emit("auth-changed", false).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_app_config,
            set_auth_token,
            get_auth_token,
            clear_auth_token,
            commands::system::get_system_info,
            commands::system::open_url,
            commands::file::list_directory,
            commands::file::read_file,
            commands::file::write_file,
        ])
        .setup(|app| {
            tray::create_tray(app.handle())?;
            info!("MAR 1.0 desktop initialized");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error running MAR 1.0");
}
