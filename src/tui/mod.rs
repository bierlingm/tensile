#[cfg(feature = "tui")]
pub mod app;
#[cfg(feature = "tui")]
pub mod dashboard;
#[cfg(feature = "tui")]
pub mod events;

#[cfg(feature = "tui")]
pub use app::App;
#[cfg(feature = "tui")]
pub use dashboard::draw_dashboard;
#[cfg(feature = "tui")]
pub use events::EventHandler;

#[cfg(feature = "tui")]
pub async fn run_dashboard() -> crate::TensileResult<()> {
    use crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::prelude::*;
    use std::io;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new()?;
    let mut event_handler = EventHandler::new();

    // Main loop
    loop {
        terminal.draw(|f| draw_dashboard(f, &app))?;

        if event_handler.poll()? {
            if app.handle_events(&event_handler)? {
                break;
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
