extern crate native_windows_gui as nwg;

use molet_app::SystemTray;
use nwg::NativeUi;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
