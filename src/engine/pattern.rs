use crate::models::{ActionLog, Database};
use uuid::Uuid;

pub struct PatternAnalyzer;

#[derive(Debug, Clone)]
pub enum Pattern {
    Advancing,
    Oscillating,
    Stagnant,
}

#[derive(Debug, Clone)]
pub struct PatternMetrics {
    #[allow(dead_code)]
    pub vision_id: Uuid,
    pub pattern: Pattern,
    pub success_rate: f32,
    pub total_actions: usize,
    pub recent_actions: usize,
    pub velocity: f32,
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

    pub fn get_detailed_metrics(db: &Database, vision_id: Uuid) -> Option<PatternMetrics> {
        let vision = db.visions.iter().find(|v| v.id == vision_id)?;

        let all_actions: Vec<&ActionLog> = db
            .actions
            .iter()
            .filter(|a| a.vision_id == vision_id)
            .collect();

        if all_actions.is_empty() {
            return Some(PatternMetrics {
                vision_id,
                pattern: Pattern::Stagnant,
                success_rate: 0.0,
                total_actions: 0,
                recent_actions: 0,
                velocity: 0.0,
            });
        }

        let now = chrono::Utc::now();
        let seven_days_ago = now - chrono::Duration::days(7);

        let recent_actions = all_actions
            .iter()
            .filter(|a| a.timestamp > seven_days_ago)
            .count();
        let days_active = now
            .signed_duration_since(vision.created_at)
            .num_days()
            .max(1);
        let velocity = all_actions.len() as f32 / days_active as f32;

        let success_rate =
            all_actions.iter().filter(|a| a.success).count() as f32 / all_actions.len() as f32;

        Some(PatternMetrics {
            vision_id,
            pattern: Self::analyze_vision_pattern(db, vision_id),
            success_rate,
            total_actions: all_actions.len(),
            recent_actions,
            velocity,
        })
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
