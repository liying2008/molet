pub mod common;
pub mod conf;
pub mod data;
pub mod op;
pub mod ui;

use chrono::Local;
use data::StagingData;

use crate::conf::Config;
use crate::data::{wrapper, DB};
use rusqlite::Result;
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

pub fn get_all_data() -> Vec<StagingData> {
    let time1 = Local::now();
    let config = Config::load_conf().unwrap();
    let r = wrapper::get_all(&config);
    let time2 = Local::now();
    println!("load data from db time: {}", time2 - time1);
    match r {
        Ok(data) => {
            // let s = serde_json::to_string(&data).unwrap();
            // let time3 = Local::now();
            // println!("serialize json time: {}", time3 - time2);
            // s
            data
        }
        Err(e) => {
            eprintln!("{}", e);
            vec![]
        }
    }
}
