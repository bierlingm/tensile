use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionLog {
    pub id: Uuid,
    pub vision_id: Uuid,
    pub entry: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
}

impl ActionLog {
    pub fn new(vision_id: Uuid, entry: String) -> Self {
        ActionLog {
            id: Uuid::new_v4(),
            vision_id,
            entry,
            timestamp: Utc::now(),
            success: true,
        }
    }
}
