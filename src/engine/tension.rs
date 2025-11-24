use crate::models::Database;
use uuid::Uuid;

#[allow(dead_code)]
pub struct TensionCalculator;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VisionTension {
    pub vision_id: Uuid,
    pub tension_score: f32,
    pub action_count: usize,
    pub reality_count: usize,
}

impl TensionCalculator {
    #[allow(dead_code)]
    pub fn calculate_vision_tension(db: &Database, vision_id: Uuid) -> Option<VisionTension> {
        db.visions.iter().find(|v| v.id == vision_id)?;

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

        let gap_factor = if reality_count == 0 {
            1.0
        } else {
            1.0 - (action_count as f32 / (reality_count + action_count + 1) as f32)
        };

        let tension_score = gap_factor * 100.0;

        Some(VisionTension {
            vision_id,
            tension_score,
            action_count,
            reality_count,
        })
    }

    #[allow(dead_code)]
    pub fn calculate_all_tensions(db: &Database) -> Vec<VisionTension> {
        db.visions
            .iter()
            .filter_map(|v| Self::calculate_vision_tension(db, v.id))
            .collect()
    }

    #[allow(dead_code)]
    pub fn sort_by_tension(mut tensions: Vec<VisionTension>) -> Vec<VisionTension> {
        tensions.sort_by(|a, b| b.tension_score.partial_cmp(&a.tension_score).unwrap());
        tensions
    }
}
