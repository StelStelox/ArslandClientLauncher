use std::{path::PathBuf, sync::Mutex};
use std::env;
use tauri::{App, Manager};
mod api;
mod structures;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[derive(Default)]
#[allow(non_snake_case)]
struct StorageData {
    storage_dir: PathBuf,
    accessToken: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            api::authentication,
            api::check_token,
            api::get_clients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// TODO нет смысла создавать костомную директорию, так как LazyStore не умеет до сих не умеет работать с кастомной директории (Нужен точный путь до директории а не относительный)

fn init_storage(app: &mut App) {
    app.manage(Mutex::new(StorageData::default()));
    let state = app.state::<Mutex<StorageData>>();
    let mut state = state.lock().unwrap();
    state.accessToken = state.storage_dir.clone().join("accessToken");
}