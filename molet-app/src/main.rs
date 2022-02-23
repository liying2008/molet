extern crate native_windows_gui as nwg;

use molet_app::conf::conf::Config;
use molet_app::data::db::DB;
use molet_app::SystemTray;
use nwg::NativeUi;
use std::process;

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

    // 初始化配置文件
    Config::init_conf().unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        process::exit(1);
    });
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
