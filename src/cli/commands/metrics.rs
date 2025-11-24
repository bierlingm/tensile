use clap::Subcommand;
use uuid::Uuid;

use crate::{
    engine::{PatternAnalyzer, TensionCalculator},
    error::{TensileError, TensileResult},
    persistence,
};

#[derive(Subcommand)]
pub enum MetricsCommands {
    /// Show priority visions (sorted by tension)
    #[command(visible_alias = "p")]
    Priority,

    /// Show detailed metrics for a vision
    #[command(visible_alias = "d")]
    Detail { vision_id: String },

    /// Show summary dashboard
    #[command(visible_alias = "s")]
    Summary,
}

impl MetricsCommands {
    pub fn execute(self) -> TensileResult<()> {
        match self {
            MetricsCommands::Priority => show_priorities(),
            MetricsCommands::Detail { vision_id } => show_detail(vision_id),
            MetricsCommands::Summary => show_summary(),
        }
    }
}

fn show_priorities() -> TensileResult<()> {
    let db = persistence::load_database()?;

    let tensions = TensionCalculator::calculate_all_tensions(&db);
    let sorted = TensionCalculator::sort_by_tension(tensions);

    println!("\n📊 Priority Visions (by Tension):");
    println!("{:-^60}", "Rank | Vision | Tension | Actions");

    for (rank, tension) in sorted.iter().enumerate() {
        let priority_icon = match rank {
            0 => "🔴",
            1 => "🟠",
            2 => "🟡",
            _ => "  ",
        };

        println!(
            "{} {} | {} | {:.0}% | {}",
            priority_icon,
            rank + 1,
            truncate(&tension.vision_title, 25),
            tension.tension_score,
            tension.action_count
        );
    }

    Ok(())
}

fn show_detail(vision_id: String) -> TensileResult<()> {
    let uuid = Uuid::parse_str(&vision_id)
        .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", vision_id)))?;

    let db = persistence::load_database()?;

    let vision = db
        .visions
        .iter()
        .find(|v| v.id == uuid)
        .ok_or_else(|| TensileError::NotFound(format!("Vision not found: {}", vision_id)))?;

    if let Some(tension) = TensionCalculator::calculate_vision_tension(&db, uuid) {
        if let Some(metrics) = PatternAnalyzer::get_detailed_metrics(&db, uuid) {
            println!("\n📈 Detailed Metrics: {}", vision.title);
            println!("{:─<50}", "");
            println!("  State: {}", vision.state);
            println!("  Tension Score: {:.1}%", tension.tension_score);
            println!("  Days Active: {}", tension.days_active);
            println!();
            println!("  Pattern: {:?}", metrics.pattern);
            println!("  Success Rate: {:.1}%", metrics.success_rate * 100.0);
            println!("  Total Actions: {}", metrics.total_actions);
            println!("  Recent (7 days): {}", metrics.recent_actions);
            println!("  Velocity: {:.2} actions/day", metrics.velocity);
            println!();

            if let Some(desc) = &vision.description {
                println!("  Description: {}", desc);
            }
        }
    }

    Ok(())
}

fn show_summary() -> TensileResult<()> {
    let db = persistence::load_database()?;

    let total_visions = db.visions.len();
    let active_visions = db
        .visions
        .iter()
        .filter(|v| !v.completed && matches!(v.state, crate::models::VisionState::InProgress))
        .count();
    let total_actions = db.actions.len();
    let total_reality = db.realities.len();

    let avg_tension = {
        let tensions = TensionCalculator::calculate_all_tensions(&db);
        if tensions.is_empty() {
            0.0
        } else {
            tensions.iter().map(|t| t.tension_score).sum::<f32>() / tensions.len() as f32
        }
    };

    println!("\n📊 Summary Dashboard");
    println!("{:─<50}", "");
    println!("  Total Visions: {}", total_visions);
    println!("  Active Visions: {}", active_visions);
    println!("  Total Actions Logged: {}", total_actions);
    println!("  Total Reality Assessments: {}", total_reality);
    println!("  Average Tension: {:.1}%", avg_tension);

    if let Some(priority) = TensionCalculator::get_priority_vision(&db) {
        println!("\n  🎯 Top Priority: {}", priority.vision_title);
        println!("     Tension: {:.1}%", priority.tension_score);
    }

    Ok(())
}

fn truncate(s: &str, len: usize) -> String {
    if s.len() > len {
        format!("{}...", &s[..len - 3])
    } else {
        s.to_string()
    }
}
