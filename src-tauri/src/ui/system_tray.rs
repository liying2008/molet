use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Window,
};

use crate::{
    common::error::MoletError,
    conf::Config,
    op::data_op::DataOp,
    ui::notification::{NotificationPayload, NOTIFICATION_EVENT_NAME},
};

pub fn get_system_tray() -> SystemTray {
    let store = CustomMenuItem::new("store".to_string(), "Store Clipboard Data");
    let show = CustomMenuItem::new("show".to_string(), "Show Stored Data");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(store)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    return SystemTray::new().with_menu(tray_menu);
}

pub fn system_tray_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
            let molet_window = app.get_window("main");
            if let Some(molet_window) = molet_window {
                if let Ok(false) = molet_window.is_visible() {
                    molet_window.show().unwrap();
                    molet_window.set_focus().unwrap();
                } else if let Ok(true) = molet_window.is_visible() {
                    molet_window.hide().unwrap();
                }
            }
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "store" => {
                let molet_window = app.get_window("main");
                if let Some(molet_window) = molet_window {
                    store_clipboard_data(&molet_window)
                }
            }
            "show" => {
                let molet_window = app.get_window("main");
                if let Some(molet_window) = molet_window {
                    if let Ok(false) = molet_window.is_visible() {
                        molet_window.show().unwrap();
                        molet_window.set_focus().unwrap();
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn store_clipboard_data(window: &Window) {
    let config = match Config::load_conf() {
        Ok(config) => config,
        Err(e) => {
            println!("Error occurred: {}", e);
            window
                .emit(
                    NOTIFICATION_EVENT_NAME,
                    NotificationPayload {
                        title: "Error occurred".into(),
                        body: Some(format!("{}", e)),
                        icon: None,
                    }
                    .to_json()
                    .unwrap(),
                )
                .unwrap();
            return;
        }
    };
    match DataOp::clipboard_to_db(&config) {
        Ok(title) => {
            // 显示存储成功的通知
            println!("Store successfully: {}", title);
            window
                .emit(
                    NOTIFICATION_EVENT_NAME,
                    NotificationPayload {
                        title: "Success!".into(),
                        body: Some(title),
                        icon: None,
                    }
                    .to_json()
                    .unwrap(),
                )
                .unwrap();
        }
        Err(MoletError::Warning(s)) => {
            // 显示警告信息
            println!("Molet Warning: {}", s);
            window
                .emit(
                    NOTIFICATION_EVENT_NAME,
                    NotificationPayload {
                        title: "Warning!".into(),
                        body: Some(s),
                        icon: None,
                    }
                    .to_json()
                    .unwrap(),
                )
                .unwrap();
        }
        Err(MoletError::Info(s)) => {
            // 显示普通信息
            println!("Molet Info: {}", s);
            window
                .emit(
                    NOTIFICATION_EVENT_NAME,
                    NotificationPayload {
                        title: "Info:".into(),
                        body: Some(s),
                        icon: None,
                    }
                    .to_json()
                    .unwrap(),
                )
                .unwrap();
        }
        Err(e) => {
            // 显示存储失败信息
            println!("Failed to store data: {}", e);
            window
                .emit(
                    NOTIFICATION_EVENT_NAME,
                    NotificationPayload {
                        title: "Error!".into(),
                        body: Some(format!("{}", e)),
                        icon: None,
                    }
                    .to_json()
                    .unwrap(),
                )
                .unwrap();
        }
    };
}
