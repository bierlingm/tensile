pub mod ron_store;
pub mod traits;

pub use ron_store::RonStore;
pub use traits::PersistenceBackend;

use crate::{error::TensileResult, models::Database};

pub fn load_database() -> TensileResult<Database> {
    RonStore::load()
}

pub fn save_database(db: &Database) -> TensileResult<()> {
    RonStore::save(db)
}
