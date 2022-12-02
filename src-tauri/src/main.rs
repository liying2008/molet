#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use molet::{
    init_db, load_config,
    ui::{
        setup::setup_app,
        system_tray::{get_system_tray, system_tray_handler},
    },
};

fn main() {
    let config = load_config();
    init_db(&config);

    tauri::Builder::default()
        .setup(setup_app)
        .system_tray(get_system_tray())
        .on_system_tray_event(system_tray_handler)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
