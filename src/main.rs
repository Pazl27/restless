use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};

mod app;
use app::App;

mod ui;
use ui::ui;

mod error;
mod handlers;
mod logic;
mod terminal;

use crate::error::RestlessError;
use crate::handlers::handle_key_event;
use crate::terminal::TerminalManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize terminal
    let mut terminal_manager = TerminalManager::new().map_err(|e| {
        eprintln!("Failed to initialize terminal: {}", e);
        e
    })?;

    // Validate terminal size
    if let Err(e) = terminal_manager.validate_size() {
        eprintln!("Terminal size error: {}", e);
        return Err(e.into());
    }

    // Initialize application
    let mut app = App::new();

    // Run the application
    let result = run_app(&mut terminal_manager, &mut app).await;

    // Cleanup is handled by the TerminalManager's Drop implementation
    // but we can also explicitly cleanup for better error handling
    if let Err(cleanup_error) = terminal_manager.cleanup() {
        eprintln!("Warning: Failed to cleanup terminal: {}", cleanup_error);
    }

    result
}

async fn run_app(terminal_manager: &mut TerminalManager, app: &mut App) -> Result<()> {
    // Store any error message to display to the user
    let mut error_message: Option<String> = None;

    loop {
        // Draw the UI
        terminal_manager
            .terminal_mut()
            .draw(|f| ui(f, app, &error_message))
            .map_err(|e| RestlessError::terminal(format!("Failed to draw UI: {}", e)))?;

        // Handle events
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            // If there's an error message, any key press dismisses it
            if error_message.is_some() {
                error_message = None;
                continue;
            }

            // Handle the key event using the modular handler
            match handle_key_event(app, key).await {
                Ok(Some(msg)) => {
                    error_message = Some(msg);
                }
                Ok(None) => {
                    // Check if we should exit
                    if matches!(app.current_screen, app::CurrentScreen::Exiting) {
                        return Ok(());
                    }
                }
                Err(e) => {
                    error_message = Some(format!("Error: {}", e));
                }
            }
        }
    }
}
