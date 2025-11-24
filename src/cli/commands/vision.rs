use crate::{
    error::{TensileError, TensileResult},
    models::Vision,
    persistence,
};
use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum VisionCommands {
    /// Create a new vision
    #[command(visible_alias = "n")]
    New { title: String },

    /// View the vision tree
    #[command(visible_alias = "t")]
    Tree {
        #[arg(long)]
        format: Option<String>,
    },

    /// Add/edit description for a vision
    #[command(visible_alias = "d")]
    Describe {
        id: String,
        #[arg(trailing_var_arg = true)]
        text: Vec<String>,
    },

    /// Link child vision to parent
    #[command(visible_alias = "l")]
    Link { parent: String, child: String },

    /// Mark vision as achieved
    #[command(visible_alias = "c")]
    Complete { id: String },
}

impl VisionCommands {
    pub fn execute(self) -> TensileResult<()> {
        match self {
            VisionCommands::New { title } => new_vision(title),
            VisionCommands::Tree { format } => view_tree(format),
            VisionCommands::Describe { id, text } => describe_vision(id, text),
            VisionCommands::Link { parent, child } => link_visions(parent, child),
            VisionCommands::Complete { id } => complete_vision(id),
        }
    }
}

fn new_vision(title: String) -> TensileResult<()> {
    let mut db = persistence::load_database()?;
    let vision = Vision::new(title.clone());
    let id = vision.id;
    db.visions.push(vision);
    persistence::save_database(&db)?;
    println!("✓ Created vision: {} [{}]", title, id);
    Ok(())
}

fn view_tree(format: Option<String>) -> TensileResult<()> {
    let db = persistence::load_database()?;

    if format.as_deref() == Some("json") {
        let json = serde_json::to_string_pretty(&db.visions)
            .map_err(|e| TensileError::Serialization(e.to_string()))?;
        println!("{}", json);
    } else {
        println!("\nVisions:");
        for vision in &db.visions {
            let state_marker = match &vision.state {
                crate::models::VisionState::Achieved => "✓",
                crate::models::VisionState::Blocked => "⊗",
                crate::models::VisionState::InProgress => "→",
                _ => "○",
            };
            println!(
                "  {} {} [{}] {}",
                state_marker, vision.title, vision.id, vision.state
            );
            if let Some(desc) = &vision.description {
                println!("     {}", desc);
            }
        }
    }

    Ok(())
}

fn describe_vision(id: String, text: Vec<String>) -> TensileResult<()> {
    let uuid =
        Uuid::parse_str(&id).map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", id)))?;

    let mut db = persistence::load_database()?;
    let vision_title = {
        let vision = db
            .visions
            .iter_mut()
            .find(|v| v.id == uuid)
            .ok_or_else(|| TensileError::NotFound(format!("Vision not found: {}", id)))?;

        vision.description = Some(text.join(" "));
        vision.title.clone()
    };

    persistence::save_database(&db)?;
    println!("✓ Description updated for vision: {}", vision_title);
    Ok(())
}

fn link_visions(parent_id: String, child_id: String) -> TensileResult<()> {
    let parent_uuid = Uuid::parse_str(&parent_id)
        .map_err(|_| TensileError::Parse(format!("Invalid parent UUID: {}", parent_id)))?;
    let child_uuid = Uuid::parse_str(&child_id)
        .map_err(|_| TensileError::Parse(format!("Invalid child UUID: {}", child_id)))?;

    let mut db = persistence::load_database()?;

    let child_exists = db.visions.iter().any(|v| v.id == child_uuid);
    let parent_exists = db.visions.iter().any(|v| v.id == parent_uuid);

    if !child_exists || !parent_exists {
        return Err(TensileError::NotFound(
            "Parent or child vision not found".to_string(),
        ));
    }

    let child = db.visions.iter_mut().find(|v| v.id == child_uuid).unwrap();
    child.parent = Some(parent_uuid);

    let parent = db.visions.iter_mut().find(|v| v.id == parent_uuid).unwrap();
    if !parent.children.contains(&child_uuid) {
        parent.children.push(child_uuid);
    }

    persistence::save_database(&db)?;
    println!("✓ Linked visions");
    Ok(())
}

fn complete_vision(id: String) -> TensileResult<()> {
    let uuid =
        Uuid::parse_str(&id).map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", id)))?;

    let mut db = persistence::load_database()?;
    let vision_title = {
        let vision = db
            .visions
            .iter_mut()
            .find(|v| v.id == uuid)
            .ok_or_else(|| TensileError::NotFound(format!("Vision not found: {}", id)))?;

        vision.completed = true;
        vision.state = crate::models::VisionState::Achieved;
        vision.title.clone()
    };

    persistence::save_database(&db)?;
    println!("✓ Vision achieved: {}", vision_title);
    Ok(())
}
