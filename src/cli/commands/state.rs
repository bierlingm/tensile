use crate::{
    engine::StateMachine,
    error::{TensileError, TensileResult},
    models::VisionState,
    persistence,
};
use clap::Subcommand;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum StateCommands {
    /// Show current state of a vision
    #[command(visible_alias = "s")]
    Show { vision_id: String },

    /// Transition vision to a new state
    #[command(visible_alias = "t")]
    Transition { vision_id: String, state: String },
}

impl StateCommands {
    pub fn execute(self) -> TensileResult<()> {
        match self {
            StateCommands::Show { vision_id } => show_state(vision_id),
            StateCommands::Transition { vision_id, state } => transition_state(vision_id, state),
        }
    }
}

fn show_state(vision_id: String) -> TensileResult<()> {
    let uuid = Uuid::parse_str(&vision_id)
        .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", vision_id)))?;

    let db = persistence::load_database()?;
    let vision = db
        .visions
        .iter()
        .find(|v| v.id == uuid)
        .ok_or_else(|| TensileError::NotFound(format!("Vision not found: {}", vision_id)))?;

    println!("\nState of Vision: {}", vision.title);
    println!("  Current State: {}", vision.state);
    println!("  Created: {}", vision.created_at.format("%Y-%m-%d"));
    println!(
        "  Completed: {}",
        if vision.completed { "Yes" } else { "No" }
    );

    Ok(())
}

fn transition_state(vision_id: String, new_state_str: String) -> TensileResult<()> {
    let uuid = Uuid::parse_str(&vision_id)
        .map_err(|_| TensileError::Parse(format!("Invalid UUID: {}", vision_id)))?;

    let new_state = VisionState::from_str(&new_state_str).map_err(TensileError::Parse)?;

    let mut db = persistence::load_database()?;
    let (old_state_str, new_state_str_display) = {
        let vision = db
            .visions
            .iter_mut()
            .find(|v| v.id == uuid)
            .ok_or_else(|| TensileError::NotFound(format!("Vision not found: {}", vision_id)))?;

        let old_state = vision.state.clone();
        StateMachine::validate_transition(&old_state, &new_state)?;

        vision.state = new_state.clone();
        (old_state.to_string(), new_state.to_string())
    };

    persistence::save_database(&db)?;
    println!(
        "✓ State transitioned: {} -> {}",
        old_state_str, new_state_str_display
    );
    Ok(())
}
