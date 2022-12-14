use crate::common::error::MoletError;
use crate::data::wrapper::insert_one;
use crate::data::ContentType;
use crate::data::StagingData;
use crate::Config;
use arboard::Clipboard;
use chrono::Local;

const TITLE_STRING_LEN: usize = 12;
pub struct DataOp {}

impl DataOp {
    fn get_clipboard_unicode() -> String {
        let mut clipboard = Clipboard::new().unwrap();
        let result = clipboard.get_text();
        match result {
            Ok(data) => {
                println!("get_clipboard_unicode::ok={}", data);
                data
            }
            Err(err) => {
                println!("get_clipboard_unicode::err={}", err.to_string());
                String::new()
            }
        }
    }

    fn get_clipboard_bitmap() -> Vec<u8> {
        let mut clipboard = Clipboard::new().unwrap();
        let result = clipboard.get_image();

        match result {
            Ok(data) => {
                println!("get_clipboard_bitmap::ok={}", data.bytes.len());
                // TODO get bitmap bytes
                data.bytes.to_vec()
            }
            Err(err) => {
                println!("get_clipboard_bitmap::err={}", err.to_string());
                vec![]
            }
        }
    }

    fn store_unicode_to_db(config: &Config, data: String) -> Result<String, MoletError> {
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

    fn store_bitmap_to_db(config: &Config, data: Vec<u8>) -> Result<String, MoletError> {
        let staging_data = StagingData {
            id: None,
            content_type: ContentType::Bitmap,
            creation_time: Local::now().timestamp(),
            title: "Bitmap Data".into(),
            content: Some(data),
        };
        match insert_one(config, &staging_data) {
            Ok(_) => Ok(staging_data.title),
            Err(e) => Err(MoletError::IOError(e.to_string())),
        }
    }

    pub fn clipboard_to_db(config: &Config) -> Result<String, MoletError> {
        let data = DataOp::get_clipboard_unicode();
        if !data.is_empty() {
            // 存储文本类型到数据库
            return DataOp::store_unicode_to_db(config, data);
        }

        let data = DataOp::get_clipboard_bitmap();
        if !data.is_empty() {
            // 存储 Bitmap 类型到数据库
            return DataOp::store_bitmap_to_db(config, data);
        }

        Err(MoletError::Info(
            "Clipboard is empty or data type is not supported.".into(),
        ))
    }

    pub fn data_to_clipboard(config: &Config) {}
}
