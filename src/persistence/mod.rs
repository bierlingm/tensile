pub mod sqlite_store;

pub use sqlite_store::SqliteStore;

use crate::{error::TensileResult, models::Database};

pub fn load_database() -> TensileResult<Database> {
    SqliteStore::load()
}

pub fn save_database(db: &Database) -> TensileResult<()> {
    SqliteStore::save(db)
}
