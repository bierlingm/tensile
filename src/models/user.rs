use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub current_focus: Option<Uuid>,
    pub last_reviewed: Option<DateTime<Utc>>,
}

impl User {
    pub fn new() -> Self {
        User {
            current_focus: None,
            last_reviewed: None,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User::new()
    }
}
