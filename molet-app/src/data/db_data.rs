use rusqlite::types::ToSqlOutput;
use rusqlite::ToSql;

pub enum ContentType {
    Text,
}

impl ToSql for ContentType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct StagingData {
    pub id: u32,
    pub content_type: String,
    pub creation_time: u32,
    pub title: String,
    pub content: Option<Vec<u8>>,
}
