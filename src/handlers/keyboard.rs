//! Keyboard event handlers for the main application screens
//! 
//! This module handles keyboard events for the main application screens,
//! including navigation between sections, method selection, and input handling.

use crate::app::{App, CurrentScreen, ValuesScreen};
use crate::error::Result;
use crate::logic::HttpMethod;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles keyboard events for the main screens (Url, Values, Response)
pub async fn handle_main_screen_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    // Handle method dropdown if open
    if app.method_dropdown_open {
        return handle_method_dropdown_keys(app, key).await;
    }

    match key.code {
        // Navigation between main sections
        KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            navigate_section_down(app);
            Ok(None)
        }
        KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            navigate_section_up(app);
            Ok(None)
        }
        
        // URL editing
        KeyCode::Char('u') => {
            app.current_screen = CurrentScreen::EditingUrl;
            Ok(None)
        }
        
        // Method selection
        KeyCode::Char('m') => {
            open_method_dropdown(app);
            Ok(None)
        }
        
        // Send request
        KeyCode::Enter => {
            handle_send_request(app).await
        }
        
        // Tab management
        KeyCode::Char('t') => {
            handle_new_tab(app)
        }
        KeyCode::Char('x') => {
            handle_close_tab(app)
        }
        KeyCode::Tab => {
            handle_next_tab(app)
        }
        KeyCode::BackTab => {
            handle_prev_tab(app)
        }
        
        // Screen-specific handlers
        _ => match app.current_screen {
            CurrentScreen::Values => handle_values_screen_keys(app, key).await,
            CurrentScreen::Response => handle_response_screen_keys(app, key).await,
            CurrentScreen::Url => handle_url_screen_keys(app, key).await,
            _ => Ok(None),
        }
    }
}

/// Handles method dropdown navigation
async fn handle_method_dropdown_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    match key.code {
        KeyCode::Up => {
            if app.method_dropdown_selected == 0 {
                app.method_dropdown_selected = 3;
            } else {
                app.method_dropdown_selected -= 1;
            }
            Ok(None)
        }
        KeyCode::Down => {
            if app.method_dropdown_selected == 3 {
                app.method_dropdown_selected = 0;
            } else {
                app.method_dropdown_selected += 1;
            }
            Ok(None)
        }
        KeyCode::Enter => {
            app.selected_method = match app.method_dropdown_selected {
                0 => HttpMethod::GET,
                1 => HttpMethod::POST,
                2 => HttpMethod::PUT,
                3 => HttpMethod::DELETE,
                _ => HttpMethod::GET,
            };
            app.method_dropdown_open = false;
            Ok(None)
        }
        KeyCode::Esc => {
            app.method_dropdown_open = false;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Handles keys specific to the Values screen
async fn handle_values_screen_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    match key.code {
        // Navigate between tabs
        KeyCode::Char('h') | KeyCode::Left => {
            app.values_screen = match app.values_screen {
                ValuesScreen::Headers => ValuesScreen::Body,
                ValuesScreen::Params => ValuesScreen::Headers,
                _ => app.values_screen,
            };
            Ok(None)
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app.values_screen = match app.values_screen {
                ValuesScreen::Body => ValuesScreen::Headers,
                ValuesScreen::Headers => ValuesScreen::Params,
                _ => app.values_screen,
            };
            Ok(None)
        }
        
        // Enter editing mode
        KeyCode::Char('i') => {
            match app.values_screen {
                ValuesScreen::Body => {
                    app.current_screen = CurrentScreen::EditingBody;
                }
                ValuesScreen::Headers => {
                    app.current_screen = CurrentScreen::EditingHeaders;
                }
                ValuesScreen::Params => {
                    app.current_screen = CurrentScreen::EditingParams;
                }
            }
            Ok(None)
        }
        
        _ => Ok(None),
    }
}

/// Handles keys specific to the Response screen
async fn handle_response_screen_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    match key.code {
        // Navigate between response tabs
        KeyCode::Left | KeyCode::Char('h') => {
            app.response_tab_selected = 0; // Headers
            Ok(None)
        }
        KeyCode::Right | KeyCode::Char('b') => {
            app.response_tab_selected = 1; // Body
            Ok(None)
        }
        
        // Scroll response content
        KeyCode::Char('j') => {
            if app.response_tab_selected == 1 {
                app.response_scroll = app.response_scroll.saturating_add(1);
            }
            Ok(None)
        }
        KeyCode::Char('k') => {
            if app.response_tab_selected == 1 {
                app.response_scroll = app.response_scroll.saturating_sub(1);
            }
            Ok(None)
        }
        
        _ => Ok(None),
    }
}

/// Handles keys specific to the URL screen
async fn handle_url_screen_keys(_app: &mut App, _key: KeyEvent) -> Result<Option<String>> {
    // URL screen doesn't have specific key handlers beyond global ones
    Ok(None)
}

