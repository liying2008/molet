use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ContentType {
    Unicode,
    Bitmap,
    FileList,
    RawData,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Unicode => write!(f, "Unicode"),
            ContentType::Bitmap => write!(f, "Bitmap"),
            ContentType::FileList => write!(f, "FileList"),
            ContentType::RawData => write!(f, "RawData"),
        }
    }
}

impl From<String> for ContentType {
    fn from(content_type: String) -> Self {
        match content_type {
            content_type if content_type == "Unicode" => ContentType::Unicode,
            content_type if content_type == "Bitmap" => ContentType::Bitmap,
            content_type if content_type == "FileList" => ContentType::FileList,
            content_type if content_type == "RawData" => ContentType::RawData,
            _ => ContentType::RawData,
        }
    }
}

impl From<ContentType> for String {
    fn from(content_type: ContentType) -> Self {
        match content_type {
            ContentType::Unicode => "Unicode".into(),
            ContentType::Bitmap => "Bitmap".into(),
            ContentType::FileList => "FileList".into(),
            ContentType::RawData => "RawData".into(),
        }
    }
}

impl FromSql for ContentType {
    #[inline]
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().map(ToString::to_string).map(Into::into)
    }
}

#[derive(Debug)]
pub struct StagingData {
    pub id: u32,
    pub content_type: ContentType,
    pub creation_time: u32,
    pub title: String,
    pub content: Option<Vec<u8>>,
}
