use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RealityAssessment {
    pub id: Uuid,
    pub vision_id: Uuid,
    pub entry: String,
    pub timestamp: DateTime<Utc>,
}

impl RealityAssessment {
    pub fn new(vision_id: Uuid, entry: String) -> Self {
        RealityAssessment {
            id: Uuid::new_v4(),
            vision_id,
            entry,
            timestamp: Utc::now(),
        }
    }
}
