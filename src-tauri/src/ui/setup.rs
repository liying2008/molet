use tauri::{App, Event, Manager, Window};

use crate::get_all_data;
pub const EVENT_GET_DATA: &str = "get_data";
pub const EVENT_SEND_DATA: &str = "send_data";

fn emit_event(window: &Window, event_name: &str, event_data: String) {
    window.emit(event_name, event_data).unwrap();
}

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_window("main").unwrap();

    let id = main_window.listen(EVENT_GET_DATA, |event| {
        println!("got window event-name with payload {:?}", event.payload());
        // let main_window = app.get_window("main").unwrap();
        // emit_event(&main_window, EVENT_SEND_DATA, get_all_data())
    });
    main_window.unlisten(id);

    Ok(())
}
