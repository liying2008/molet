use crate::data::db_data::StagingData;
use crate::Config;
use rusqlite::{params, Connection, Result};

pub const STAGING_DATA_TABLE_NAME: &str = "staging_data";

pub struct DB {
    pub conn: Connection,
}

impl DB {
    pub fn connect(config: &Config) -> Result<DB> {
        let db_path = config.db_path.clone();
        let conn = Connection::open(&db_path)?;
        Ok(DB { conn })
    }

    fn check_table_exists(&self, table_name: &str) -> bool {
        let sql = "SELECT COUNT(`name`) FROM `sqlite_master` WHERE `type` = 'table' AND `name` = ?";
        let mut stmt = self.conn.prepare(sql).unwrap();
        let rs = stmt.query_row(params![table_name], |row| row.get(0) as Result<i32>);

        let count = rs.unwrap();

        count > 0
    }

    fn create_table(&self, table_name: &str) -> Result<usize> {
        let sql = format!(
            r#"CREATE TABLE IF NOT EXISTS `{}` (
             `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 
             `content_type` TEXT NOT NULL,
             `creation_time` INTEGER NOT NULL,
             `title` TEXT NOT NULL,
             `content` BLOB
            )"#,
            table_name
        );
        self.conn.execute(sql.as_str(), params![])
    }

    pub fn init_db(&self) -> Result<()> {
        match self.create_table(STAGING_DATA_TABLE_NAME) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn insert_one(&self, data: &StagingData) -> Result<usize> {
        let sql = format!(
            "INSERT INTO {} (content_type, creation_time, title, content) VALUES (?1, ?2, ?3, ?4)",
            STAGING_DATA_TABLE_NAME
        );
        self.conn.execute(
            sql.as_str(),
            params![
                data.content_type,
                data.creation_time,
                data.title,
                data.content
            ],
        )
    }

    pub fn get_all(&self) -> Result<Vec<StagingData>> {
        let sql = format!(
            "SELECT id, content_type, creation_time, title, content FROM {}",
            STAGING_DATA_TABLE_NAME
        );
        let mut stmt = self.conn.prepare(sql.as_str())?;
        let data_iter = stmt.query_map([], |row| {
            Ok(StagingData {
                id: row.get(0)?,
                content_type: row.get(1)?,
                creation_time: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
            })
        })?;
        let mut data_vec = Vec::new();
        for data in data_iter {
            data_vec.push(data.unwrap())
        }
        Ok(data_vec)
    }

    pub fn delete_by_id(&self, id: u32) -> Result<usize> {
        let sql = format!("DELETE FROM {} WHERE id=?", STAGING_DATA_TABLE_NAME);
        self.conn.execute(sql.as_str(), params![id])
    }

    pub fn delete_all(&self) -> Result<usize> {
        let sql = format!("DELETE FROM {}", STAGING_DATA_TABLE_NAME);
        self.conn.execute(sql.as_str(), params![])
    }

    pub fn close(self) -> Result<()> {
        match self.conn.close() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::global::{DEFAULT_DB_EXT, DEFAULT_DB_NAME};
    use crate::data::{ContentType, StagingData};
    use crate::{Config, DB};
    use chrono::Local;

    #[test]
    fn test_db() {
        let config = Config {
            db_path: format!("./{}{}", DEFAULT_DB_NAME, DEFAULT_DB_EXT),
        };
        let db = DB::connect(&config).unwrap();
        db.init_db().unwrap();

        db.delete_all().unwrap();

        let data = StagingData {
            id: None,
            content_type: ContentType::Unicode,
            creation_time: Local::now().timestamp(),
            title: "标题".to_string(),
            content: Some(Vec::from("TEST DATA".as_bytes())),
        };

        db.insert_one(&data).unwrap();
        db.insert_one(&data).unwrap();
        let data_vec = db.get_all().unwrap();
        for data in data_vec {
            println!("data: {:?}", data)
        }

        db.close().unwrap()
    }
}
