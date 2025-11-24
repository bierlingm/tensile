use crate::models::{Database, VisionState};
use uuid::Uuid;

pub struct TensionCalculator;

#[derive(Debug, Clone)]
pub struct VisionTension {
    #[allow(dead_code)]
    pub vision_id: Uuid,
    pub vision_title: String,
    pub tension_score: f32,
    pub action_count: usize,
    #[allow(dead_code)]
    pub reality_count: usize,
    #[allow(dead_code)]
    pub state: VisionState,
    pub days_active: u64,
}

impl TensionCalculator {
    pub fn calculate_vision_tension(db: &Database, vision_id: Uuid) -> Option<VisionTension> {
        let vision = db.visions.iter().find(|v| v.id == vision_id)?;

        let action_count = db
            .actions
            .iter()
            .filter(|a| a.vision_id == vision_id)
            .count();

        let reality_count = db
            .realities
            .iter()
            .filter(|r| r.vision_id == vision_id)
            .count();

        let total_entries = action_count + reality_count;
        let gap_factor = if total_entries == 0 {
            1.0
        } else {
            1.0 - (action_count as f32 / (total_entries + 1) as f32)
        };
        let tension_score = if vision.completed {
            0.0
        } else {
            gap_factor * 100.0
        };

        let days_active = (chrono::Utc::now()
            .signed_duration_since(vision.created_at)
            .num_days() as u64)
            .max(1);

        Some(VisionTension {
            vision_id,
            vision_title: vision.title.clone(),
            tension_score,
            action_count,
            reality_count,
            state: vision.state.clone(),
            days_active,
        })
    }

    pub fn calculate_all_tensions(db: &Database) -> Vec<VisionTension> {
        db.visions
            .iter()
            .filter_map(|v| Self::calculate_vision_tension(db, v.id))
            .collect()
    }

    pub fn sort_by_tension(mut tensions: Vec<VisionTension>) -> Vec<VisionTension> {
        tensions.sort_by(|a, b| b.tension_score.partial_cmp(&a.tension_score).unwrap());
        tensions
    }

    pub fn get_priority_vision(db: &Database) -> Option<VisionTension> {
        Self::sort_by_tension(Self::calculate_all_tensions(db))
            .into_iter()
            .next()
    }
}
