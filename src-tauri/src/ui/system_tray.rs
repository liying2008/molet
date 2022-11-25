use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

use crate::{common::error::MoletError, conf::Config, op::data_op::DataOp};

pub fn get_system_tray() -> SystemTray {
    let store = CustomMenuItem::new("store".to_string(), "Store Clipboard Data");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(store)
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
            "store" => store_clipboard_data(),
            _ => {}
        },
        _ => {}
    }
}

fn store_clipboard_data() {
    let config = match Config::load_conf() {
        Ok(config) => config,
        Err(e) => {
            println!("Error occurred: {}", e);
            // nwg::modal_error_message(
            //     &self.window,
            //     "Error",
            //     format!("Error occurred: {}", e).as_str(),
            // );
            return;
        }
    };
    match DataOp::clipboard_to_db(&config) {
        Ok(title) => {
            // 显示存储成功的通知
            println!("Store successfully: {}", title);
            // let flags =
            //     nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
            // self.tray.show(
            //     title.as_str(),
            //     Some("Store successfully"),
            //     Some(flags),
            //     Some(&self.icon),
            // );
        }
        Err(MoletError::Warning(s)) => {
            // 显示警告信息
            println!("Molet Warning: {}", s);
            // let flags = nwg::TrayNotificationFlags::WARNING_ICON
            //     | nwg::TrayNotificationFlags::LARGE_ICON;
            // self.tray.show(
            //     s.as_str(),
            //     Some("Molet Warning"),
            //     Some(flags),
            //     Some(&self.icon),
            // );
        }
        Err(MoletError::Info(s)) => {
            // 显示普通信息
            println!("Molet Info: {}", s);
            // let flags =
            //     nwg::TrayNotificationFlags::INFO_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
            // self.tray.show(
            //     s.as_str(),
            //     Some("Molet Info"),
            //     Some(flags),
            //     Some(&self.icon),
            // );
        }
        Err(e) => {
            // 显示存储失败信息
            println!("Failed to store data: {}", e);
            // let flags =
            //     nwg::TrayNotificationFlags::ERROR_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
            // self.tray.show(
            //     e.to_string().as_str(),
            //     Some("Failed to store data"),
            //     Some(flags),
            //     Some(&self.icon),
            // );
        }
    };
}
