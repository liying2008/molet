extern crate native_windows_gui as nwg;

use crate::op::data_op::DataOp;

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
        // nwg::modal_info_message(&self.window, "Hello", "Hello World!");
        DataOp::clipboard_to_db()
    }

    pub fn show_clipboard_data(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show(
            "Hello World",
            Some("Welcome to my application"),
            Some(flags),
            Some(&self.icon),
        );
    }

    pub fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}
