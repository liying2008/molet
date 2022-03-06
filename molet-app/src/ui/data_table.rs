extern crate native_windows_gui as nwg;

use nwg::NativeUi;

#[derive(Default)]
pub struct DataTable {
    pub window: nwg::Window,
    pub name_edit: nwg::TextInput,
    pub hello_button: nwg::Button,
}

impl DataTable {
    pub(crate) fn say_hello(&self) {
        nwg::modal_info_message(
            &self.window,
            "Hello",
            &format!("Hello {}", self.name_edit.text()),
        );
    }

    pub(crate) fn say_goodbye(&self) {
        nwg::modal_info_message(
            &self.window,
            "Goodbye",
            &format!("Goodbye {}", self.name_edit.text()),
        );
        nwg::stop_thread_dispatch();
    }
}
