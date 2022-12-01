use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::ToSql;
use std::fmt::{Display, Formatter};

#[derive(Debug, serde::Serialize)]
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

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        match content_type {
            content_type if content_type == "Unicode" => ContentType::Unicode,
            content_type if content_type == "Bitmap" => ContentType::Bitmap,
            content_type if content_type == "FileList" => ContentType::FileList,
            content_type if content_type == "RawData" => ContentType::RawData,
            _ => ContentType::RawData,
        }
    }
}

impl From<ContentType> for &str {
    fn from(content_type: ContentType) -> Self {
        match content_type {
            ContentType::Unicode => "Unicode",
            ContentType::Bitmap => "Bitmap",
            ContentType::FileList => "FileList",
            ContentType::RawData => "RawData",
        }
    }
}

impl FromSql for ContentType {
    #[inline]
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().map(Into::into)
    }
}

impl ToSql for ContentType {
    #[inline]
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

#[derive(Debug, serde::Serialize)]
pub struct StagingData {
    pub id: Option<u32>,
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(rename = "creationTime")]
    pub creation_time: i64,
    pub title: String,
    pub content: Option<Vec<u8>>,
}
