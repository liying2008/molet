use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ContentType {
    Text,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Text => write!(f, "Text"),
        }
    }
}

impl From<String> for ContentType {
    fn from(content_type: String) -> Self {
        match content_type {
            content_type if content_type == "Text" => ContentType::Text,
            _ => ContentType::Text,
        }
    }
}

impl From<ContentType> for String {
    fn from(content_type: ContentType) -> Self {
        match content_type {
            ContentType::Text => "Text".into(),
        }
    }
}

impl FromSql for ContentType {
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
