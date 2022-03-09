use crate::common::error::MoletError;
use crate::common::error::MoletError::{AppError, EnvError, IOError, SystemError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const APP_VENDOR: &str = "liying2008";
const APP_FOLDER_NAME: &str = "molet";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db_path: String,
}

impl Config {
    pub fn init_conf() -> Result<(), MoletError> {
        let app_data_env = env!("APPDATA");
        if app_data_env.is_empty() {
            return Err(EnvError(String::from(
                "APPDATA environment variable not set.",
            )));
        }
        let mut config_path = PathBuf::from(app_data_env);
        if !config_path.exists() {
            return Err(SystemError(String::from("APPDATA path does not exist.")));
        }
        config_path.push(APP_VENDOR);
        config_path.push(APP_FOLDER_NAME);
        if !config_path.exists() {
            if let Err(e) = fs::create_dir_all(&config_path) {
                println!("{}", e);
                return Err(IOError(String::from("Create app data path failed.")));
            }
        }
        println!("config_path={:?}", &config_path);
        config_path.push(CONFIG_FILENAME);
        if !config_path.exists() {
            let config = Config {
                db_path: String::new(),
            };
            let config_json = serde_json::to_string(&config).unwrap();
            if let Err(e) = fs::write(&config_path, config_json) {
                eprintln!("{}", e);
                return Err(IOError(String::from(
                    "Write initialization configuration failed.",
                )));
            }
        }
        Ok(())
    }

    pub fn load_conf() -> Result<Config, MoletError> {
        let app_data_env = env!("APPDATA");
        if app_data_env.is_empty() {
            return Err(EnvError(String::from(
                "APPDATA environment variable not set.",
            )));
        }
        let mut config_file_path = PathBuf::from(app_data_env);
        config_file_path.push(APP_VENDOR);
        config_file_path.push(APP_FOLDER_NAME);
        config_file_path.push(CONFIG_FILENAME);
        match fs::read_to_string(config_file_path) {
            Ok(s) => match serde_json::from_str(&s) {
                Ok(config) => Ok(config),
                Err(e) => Err(AppError(e.to_string())),
            },
            Err(e) => {
                eprintln!("{}", e);
                Err(IOError(String::from("Read configuration file failed.")))
            }
        }
    }
}
