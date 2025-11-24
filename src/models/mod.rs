pub mod action;
pub mod reality;
pub mod user;
pub mod vision;

pub use action::ActionLog;
pub use reality::RealityAssessment;
pub use user::User;
pub use vision::{Vision, VisionState};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Database {
    pub visions: Vec<Vision>,
    pub realities: Vec<RealityAssessment>,
    pub actions: Vec<ActionLog>,
    pub user: Option<User>,
}
