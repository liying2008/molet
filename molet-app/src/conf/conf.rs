use std::error::Error;
use std::fs;
use std::path::PathBuf;

const APP_VENDOR: &str = "liying2008";
const APP_FOLDER_NAME: &str = "molet";
const CONFIG_FILENAME: &str = "config.json";

pub struct Config {
    pub db_path: String,
}

impl Config {
    pub fn init_conf() -> Result<(), &'static str> {
        let mut config_path = PathBuf::from(env!("APPDATA"));
        if !config_path.exists() {
            return Err("APPDATA env or path does not exist.");
        }
        config_path.push(APP_VENDOR);
        config_path.push(APP_FOLDER_NAME);
        if !config_path.exists() {
            if let Err(e) = fs::create_dir_all(&config_path) {
                println!("{}", e);
                return Err("Create app data path failed.");
            }
        }
        println!("config_path={:?}", &config_path);
        config_path.push(CONFIG_FILENAME);
        if !config_path.exists() {
            if let Err(e) = fs::write(&config_path, "{}") {
                println!("{}", e);
                return Err("Write initialization configuration failed.");
            }
        }
        return Ok(());
    }
}
