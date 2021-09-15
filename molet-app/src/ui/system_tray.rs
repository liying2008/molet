extern crate native_windows_gui as nwg;

#[derive(Default)]
pub struct SystemTray {
    pub window: nwg::MessageWindow,
    pub icon: nwg::Icon,
    pub tray: nwg::TrayNotification,
    pub tray_menu: nwg::Menu,
    pub tray_item1: nwg::MenuItem,
    pub tray_item2: nwg::MenuItem,
    pub tray_item3: nwg::MenuItem,
}

impl SystemTray {
    pub fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    pub fn hello1(&self) {
        nwg::modal_info_message(&self.window, "Hello", "Hello World!");
    }

    pub fn hello2(&self) {
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
