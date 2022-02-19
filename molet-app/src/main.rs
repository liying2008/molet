extern crate native_windows_gui as nwg;

use molet_app::data::db::DB;
use molet_app::SystemTray;
use nwg::NativeUi;

fn main() {
    let r = DB::init_db();
    match r {
        Ok(r) => {
            println!("{:?}", r)
        }
        Err(err) => {
            eprintln!("{:?}", err)
        }
    }

    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
