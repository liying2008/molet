pub mod common;
pub mod conf;
pub mod data;
pub mod op;
pub mod ui;


use crate::conf::Config;
use crate::data::DB;
use std::process;


pub fn load_config() -> Config {
    // 初始化配置文件
    if let Err(err) = Config::init_conf() {
        println!("Error occurred: {}", err);
        process::exit(1);
    };

    let config = match Config::load_conf() {
        Ok(config) => config,
        Err(e) => {
            println!("Error occurred: {}", e);
            process::exit(1);
        }
    };
    println!("{:?}", config);
    config
}

pub fn init_db(config: &Config) {
    let db = DB::connect(config).unwrap_or_else(|e| {
        println!("Error occurred: Database connection failed: {}", e);
        process::exit(1);
    });
    db.init_db().unwrap_or_else(|e| {
        println!("Error occurred: Database initialization failed: {}", e);
        process::exit(1);
    });
    db.close().unwrap_or_else(|e| {
        println!("Error occurred: Database close failed: {}", e);
        process::exit(1);
    });
}
