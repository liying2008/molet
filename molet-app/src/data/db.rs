use crate::data::db_data::ContentType::Text;
use crate::data::db_data::StagingData;
use rusqlite::{params, Connection, Result};

pub struct DB {}

impl DB {
    pub fn init_db() -> Result<()> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            "CREATE TABLE staging_data (
                  id              INTEGER PRIMARY KEY AUTOINCREMENT,
                  content_type            TEXT NOT NULL,
                  creation_time            INTEGER NOT NULL,
                  content            BLOB
                  )",
            [],
        )?;
        let data = StagingData {
            id: 0,
            content_type: "Text".to_string(),
            creation_time: 12345678,
            content: Some(Vec::from("TEST DATA".as_bytes())),
        };
        conn.execute(
            "INSERT INTO staging_data (content_type, creation_time, content) VALUES (?1, ?2, ?3)",
            params![data.content_type, data.creation_time, data.content],
        )?;

        let mut stmt =
            conn.prepare("SELECT id, content_type, creation_time, content FROM staging_data")?;
        let data_iter = stmt.query_map([], |row| {
            Ok(StagingData {
                id: row.get(0)?,
                content_type: row.get(1)?,
                creation_time: row.get(2)?,
                content: row.get(3)?,
            })
        })?;

        for data in data_iter {
            let data = data.unwrap();
            println!("Found data {:?}", data);
            println!(
                "content is: {}",
                String::from_utf8(data.content.unwrap()).unwrap()
            )
        }
        Ok(())
    }
}
