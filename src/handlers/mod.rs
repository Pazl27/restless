//! Event handlers for the Restless application
//!
//! This module contains all event handling logic, organized by functionality.
//! Each handler is responsible for processing specific types of events and
//! updating the application state accordingly.

#![allow(dead_code)]

pub mod keyboard;
pub mod navigation;
pub mod request;
pub mod tab;

pub use keyboard::*;

use crate::app::{App, CurrentScreen};
use crate::error::Result;
use crossterm::event::{KeyCode, KeyEvent};

/// Main event handler that routes events to appropriate sub-handlers
pub async fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    // Global key handlers that work in any screen
    if let Some(result) = handle_global_keys(app, key).await? {
        return Ok(result);
    }

    // Screen-specific handlers
    match app.current_screen {
        CurrentScreen::Url | CurrentScreen::Values | CurrentScreen::Response => {
            handle_main_screen_keys(app, key).await
        }
        CurrentScreen::EditingUrl => handle_url_editing_keys(app, key).await,
        CurrentScreen::EditingBody => handle_body_editing_keys(app, key).await,
        CurrentScreen::EditingHeaders => handle_headers_editing_keys(app, key).await,
        CurrentScreen::EditingParams => handle_params_editing_keys(app, key).await,
        CurrentScreen::Help => handle_help_keys(app, key).await,
        CurrentScreen::Exiting => Ok(Some("Application exiting".to_string())),
    }
}

/// Handles global keys that work in any screen
async fn handle_global_keys(app: &mut App, key: KeyEvent) -> Result<Option<Option<String>>> {
    match key.code {
        KeyCode::Char('q') if !is_editing_mode(app) => {
            app.current_screen = CurrentScreen::Exiting;
            Ok(Some(None))
        }
        KeyCode::Char('?') if !is_editing_mode(app) => {
            if app.help_visible {
                app.hide_help();
            } else {
                app.show_help();
            }
            Ok(Some(None))
        }
        _ => Ok(None),
    }
}

/// Checks if the app is in any editing mode
fn is_editing_mode(app: &App) -> bool {
    matches!(
        app.current_screen,
        CurrentScreen::EditingUrl
            | CurrentScreen::EditingBody
            | CurrentScreen::EditingHeaders
            | CurrentScreen::EditingParams
    )
}

/// Event handler result type
#[derive(Debug, Clone)]
#[cfg(test)]
pub enum EventResult {
    /// Continue processing normally
    Continue,
    /// Exit the application
    Exit,
    /// Show an error message
    Error(String),
    /// Show an info message
    Info(String),
}

#[cfg(test)]
impl EventResult {
    pub fn error<S: Into<String>>(message: S) -> Self {
        Self::Error(message.into())
    }

    pub fn info<S: Into<String>>(message: S) -> Self {
        Self::Info(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    fn create_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    fn create_key_event_with_modifiers(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_is_editing_mode() {
        let mut app = App::new();

        assert!(!is_editing_mode(&app));

        app.current_screen = CurrentScreen::EditingUrl;
        assert!(is_editing_mode(&app));

        app.current_screen = CurrentScreen::EditingBody;
        assert!(is_editing_mode(&app));

        app.current_screen = CurrentScreen::Values;
        assert!(!is_editing_mode(&app));
    }

    #[tokio::test]
    async fn test_global_quit_key() {
        let mut app = App::new();
        let key = create_key_event(KeyCode::Char('q'));

        let result = handle_global_keys(&mut app, key).await.unwrap();
        assert!(result.is_some());
        assert_eq!(app.current_screen, CurrentScreen::Exiting);
    }

    #[tokio::test]
    async fn test_global_help_key() {
        let mut app = App::new();
        let key = create_key_event(KeyCode::Char('?'));

        assert!(!app.help_visible);

        let result = handle_global_keys(&mut app, key).await.unwrap();
        assert!(result.is_some());
        assert!(app.help_visible);

        // Test toggle
        let result = handle_global_keys(&mut app, key).await.unwrap();
        assert!(result.is_some());
        assert!(!app.help_visible);
    }

    #[tokio::test]
    async fn test_global_keys_ignored_in_editing_mode() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::EditingUrl;

        let quit_key = create_key_event(KeyCode::Char('q'));
        let result = handle_global_keys(&mut app, quit_key).await.unwrap();
        assert!(result.is_none());
        assert_ne!(app.current_screen, CurrentScreen::Exiting);

        let help_key = create_key_event(KeyCode::Char('?'));
        let result = handle_global_keys(&mut app, help_key).await.unwrap();
        assert!(result.is_none());
        assert!(!app.help_visible);
    }
}
