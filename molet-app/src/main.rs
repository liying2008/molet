#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;

use molet_app::conf::Config;
use molet_app::data::db::DB;
use molet_app::SystemTray;
use nwg::NativeUi;
use std::process;

fn main() {
    // 初始化配置文件
    if let Err(err) = Config::init_conf() {
        println!("Error occurred: {}", err);
        process::exit(1);
    };

    match Config::load_conf() {
        Ok(config) => {
            println!("{:?}", config)
        }
        Err(e) => {
            println!("Error occurred: {}", e);
            process::exit(1);
        }
    }

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
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
