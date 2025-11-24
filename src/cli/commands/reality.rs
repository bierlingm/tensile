use crate::{
    error::{TensileError, TensileResult},
    models::RealityAssessment,
    persistence,
};
use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum RealityCommands {
    /// Update current reality assessment
    #[command(visible_alias = "u")]
    Update {
        vision_id: String,
        #[arg(trailing_var_arg = true)]
        entry: Vec<String>,
    },

    /// View latest reality assessments
    #[command(visible_alias = "v")]
    View {
        #[arg(long)]
        vision: Option<String>,
    },

    /// Show most recent assessment
    #[command(visible_alias = "l")]
    Latest {
        #[arg(long)]
        vision: Option<String>,
    },
}

impl RealityCommands {
    pub fn execute(self) -> TensileResult<()> {
        match self {
            RealityCommands::Update { vision_id, entry } => update_reality(vision_id, entry),
            RealityCommands::View { vision } => view_reality(vision),
            RealityCommands::Latest { vision } => latest_reality(vision),
        }
    }
}

fn update_reality(vision_id: String, entry: Vec<String>) -> TensileResult<()> {
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

    let assessment = RealityAssessment::new(uuid, entry.join(" "));
    db.realities.push(assessment);
    persistence::save_database(&db)?;
    println!("✓ Reality assessment recorded");
    Ok(())
}

fn view_reality(vision_id: Option<String>) -> TensileResult<()> {
    let db = persistence::load_database()?;

    let filter_uuid = if let Some(id) = vision_id {
        Some(
            Uuid::parse_str(&id)
                .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", id)))?,
        )
    } else {
        None
    };

    println!("\nReality Assessments:");
    for assessment in &db.realities {
        if filter_uuid.is_none() || filter_uuid == Some(assessment.vision_id) {
            println!(
                "  [{}] {}",
                assessment.timestamp.format("%Y-%m-%d %H:%M"),
                assessment.entry
            );
        }
    }

    Ok(())
}

fn latest_reality(vision_id: Option<String>) -> TensileResult<()> {
    let db = persistence::load_database()?;

    let filter_uuid = if let Some(id) = vision_id {
        Some(
            Uuid::parse_str(&id)
                .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", id)))?,
        )
    } else {
        None
    };

    let mut assessments: Vec<_> = db
        .realities
        .iter()
        .filter(|a| filter_uuid.is_none() || filter_uuid == Some(a.vision_id))
        .collect();

    assessments.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    if let Some(latest) = assessments.first() {
        println!("\nLatest Assessment:");
        println!(
            "  [{}] {}",
            latest.timestamp.format("%Y-%m-%d %H:%M"),
            latest.entry
        );
    } else {
        println!("No reality assessments found");
    }

    Ok(())
}
