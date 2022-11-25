#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use molet::{
    init_db, load_config,
    ui::system_tray::{get_system_tray, system_tray_handler},
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let config = load_config();
    init_db(&config);

    tauri::Builder::default()
        .system_tray(get_system_tray())
        .on_system_tray_event(system_tray_handler)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
