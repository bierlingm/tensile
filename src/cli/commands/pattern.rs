use crate::{
    engine::PatternAnalyzer,
    error::{TensileError, TensileResult},
    persistence,
};
use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum PatternCommands {
    /// Analyze patterns in actions (advancing vs oscillating)
    #[command(visible_alias = "c")]
    Check {
        #[arg(long)]
        vision: Option<String>,
    },
}

impl PatternCommands {
    pub fn execute(self) -> TensileResult<()> {
        match self {
            PatternCommands::Check { vision } => check_patterns(vision),
        }
    }
}

fn check_patterns(vision_id: Option<String>) -> TensileResult<()> {
    let db = persistence::load_database()?;

    if let Some(id) = vision_id {
        let uuid = Uuid::parse_str(&id)
            .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", id)))?;

        let vision = db
            .visions
            .iter()
            .find(|v| v.id == uuid)
            .ok_or_else(|| TensileError::NotFound(format!("Vision not found: {}", id)))?;

        let pattern = PatternAnalyzer::analyze_vision_pattern(&db, uuid);
        println!("\nPattern Analysis for: {}", vision.title);
        println!("  Pattern: {:?}", pattern);

        let actions: Vec<_> = db.actions.iter().filter(|a| a.vision_id == uuid).collect();

        if !actions.is_empty() {
            let success_rate =
                actions.iter().filter(|a| a.success).count() as f32 / actions.len() as f32;
            println!("  Success Rate: {:.1}%", success_rate * 100.0);
            println!("  Total Actions: {}", actions.len());
        }
    } else {
        println!("\nPattern Analysis - All Visions:");
        let patterns = PatternAnalyzer::analyze_all_patterns(&db);

        for (vision_id, pattern) in patterns {
            if let Some(vision) = db.visions.iter().find(|v| v.id == vision_id) {
                println!("  {} - {:?}", vision.title, pattern);
            }
        }
    }

    Ok(())
}
