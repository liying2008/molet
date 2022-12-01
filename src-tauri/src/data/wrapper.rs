use crate::data::StagingData;
use crate::{Config, DB};
use rusqlite::Result;

pub fn insert_one(config: &Config, data: &StagingData) -> Result<usize> {
    let db = DB::connect(config)?;
    let result = db.insert_one(data)?;
    db.close()?;
    Ok(result)
}

pub fn get_all(config: &Config) -> Result<Vec<StagingData>> {
    let db = DB::connect(config)?;
    let result = db.get_all()?;
    db.close()?;
    Ok(result)
}
