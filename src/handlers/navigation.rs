//! Navigation handlers for the Restless application
//!
//! This module contains handlers for navigation between different screens
//! and sections of the application.

use crate::app::{App, CurrentScreen, ValuesScreen};
use crate::error::Result;

/// Handles navigation to the next section down
pub fn navigate_section_down(app: &mut App) -> Result<Option<String>> {
    app.current_screen = match app.current_screen {
        CurrentScreen::Url => CurrentScreen::Values,
        CurrentScreen::Values => CurrentScreen::Response,
        _ => app.current_screen,
    };
    Ok(None)
}

/// Handles navigation to the previous section up
pub fn navigate_section_up(app: &mut App) -> Result<Option<String>> {
    app.current_screen = match app.current_screen {
        CurrentScreen::Response => CurrentScreen::Values,
        CurrentScreen::Values => CurrentScreen::Url,
        _ => app.current_screen,
    };
    Ok(None)
}

/// Handles navigation between values tabs (left)
pub fn navigate_values_left(app: &mut App) -> Result<Option<String>> {
    if matches!(app.current_screen, CurrentScreen::Values) {
        app.values_screen = match app.values_screen {
            ValuesScreen::Headers => ValuesScreen::Body,
            ValuesScreen::Params => ValuesScreen::Headers,
            _ => app.values_screen,
        };
    }
    Ok(None)
}

/// Handles navigation between values tabs (right)
pub fn navigate_values_right(app: &mut App) -> Result<Option<String>> {
    if matches!(app.current_screen, CurrentScreen::Values) {
        app.values_screen = match app.values_screen {
            ValuesScreen::Body => ValuesScreen::Headers,
            ValuesScreen::Headers => ValuesScreen::Params,
            _ => app.values_screen,
        };
    }
    Ok(None)
}

/// Handles navigation between response tabs
pub fn navigate_response_tabs(app: &mut App, tab_index: usize) -> Result<Option<String>> {
    if matches!(app.current_screen, CurrentScreen::Response) {
        if tab_index < 2 {
            app.response_tab_selected = tab_index;
        }
    }
    Ok(None)
}

/// Handles scrolling in the response section
pub fn scroll_response(app: &mut App, direction: ScrollDirection) -> Result<Option<String>> {
    if matches!(app.current_screen, CurrentScreen::Response) && app.response_tab_selected == 1 {
        match direction {
            ScrollDirection::Up => {
                app.response_scroll = app.response_scroll.saturating_sub(1);
            }
            ScrollDirection::Down => {
                app.response_scroll = app.response_scroll.saturating_add(1);
            }
        }
    }
    Ok(None)
}

/// Handles scrolling in the help section
pub fn scroll_help(app: &mut App, direction: ScrollDirection) -> Result<Option<String>> {
    if app.help_visible {
        let help_content = app.get_help_content();
        match direction {
            ScrollDirection::Up => {
                app.help_scroll = app.help_scroll.saturating_sub(1);
            }
            ScrollDirection::Down => {
                if app.help_scroll < help_content.len().saturating_sub(1) {
                    app.help_scroll = app.help_scroll.saturating_add(1);
                }
            }
        }
    }
    Ok(None)
}

/// Enters editing mode for the current context
pub fn enter_edit_mode(app: &mut App) -> Result<Option<String>> {
    match app.current_screen {
        CurrentScreen::Url => {
            app.current_screen = CurrentScreen::EditingUrl;
        }
        CurrentScreen::Values => match app.values_screen {
            ValuesScreen::Body => {
                app.current_screen = CurrentScreen::EditingBody;
            }
            ValuesScreen::Headers => {
                app.current_screen = CurrentScreen::EditingHeaders;
            }
            ValuesScreen::Params => {
                app.current_screen = CurrentScreen::EditingParams;
            }
        },
        _ => {
            return Ok(Some(
                "Cannot enter edit mode from current screen".to_string(),
            ));
        }
    }
    Ok(None)
}

/// Exits editing mode and returns to the parent screen
pub fn exit_edit_mode(app: &mut App) -> Result<Option<String>> {
    match app.current_screen {
        CurrentScreen::EditingUrl => {
            app.current_screen = CurrentScreen::Url;
        }
        CurrentScreen::EditingBody
        | CurrentScreen::EditingHeaders
        | CurrentScreen::EditingParams => {
            app.current_screen = CurrentScreen::Values;
        }
        _ => {
            return Ok(Some("Not in edit mode".to_string()));
        }
    }
    Ok(None)
}

