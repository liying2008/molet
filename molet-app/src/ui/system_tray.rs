extern crate native_windows_gui as nwg;

use crate::op::data_op::DataOp;
use crate::ui::data_table::DataTable;
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
        // nwg::modal_info_message(&self.window, "Hello", "Hello World!");
        DataOp::clipboard_to_db();
        // 显示存储成功的通知
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show(
            "Hello World",
            Some("Welcome to my application"),
            Some(flags),
            Some(&self.icon),
        );
    }

    pub fn show_clipboard_data(&self) {
        let _ui = DataTable::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    }

    pub fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}