/// Handles URL editing mode
pub async fn handle_url_editing_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    match key.code {
        KeyCode::Enter => {
            if let Err(e) = app.save_current_tab_state() {
                return Ok(Some(format!("Failed to save tab state: {}", e)));
            }
            app.current_screen = CurrentScreen::Url;
            Ok(None)
        }
        KeyCode::Backspace => {
            app.url_input.pop();
            Ok(None)
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Url;
            Ok(None)
        }
        KeyCode::Char(c) => {
            app.url_input.push(c);
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Handles body editing mode
pub async fn handle_body_editing_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    match key.code {
        KeyCode::Enter => {
            app.body_input.push('\n');
            Ok(None)
        }
        KeyCode::Backspace => {
            app.body_input.pop();
            Ok(None)
        }
        KeyCode::Esc => {
            app.current_screen = CurrentScreen::Values;
            Ok(None)
        }
        KeyCode::Char(c) => {
            app.body_input.push(c);
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Handles headers editing mode
pub async fn handle_headers_editing_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    match key.code {
        KeyCode::Enter => {
            if !app.current_header_key.is_empty() {
                if let Err(e) = app.add_header() {
                    return Ok(Some(format!("Header error: {}", e)));
                }
            } else {
                app.current_screen = CurrentScreen::Values;
            }
            Ok(None)
        }
        KeyCode::Tab => {
            // Switch focus between key and value (simplified)
            if !app.current_header_key.is_empty() && app.current_header_value.is_empty() {
                app.current_header_value.push(' ');
                app.current_header_value.clear();
            }
            Ok(None)
        }
        KeyCode::Backspace => {
            if !app.current_header_value.is_empty() {
                app.current_header_value.pop();
            } else if !app.current_header_key.is_empty() {
                app.current_header_key.pop();
            }
            Ok(None)
        }
        KeyCode::Esc => {
            app.current_header_key.clear();
            app.current_header_value.clear();
            app.current_screen = CurrentScreen::Values;
            Ok(None)
        }
        KeyCode::Char(':') => {
            if !app.current_header_key.is_empty() && app.current_header_value.is_empty() {
                app.current_header_key.push(':');
            } else if !app.current_header_key.contains(':') {
                app.current_header_key.push(':');
            } else {
                app.current_header_value.push(':');
            }
            Ok(None)
        }
        KeyCode::Char(' ') => {
            if app.current_header_key.ends_with(':') && app.current_header_value.is_empty() {
                // Start value input after ': '
            } else if !app.current_header_value.is_empty() || !app.current_header_key.is_empty() {
                if app.current_header_key.contains(':') {
                    app.current_header_value.push(' ');
                } else {
                    app.current_header_key.push(' ');
                }
            }
            Ok(None)
        }
        KeyCode::Char(c) => {
            if !app.current_header_key.contains(':') {
                app.current_header_key.push(c);
            } else {
                app.current_header_value.push(c);
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Handles parameters editing mode
pub async fn handle_params_editing_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    match key.code {
        KeyCode::Enter => {
            if !app.current_param_key.is_empty() {
                if let Err(e) = app.add_param() {
                    return Ok(Some(format!("Parameter error: {}", e)));
                }
            } else {
                app.current_screen = CurrentScreen::Values;
            }
            Ok(None)
        }
        KeyCode::Tab => {
            // Switch focus between key and value
            if !app.current_param_key.is_empty() && app.current_param_value.is_empty() {
                app.current_param_value.push(' ');
                app.current_param_value.clear();
            }
            Ok(None)
        }
        KeyCode::Backspace => {
            if !app.current_param_value.is_empty() {
                app.current_param_value.pop();
            } else if !app.current_param_key.is_empty() {
                app.current_param_key.pop();
            }
            Ok(None)
        }
        KeyCode::Esc => {
            app.current_param_key.clear();
            app.current_param_value.clear();
            app.current_screen = CurrentScreen::Values;
            Ok(None)
        }
        KeyCode::Char('=') => {
            if !app.current_param_key.is_empty() && app.current_param_value.is_empty() {
                app.current_param_key.push('=');
            } else if !app.current_param_key.contains('=') {
                app.current_param_key.push('=');
            } else {
                app.current_param_value.push('=');
            }
            Ok(None)
        }
        KeyCode::Char(c) => {
            if !app.current_param_key.contains('=') {
                app.current_param_key.push(c);
            } else {
                app.current_param_value.push(c);
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// Handles help screen navigation
pub async fn handle_help_keys(app: &mut App, key: KeyEvent) -> Result<Option<String>> {
    match key.code {
        KeyCode::Esc => {
            app.hide_help();
            Ok(None)
        }
        KeyCode::Char('j') => {
            let help_content = app.get_help_content();
            if app.help_scroll < help_content.len().saturating_sub(1) {
                app.help_scroll = app.help_scroll.saturating_add(1);
            }
            Ok(None)
        }
        KeyCode::Char('k') => {
            app.help_scroll = app.help_scroll.saturating_sub(1);
            Ok(None)
        }
        _ => Ok(None),
    }
}

// Helper functions for navigation and actions

fn navigate_section_down(app: &mut App) {
    app.current_screen = match app.current_screen {
        CurrentScreen::Url => CurrentScreen::Values,
        CurrentScreen::Values => CurrentScreen::Response,
        _ => app.current_screen,
    };
}

fn navigate_section_up(app: &mut App) {
    app.current_screen = match app.current_screen {
        CurrentScreen::Response => CurrentScreen::Values,
        CurrentScreen::Values => CurrentScreen::Url,
        _ => app.current_screen,
    };
}

fn open_method_dropdown(app: &mut App) {
    app.method_dropdown_open = true;
    app.method_dropdown_selected = match app.selected_method {
        HttpMethod::GET => 0,
        HttpMethod::POST => 1,
        HttpMethod::PUT => 2,
        HttpMethod::DELETE => 3,
    };
}

async fn handle_send_request(app: &mut App) -> Result<Option<String>> {
    // Validate request before sending
    if let Err(e) = app.validate_current_request() {
        return Ok(Some(format!("Validation error: {}", e)));
    }
    
    // Send request with error handling
    match app.tabs[app.selected_tab].request.send().await {
        Ok((status_code, headers, body)) => {
            match crate::logic::response::Response::new(status_code, headers.clone(), body.clone()) {
                Ok(response) => {
                    app.tabs[app.selected_tab].response = Some(response);
                    Ok(None)
                }
                Err(e) => {
                    // Still create response with unchecked method for display
                    let response = crate::logic::response::Response::new_unchecked(status_code, headers, body);
                    app.tabs[app.selected_tab].response = Some(response);
                    Ok(Some(format!("Response parsing error: {}", e)))
                }
            }
        }
        Err(e) => {
            Ok(Some(format!("Request failed: {}", e)))
        }
    }
}

fn handle_new_tab(app: &mut App) -> Result<Option<String>> {
    if let Err(e) = app.add_new_tab() {
        Ok(Some(format!("Tab error: {}", e)))
    } else {
        Ok(None)
    }
}

fn handle_close_tab(app: &mut App) -> Result<Option<String>> {
    if let Err(e) = app.close_current_tab() {
        Ok(Some(format!("Tab error: {}", e)))
    } else {
        Ok(None)
    }
}

fn handle_next_tab(app: &mut App) -> Result<Option<String>> {
    if let Err(e) = app.next_tab() {
        Ok(Some(format!("Tab error: {}", e)))
    } else {
        Ok(None)
    }
}

fn handle_prev_tab(app: &mut App) -> Result<Option<String>> {
    if let Err(e) = app.prev_tab() {
        Ok(Some(format!("Tab error: {}", e)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventKind, KeyEventState};

    fn create_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    fn create_key_event_with_ctrl(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[tokio::test]
    async fn test_navigation_keys() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Url;

        // Test Ctrl+j navigation
        let key = create_key_event_with_ctrl(KeyCode::Char('j'));
        let result = handle_main_screen_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.current_screen, CurrentScreen::Values);

        // Test Ctrl+k navigation
        let key = create_key_event_with_ctrl(KeyCode::Char('k'));
        let result = handle_main_screen_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.current_screen, CurrentScreen::Url);
    }

    #[tokio::test]
    async fn test_url_editing() {
        let mut app = App::new();
        
        // Start editing
        let key = create_key_event(KeyCode::Char('u'));
        let result = handle_main_screen_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.current_screen, CurrentScreen::EditingUrl);

        // Type some text
        let key = create_key_event(KeyCode::Char('h'));
        let result = handle_url_editing_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.url_input, "h");

        // Exit editing
        let key = create_key_event(KeyCode::Esc);
        let result = handle_url_editing_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.current_screen, CurrentScreen::Url);
    }

    #[tokio::test]
    async fn test_method_dropdown() {
        let mut app = App::new();
        
        // Open dropdown
        let key = create_key_event(KeyCode::Char('m'));
        let result = handle_main_screen_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert!(app.method_dropdown_open);

        // Navigate down
        let key = create_key_event(KeyCode::Down);
        let result = handle_method_dropdown_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.method_dropdown_selected, 1);

        // Select method
        let key = create_key_event(KeyCode::Enter);
        let result = handle_method_dropdown_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert!(!app.method_dropdown_open);
        assert_eq!(app.selected_method, HttpMethod::POST);
    }

    #[tokio::test]
    async fn test_values_screen_navigation() {
        let mut app = App::new();
        app.current_screen = CurrentScreen::Values;
        app.values_screen = ValuesScreen::Body;

        // Navigate right
        let key = create_key_event(KeyCode::Char('l'));
        let result = handle_values_screen_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.values_screen, ValuesScreen::Headers);

        // Navigate right again
        let key = create_key_event(KeyCode::Char('l'));
        let result = handle_values_screen_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.values_screen, ValuesScreen::Params);

        // Navigate left
        let key = create_key_event(KeyCode::Char('h'));
        let result = handle_values_screen_keys(&mut app, key).await.unwrap();
        assert!(result.is_none());
        assert_eq!(app.values_screen, ValuesScreen::Headers);
    }
}