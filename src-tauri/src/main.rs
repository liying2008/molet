#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use molet::{
    conf::Config,
    data::wrapper,
    init_db, load_config,
    ui::system_tray::{get_system_tray, system_tray_handler},
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_all_data() -> String {
    let config = Config::load_conf().unwrap();
    let r = wrapper::get_all(&config);
    match r {
        Ok(data) => serde_json::to_string(&data).unwrap(),
        Err(e) => e.to_string(),
    }
}

fn main() {
    let config = load_config();
    init_db(&config);

    tauri::Builder::default()
        .system_tray(get_system_tray())
        .on_system_tray_event(system_tray_handler)
        .invoke_handler(tauri::generate_handler![get_all_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
