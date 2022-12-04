use std::thread::spawn;

use chrono::Local;
use serde::Serialize;
use tauri::{App, AppHandle, Manager};

use crate::get_all_data;
pub const EVENT_GET_DATA: &str = "molet:get-data";
pub const EVENT_SEND_DATA: &str = "molet:send-data";

fn emit_event<T: Serialize + Clone>(app_handle: &AppHandle, event_name: &str, event_data: T) {
    let time1 = Local::now();
    app_handle.emit_all(event_name, event_data).unwrap();
    let time2 = Local::now();
    println!("emit event_data time: {}", time2 - time1);
}

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.app_handle();
    // spawn(move || {
    //     emit_event(&app_handle, EVENT_SEND_DATA, get_all_data());
    // });

    let id = app.listen_global(EVENT_GET_DATA, move |event| {
        println!("got event-name with payload {:?}", event.payload());
    });
    app.unlisten(id);

    Ok(())
}
