use crate::common::error::MoletError;
use crate::data::wrapper::insert_one;
use crate::data::ContentType;
use crate::data::StagingData;
use crate::Config;
use chrono::Local;
use clipboard_win::{formats, get_clipboard, SysResult};

const TITLE_STRING_LEN: usize = 12;
pub struct DataOp {}

impl DataOp {
    pub fn clipboard_to_db(config: &Config) -> Result<String, MoletError> {
        let result: SysResult<String> = get_clipboard(formats::Unicode);
        let data = match result {
            Ok(data) => {
                println!("ok = {}", data);
                data
            }
            Err(err) => {
                println!("err = {}", err.message());
                String::new()
            }
        };
        if data.is_empty() {
            println!("data is empty");
            return Err(MoletError::Info("Clipboard is empty".into()));
        }
        let mut title_len = TITLE_STRING_LEN;
        if data.len() < TITLE_STRING_LEN {
            title_len = data.len()
        }
        let staging_data = StagingData {
            id: None,
            content_type: ContentType::Unicode,
            creation_time: Local::now().timestamp(),
            title: String::from(&data[..title_len]),
            content: Some(Vec::from(data.as_bytes())),
        };
        match insert_one(config, &staging_data) {
            Ok(_) => Ok(staging_data.title),
            Err(e) => Err(MoletError::IOError(e.to_string())),
        }
    }
}
