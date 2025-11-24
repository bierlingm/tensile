use crate::{
    config,
    error::{TensileError, TensileResult},
    models::Database,
};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

pub struct RonStore;

impl RonStore {
    pub fn load() -> TensileResult<Database> {
        let db_path = config::db_path();

        if !db_path.exists() {
            return Ok(Database::default());
        }

        let mut file = fs::File::open(&db_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let db: Database =
            ron::from_str(&contents).map_err(|e| TensileError::Serialization(e.to_string()))?;

        Ok(db)
    }

    pub fn save(db: &Database) -> TensileResult<()> {
        config::ensure_db_dir()?;

        let db_path = config::db_path();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&db_path)?;

        let contents = ron::ser::to_string_pretty(db, Default::default())
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}
