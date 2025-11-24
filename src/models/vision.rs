use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum VisionState {
    Conceived,
    InProgress,
    Blocked,
    Reassessed,
    Achieved,
}

impl std::str::FromStr for VisionState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "conceived" => Ok(VisionState::Conceived),
            "inprogress" | "in_progress" => Ok(VisionState::InProgress),
            "blocked" => Ok(VisionState::Blocked),
            "reassessed" => Ok(VisionState::Reassessed),
            "achieved" => Ok(VisionState::Achieved),
            _ => Err(format!("Unknown state: {}", s)),
        }
    }
}

impl std::fmt::Display for VisionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisionState::Conceived => write!(f, "Conceived"),
            VisionState::InProgress => write!(f, "InProgress"),
            VisionState::Blocked => write!(f, "Blocked"),
            VisionState::Reassessed => write!(f, "Reassessed"),
            VisionState::Achieved => write!(f, "Achieved"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vision {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
    pub state: VisionState,
}

impl Vision {
    pub fn new(title: String) -> Self {
        Vision {
            id: Uuid::new_v4(),
            title,
            description: None,
            parent: None,
            children: vec![],
            created_at: Utc::now(),
            completed: false,
            state: VisionState::Conceived,
        }
    }
}
