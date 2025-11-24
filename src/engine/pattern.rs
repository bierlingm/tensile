use crate::models::{ActionLog, Database};
use uuid::Uuid;

pub struct PatternAnalyzer;

#[derive(Debug, Clone)]
pub enum Pattern {
    Advancing,
    Oscillating,
    Stagnant,
}

impl PatternAnalyzer {
    pub fn analyze_vision_pattern(db: &Database, vision_id: Uuid) -> Pattern {
        let actions: Vec<&ActionLog> = db
            .actions
            .iter()
            .filter(|a| a.vision_id == vision_id)
            .collect();

        if actions.is_empty() {
            return Pattern::Stagnant;
        }

        let success_rate: f32 =
            actions.iter().filter(|a| a.success).count() as f32 / actions.len() as f32;

        if success_rate > 0.7 {
            Pattern::Advancing
        } else if success_rate > 0.3 {
            Pattern::Oscillating
        } else {
            Pattern::Stagnant
        }
    }

    pub fn analyze_all_patterns(db: &Database) -> Vec<(Uuid, Pattern)> {
        db.visions
            .iter()
            .map(|v| (v.id, Self::analyze_vision_pattern(db, v.id)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ActionLog;
    use uuid::Uuid;

    #[test]
    fn test_advancing_pattern() {
        let mut db = Database::default();
        let vision_id = Uuid::new_v4();

        for _ in 0..8 {
            db.actions
                .push(ActionLog::new(vision_id, "action".to_string()));
        }

        for _ in 0..2 {
            let mut action = ActionLog::new(vision_id, "failed".to_string());
            action.success = false;
            db.actions.push(action);
        }

        let pattern = PatternAnalyzer::analyze_vision_pattern(&db, vision_id);
        assert!(matches!(pattern, Pattern::Advancing));
    }
}
