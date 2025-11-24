use crate::{
    error::{TensileError, TensileResult},
    models::ActionLog,
    persistence,
};
use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum ActionCommands {
    /// Log an action toward a vision
    #[command(visible_alias = "l")]
    Log {
        vision_id: String,
        #[arg(trailing_var_arg = true)]
        entry: Vec<String>,
    },

    /// Prompt for today's action
    #[command(visible_alias = "t")]
    Today { vision_id: String },

    /// Review actions by period
    #[command(visible_alias = "r")]
    Review {
        #[arg(long, default_value = "daily")]
        period: String,
        #[arg(long)]
        vision: Option<String>,
    },
}

impl ActionCommands {
    pub fn execute(self) -> TensileResult<()> {
        match self {
            ActionCommands::Log { vision_id, entry } => log_action(vision_id, entry),
            ActionCommands::Today { vision_id } => today_action(vision_id),
            ActionCommands::Review { period, vision } => review_actions(period, vision),
        }
    }
}

fn log_action(vision_id: String, entry: Vec<String>) -> TensileResult<()> {
    let uuid = Uuid::parse_str(&vision_id)
        .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", vision_id)))?;

    let mut db = persistence::load_database()?;

    let vision_exists = db.visions.iter().any(|v| v.id == uuid);
    if !vision_exists {
        return Err(TensileError::NotFound(format!(
            "Vision not found: {}",
            vision_id
        )));
    }

    let action = ActionLog::new(uuid, entry.join(" "));
    db.actions.push(action);
    persistence::save_database(&db)?;
    println!("✓ Action logged");
    Ok(())
}

fn today_action(vision_id: String) -> TensileResult<()> {
    let uuid = Uuid::parse_str(&vision_id)
        .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", vision_id)))?;

    let db = persistence::load_database()?;

    let vision = db
        .visions
        .iter()
        .find(|v| v.id == uuid)
        .ok_or_else(|| TensileError::NotFound(format!("Vision not found: {}", vision_id)))?;

    println!("\nToday's Action for: {}", vision.title);
    println!("What is your next step toward this vision?");
    println!("(Run: tensile action log {} <your action>)", vision_id);

    Ok(())
}

fn review_actions(period: String, vision_id: Option<String>) -> TensileResult<()> {
    let db = persistence::load_database()?;

    let filter_uuid = if let Some(id) = vision_id {
        Some(
            Uuid::parse_str(&id)
                .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", id)))?,
        )
    } else {
        None
    };

    let now = chrono::Utc::now();
    let cutoff = match period.as_str() {
        "daily" => now - chrono::Duration::days(1),
        "weekly" => now - chrono::Duration::days(7),
        "monthly" => now - chrono::Duration::days(30),
        _ => now - chrono::Duration::days(1),
    };

    let mut actions: Vec<_> = db
        .actions
        .iter()
        .filter(|a| {
            (filter_uuid.is_none() || filter_uuid == Some(a.vision_id)) && a.timestamp > cutoff
        })
        .collect();

    actions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    println!("\nActions ({}):", period);
    let success_count = actions.iter().filter(|a| a.success).count();
    println!("  Total: {} | Successful: {}", actions.len(), success_count);

    for action in actions {
        let marker = if action.success { "✓" } else { "✗" };
        println!(
            "  {} [{}] {}",
            marker,
            action.timestamp.format("%Y-%m-%d %H:%M"),
            action.entry
        );
    }

    Ok(())
}
