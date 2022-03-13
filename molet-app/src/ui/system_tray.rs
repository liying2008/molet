extern crate native_windows_gui as nwg;

use crate::common::error::MoletError;
use crate::op::data_op::DataOp;
use crate::ui::data_table::DataTable;
use crate::Config;
use native_windows_gui::NativeUi;

#[derive(Default)]
pub struct SystemTray {
    pub window: nwg::MessageWindow,
    pub icon: nwg::Icon,
    pub tray: nwg::TrayNotification,
    pub tray_menu: nwg::Menu,
    pub tray_item_data_push: nwg::MenuItem,
    pub tray_item_data_show: nwg::MenuItem,
    pub tray_item_exit: nwg::MenuItem,
}

impl SystemTray {
    pub fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    pub fn store_clipboard_data(&self) {
        let config = match Config::load_conf() {
            Ok(config) => config,
            Err(e) => {
                println!("Error occurred: {}", e);
                nwg::modal_error_message(
                    &self.window,
                    "Error",
                    format!("Error occurred: {}", e).as_str(),
                );
                return;
            }
        };
        match DataOp::clipboard_to_db(&config) {
            Ok(title) => {
                // 显示存储成功的通知
                println!("Store successfully: {}", title);
                let flags =
                    nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
                self.tray.show(
                    title.as_str(),
                    Some("Store successfully"),
                    Some(flags),
                    Some(&self.icon),
                );
            }
            Err(MoletError::Warning(s)) => {
                // 显示警告信息
                println!("Molet Warning: {}", s);
                let flags = nwg::TrayNotificationFlags::WARNING_ICON
                    | nwg::TrayNotificationFlags::LARGE_ICON;
                self.tray.show(
                    s.as_str(),
                    Some("Molet Warning"),
                    Some(flags),
                    Some(&self.icon),
                );
            }
            Err(MoletError::Info(s)) => {
                // 显示普通信息
                println!("Molet Info: {}", s);
                let flags =
                    nwg::TrayNotificationFlags::INFO_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
                self.tray.show(
                    s.as_str(),
                    Some("Molet Info"),
                    Some(flags),
                    Some(&self.icon),
                );
            }
            Err(e) => {
                // 显示存储失败信息
                println!("Failed to store data: {}", e);
                let flags =
                    nwg::TrayNotificationFlags::ERROR_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
                self.tray.show(
                    e.to_string().as_str(),
                    Some("Failed to store data"),
                    Some(flags),
                    Some(&self.icon),
                );
            }
        };
    }

    pub fn show_clipboard_data(&self) {
        let _ui = DataTable::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    }

    pub fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}
