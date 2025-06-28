use crate::structures::StorageData;
use dotenvy_macro::dotenv;
use std::sync::Mutex;
use std::{env, fs};
use tauri::{App, Manager};
use tauri_plugin_store::StoreExt;
mod api;
mod structures;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        // Debug window on npm run tauri dev
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            init_storage(app);
            default_store(app);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            api::authentication,
            api::check_token,
            api::get_clients,
            api::get_client_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// TODO придумал способ будем сохранять путь до директории в settings.json в системной папки лаунчера
// WARNING после выпуска проекта в пред релиз надо будет проверить работу fn в snap и flatpak

fn default_store(app: &mut App) {
    let store = app.store("settings.json").unwrap();
    let state = app.state::<Mutex<StorageData>>();
    let state = state.lock().unwrap();
    if store.is_empty() {
        store.set("dir", state.storage_dir.clone().to_str().unwrap());
        store.set("access_token", "");
    }
}

fn init_storage(app: &mut App) {
    app.manage(Mutex::new(StorageData::default()));
    let state = app.state::<Mutex<StorageData>>();
    let mut state = state.lock().unwrap();
    state.storage_dir = app
        .path()
        .home_dir()
        .unwrap()
        .join(dotenv!("DERICTORY"))
        .to_owned();
    let _ = fs::create_dir_all(&state.storage_dir);
    state.access_token = state.storage_dir.clone().join("access_token");
}