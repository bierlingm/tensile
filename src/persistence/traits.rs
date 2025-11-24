use crate::{error::TensileResult, models::Database};

#[allow(dead_code)]
pub trait PersistenceBackend {
    fn load() -> TensileResult<Database>;
    fn save(db: &Database) -> TensileResult<()>;
}
