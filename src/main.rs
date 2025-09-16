use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::io;

mod app;
use app::{App, CurrentScreen, ValuesScreen};

mod ui;
use ui::ui;

mod logic;
mod error;
use crate::logic::response::Response;
use crate::logic::HttpMethod;
use crate::error::RestlessError;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize terminal with error handling
    if let Err(e) = setup_terminal().await {
        eprintln!("Failed to initialize terminal: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}

async fn setup_terminal() -> Result<()> {
    enable_raw_mode()
        .map_err(|e| RestlessError::terminal(format!("Failed to enable raw mode: {}", e)))?;
    
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| RestlessError::terminal(format!("Failed to setup terminal: {}", e)))?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)
        .map_err(|e| RestlessError::terminal(format!("Failed to create terminal: {}", e)))?;

    let mut app = App::new();
    
    // Run the app with error handling
    let run_result = run_app(&mut terminal, &mut app).await;
    
    // Always cleanup terminal, even if the app failed
    let cleanup_result = cleanup_terminal(&mut terminal);
    
    // Return the first error if any occurred
    match (run_result, cleanup_result) {
        (Err(e), _) => Err(e),
        (Ok(_), Err(e)) => Err(e),
        (Ok(success), Ok(_)) => Ok(success),
    }
}

fn cleanup_terminal<B: Backend + std::io::Write>(terminal: &mut Terminal<B>) -> Result<()> {
    disable_raw_mode()
        .map_err(|e| RestlessError::terminal(format!("Failed to disable raw mode: {}", e)))?;
    
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    ).map_err(|e| RestlessError::terminal(format!("Failed to cleanup terminal: {}", e)))?;
    
    terminal.show_cursor()
        .map_err(|e| RestlessError::terminal(format!("Failed to show cursor: {}", e)))?;
    
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    // Store any error message to display to the user
    let mut error_message: Option<String> = None;
    
    loop {
        // Pass error message to UI for display
        terminal.draw(|f| ui(f, app, &error_message))
            .map_err(|e| RestlessError::terminal(format!("Failed to draw UI: {}", e)))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            // If there's an error message, any key press dismisses it
            if error_message.is_some() {
                error_message = None;
                continue;
            }

            match app.current_screen {
                CurrentScreen::Url | CurrentScreen::Values | CurrentScreen::Response => {
                    if app.method_dropdown_open {
                        match key.code {
                            KeyCode::Up => {
                                if app.method_dropdown_selected == 0 {
                                    app.method_dropdown_selected = 3;
                                } else {
                                    app.method_dropdown_selected -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if app.method_dropdown_selected == 3 {
                                    app.method_dropdown_selected = 0;
                                } else {
                                    app.method_dropdown_selected += 1;
                                }
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
                            }
                            KeyCode::Esc => {
                                app.method_dropdown_open = false;
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {

                            KeyCode::Char('u') => {
                                app.current_screen = CurrentScreen::EditingUrl;
                            }
                            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                app.current_screen = match app.current_screen {
                                    CurrentScreen::Url => CurrentScreen::Values,
                                    CurrentScreen::Values => CurrentScreen::Response,
                                    _ => app.current_screen,
                                };
                            }
                            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                app.current_screen = match app.current_screen {
                                    CurrentScreen::Response => CurrentScreen::Values,
                                    CurrentScreen::Values => CurrentScreen::Url,
                                    _ => app.current_screen,
                                };
                            }
                            KeyCode::Enter => {
                                // Clear any previous error
                                error_message = None;
                                
                                // Validate request before sending
                                if let Err(e) = app.validate_current_request() {
                                    error_message = Some(app.get_error_message(&e));
                                    continue;
                                }
                                
                                // Send request with error handling
                                match app.tabs[app.selected_tab].request.send().await {
                                    Ok((status_code, headers, body)) => {
                                        match Response::new(status_code, headers.clone(), body.clone()) {
                                            Ok(response) => {
                                                app.tabs[app.selected_tab].response = Some(response);
                                            }
                                            Err(e) => {
                                                error_message = Some(format!("Response parsing error: {}", e));
                                                // Still create response with unchecked method for display
                                                let response = Response::new_unchecked(status_code, headers, body);
                                                app.tabs[app.selected_tab].response = Some(response);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error_message = Some(format!("Request failed: {}", e));
                                    }
                                }
                            }
                            KeyCode::Char('m') => {
                                app.method_dropdown_open = true;
                                app.method_dropdown_selected = match app.selected_method {
                                    HttpMethod::GET => 0,
                                    HttpMethod::POST => 1,
                                    HttpMethod::PUT => 2,
                                    HttpMethod::DELETE => 3,
                                };
                            }
                            KeyCode::Char('q') => {
                                app.current_screen = CurrentScreen::Exiting;
                                return Ok(());
                            }

                            KeyCode::Char('t') => {
                                if let Err(e) = app.add_new_tab() {
                                    error_message = Some(app.get_error_message(&e));
                                }
                            }
                            KeyCode::Char('x') => {
                                if let Err(e) = app.close_current_tab() {
                                    error_message = Some(app.get_error_message(&e));
                                }
                            }
                            KeyCode::Tab => {
                                if let Err(e) = app.next_tab() {
                                    error_message = Some(app.get_error_message(&e));
                                }
                            }
                            KeyCode::BackTab => {
                                if let Err(e) = app.prev_tab() {
                                    error_message = Some(app.get_error_message(&e));
                                }
                            }
                            _ => {}
                        }

                        // Screen-specific key handling
                        match app.current_screen {
                            CurrentScreen::Response => match key.code {
                                KeyCode::Left | KeyCode::Char('h') => {
                                    app.response_tab_selected = 0;
                                }
                                KeyCode::Right | KeyCode::Char('b') => {
                                    app.response_tab_selected = 1;
                                }
                                KeyCode::Char('j') => {
                                    if app.response_tab_selected == 1 {
                                        app.response_scroll =
                                            app.response_scroll.saturating_add(1);
                                    }
                                }
                                KeyCode::Char('k') => {
                                    if app.response_tab_selected == 1 {
                                        app.response_scroll =
                                            app.response_scroll.saturating_sub(1);
                                    }
                                }
                                KeyCode::Char('?') => {
                                    app.show_help();
                                }
                                _ => {}
                            },
                            CurrentScreen::Values => match key.code {
                                KeyCode::Char('h') | KeyCode::Left => {
                                    app.values_screen = match app.values_screen {
                                        ValuesScreen::Headers => ValuesScreen::Body,
                                        ValuesScreen::Params => ValuesScreen::Headers,
                                        _ => app.values_screen,
                                    };
                                }
                                KeyCode::Char('l') | KeyCode::Right => {
                                    app.values_screen = match app.values_screen {
                                        ValuesScreen::Body => ValuesScreen::Headers,
                                        ValuesScreen::Headers => ValuesScreen::Params,
                                        _ => app.values_screen,
                                    };
                                }
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
                                }
                                KeyCode::Char('?') => {
                                    app.show_help();
                                }
                                _ => {}
                            },
                            CurrentScreen::Url => match key.code {
                                KeyCode::Char('?') => {
                                    app.show_help();
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }

                CurrentScreen::Help => match key.code {
                    KeyCode::Esc => {
                        app.hide_help();
                    }
                    KeyCode::Char('j') => {
                        let help_content = app.get_help_content();
                        if app.help_scroll < help_content.len().saturating_sub(1) {
                            app.help_scroll = app.help_scroll.saturating_add(1);
                        }
                    }
                    KeyCode::Char('k') => {
                        app.help_scroll = app.help_scroll.saturating_sub(1);
                    }
                    _ => {}
                },

                CurrentScreen::EditingUrl => match key.code {
                    KeyCode::Enter => {
                        app.tabs[app.selected_tab].request.url = app.url_input.clone();
                        app.current_screen = CurrentScreen::Url;
                    }
                    KeyCode::Backspace => {
                        app.url_input.pop();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Url;
                    }
                    KeyCode::Char(c) => {
                        app.url_input.push(c);
                    }
                    _ => {}
                },
                CurrentScreen::EditingBody => match key.code {
                    KeyCode::Enter => {
                        app.body_input.push('\n');
                    }
                    KeyCode::Backspace => {
                        app.body_input.pop();
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Values;
                    }
                    KeyCode::Char(c) => {
                        app.body_input.push(c);
                    }
                    _ => {}
                },
                CurrentScreen::EditingHeaders => match key.code {
                    KeyCode::Enter => {
                        if !app.current_header_key.is_empty() {
                            if let Err(e) = app.add_header() {
                                error_message = Some(app.get_error_message(&e));
                            }
                        } else {
                            app.current_screen = CurrentScreen::Values;
                        }
                    }
                    KeyCode::Tab => {
                        // Switch focus between key and value (simplified for now)
                        if !app.current_header_key.is_empty() && app.current_header_value.is_empty() {
                            app.current_header_value.push(' '); // Start value input
                            app.current_header_value.clear();
                        }
                    }
                    KeyCode::Backspace => {
                        if !app.current_header_value.is_empty() {
                            app.current_header_value.pop();
                        } else if !app.current_header_key.is_empty() {
                            app.current_header_key.pop();
                        }
                    }
                    KeyCode::Esc => {
                        app.current_header_key.clear();
                        app.current_header_value.clear();
                        app.current_screen = CurrentScreen::Values;
                    }
                    KeyCode::Char(':') => {
                        if !app.current_header_key.is_empty() && app.current_header_value.is_empty() {
                            // Switch to value input after ':'
                        }
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
                    }
                    KeyCode::Char(c) => {
                        if !app.current_header_key.contains(':') {
                            app.current_header_key.push(c);
                        } else {
                            app.current_header_value.push(c);
                        }
                    }
                    _ => {}
                },
                CurrentScreen::EditingParams => match key.code {
                    KeyCode::Enter => {
                        if !app.current_param_key.is_empty() {
                            if let Err(e) = app.add_param() {
                                error_message = Some(app.get_error_message(&e));
                            }
                        } else {
                            app.current_screen = CurrentScreen::Values;
                        }
                    }
                    KeyCode::Tab => {
                        // Switch focus between key and value (simplified for now)
                        if !app.current_param_key.is_empty() && app.current_param_value.is_empty() {
                            app.current_param_value.push(' '); // Start value input
                            app.current_param_value.clear();
                        }
                    }
                    KeyCode::Backspace => {
                        if !app.current_param_value.is_empty() {
                            app.current_param_value.pop();
                        } else if !app.current_param_key.is_empty() {
                            app.current_param_key.pop();
                        }
                    }
                    KeyCode::Esc => {
                        app.current_param_key.clear();
                        app.current_param_value.clear();
                        app.current_screen = CurrentScreen::Values;
                    }
                    KeyCode::Char('=') => {
                        if !app.current_param_key.is_empty() && app.current_param_value.is_empty() {
                            // Switch to value input after '='
                        }
                    }
                    KeyCode::Char(c) => {
                        if !app.current_param_key.contains('=') {
                            app.current_param_key.push(c);
                        } else {
                            app.current_param_value.push(c);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