/// Toggles the help screen
pub fn toggle_help(app: &mut App) -> Result<Option<String>> {
    if app.help_visible {
        app.hide_help();
    } else {
        app.show_help();
    }
    Ok(None)
}

/// Direction for scrolling operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
}

/// Gets the current navigation context as a string for display
pub fn get_navigation_context(app: &App) -> String {
    match app.current_screen {
        CurrentScreen::Url => "URL Input".to_string(),
        CurrentScreen::Values => {
            let tab = match app.values_screen {
                ValuesScreen::Body => "Body",
                ValuesScreen::Headers => "Headers",
                ValuesScreen::Params => "Params",
            };
            format!("Values - {}", tab)
        }
        CurrentScreen::Response => {
            let tab = if app.response_tab_selected == 0 {
                "Headers"
            } else {
                "Body"
            };
            format!("Response - {}", tab)
        }
        CurrentScreen::EditingUrl => "Editing URL".to_string(),
        CurrentScreen::EditingBody => "Editing Body".to_string(),
        CurrentScreen::EditingHeaders => "Editing Headers".to_string(),
        CurrentScreen::EditingParams => "Editing Params".to_string(),
        CurrentScreen::Help => "Help".to_string(),
        CurrentScreen::Exiting => "Exiting".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigate_section_down() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Url;

        navigate_section_down(&mut app).unwrap();
        assert_eq!(app.current_screen, CurrentScreen::Values);

        navigate_section_down(&mut app).unwrap();
        assert_eq!(app.current_screen, CurrentScreen::Response);

        // Should stay at Response
        navigate_section_down(&mut app).unwrap();
        assert_eq!(app.current_screen, CurrentScreen::Response);
    }

    #[test]
    fn test_navigate_section_up() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Response;

        navigate_section_up(&mut app).unwrap();
        assert_eq!(app.current_screen, CurrentScreen::Values);

        navigate_section_up(&mut app).unwrap();
        assert_eq!(app.current_screen, CurrentScreen::Url);

        // Should stay at Url
        navigate_section_up(&mut app).unwrap();
        assert_eq!(app.current_screen, CurrentScreen::Url);
    }

    #[test]
    fn test_navigate_values_tabs() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Values;
        app.values_screen = ValuesScreen::Body;

        navigate_values_right(&mut app).unwrap();
        assert_eq!(app.values_screen, ValuesScreen::Headers);

        navigate_values_right(&mut app).unwrap();
        assert_eq!(app.values_screen, ValuesScreen::Params);

        // Should stay at Params
        navigate_values_right(&mut app).unwrap();
        assert_eq!(app.values_screen, ValuesScreen::Params);

        navigate_values_left(&mut app).unwrap();
        assert_eq!(app.values_screen, ValuesScreen::Headers);

        navigate_values_left(&mut app).unwrap();
        assert_eq!(app.values_screen, ValuesScreen::Body);
    }

    #[test]
    fn test_enter_exit_edit_mode() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Url;

        enter_edit_mode(&mut app).unwrap();
        assert_eq!(app.current_screen, CurrentScreen::EditingUrl);

        exit_edit_mode(&mut app).unwrap();
        assert_eq!(app.current_screen, CurrentScreen::Url);
    }

    #[test]
    fn test_toggle_help() {
        let mut app = App::new();
        assert!(!app.help_visible);

        toggle_help(&mut app).unwrap();
        assert!(app.help_visible);

        toggle_help(&mut app).unwrap();
        assert!(!app.help_visible);
    }

    #[test]
    fn test_scroll_response() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Response;
        app.response_tab_selected = 1; // Body tab
        app.response_scroll = 5;

        scroll_response(&mut app, ScrollDirection::Up).unwrap();
        assert_eq!(app.response_scroll, 4);

        scroll_response(&mut app, ScrollDirection::Down).unwrap();
        assert_eq!(app.response_scroll, 5);
    }

    #[test]
    fn test_get_navigation_context() {
        let mut app = App::new();

        app.current_screen = CurrentScreen::Url;
        assert_eq!(get_navigation_context(&app), "URL Input");

        app.current_screen = CurrentScreen::Values;
        app.values_screen = ValuesScreen::Body;
        assert_eq!(get_navigation_context(&app), "Values - Body");

        app.current_screen = CurrentScreen::Response;
        app.response_tab_selected = 0;
        assert_eq!(get_navigation_context(&app), "Response - Headers");
    }
}
