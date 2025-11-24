pub mod action;
pub mod pattern;
pub mod reality;
pub mod state;
pub mod vision;

use crate::error::TensileResult;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Manage visions
    #[command(subcommand)]
    Vision(vision::VisionCommands),

    /// Manage reality assessments
    #[command(subcommand)]
    Reality(reality::RealityCommands),

    /// Log actions
    #[command(subcommand)]
    Action(action::ActionCommands),

    /// Analyze patterns
    #[command(subcommand)]
    Pattern(pattern::PatternCommands),

    /// Manage vision states
    #[command(subcommand)]
    State(state::StateCommands),

    /// Show structural coaching prompts
    #[command(visible_alias = "p")]
    Prompt,
}

impl Commands {
    pub fn execute(self) -> TensileResult<()> {
        match self {
            Commands::Vision(cmd) => cmd.execute(),
            Commands::Reality(cmd) => cmd.execute(),
            Commands::Action(cmd) => cmd.execute(),
            Commands::Pattern(cmd) => cmd.execute(),
            Commands::State(cmd) => cmd.execute(),
            Commands::Prompt => crate::cli::commands::prompt_command(),
        }
    }
}

pub fn prompt_command() -> TensileResult<()> {
    let prompts = [
        "What would achieving this vision enable?",
        "What obstacles stand between current reality and vision?",
        "What is the next smallest step toward this vision?",
        "How do you know you're making progress?",
        "What assumptions are you holding about this vision?",
    ];

    let prompt = prompts[std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        % prompts.len()];

    println!("\n> {}", prompt);
    Ok(())
}
