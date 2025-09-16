use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
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
use crate::logic::response::Response;
use crate::logic::HttpMethod;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run_app(&mut terminal, &mut app).await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
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
                        if let KeyCode::Char('u') = key.code {
                            app.current_screen = CurrentScreen::EditingUrl;
                        } else {
                            match app.current_screen {
                                CurrentScreen::Response => match key.code {
                                    KeyCode::Left | KeyCode::Char('h') => {
                                        app.response_tab_selected = 0;
                                    }
                                    KeyCode::Right | KeyCode::Char('b') => {
                                        app.response_tab_selected = 1;
                                    }
                                    KeyCode::Up => {
                                        app.current_screen = CurrentScreen::Values;
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
                                    _ => {}
                                },
                                CurrentScreen::Values => match key.code {
                                    KeyCode::Down => {
                                        app.current_screen = CurrentScreen::Response;
                                    }
                                    KeyCode::Up => {
                                        app.current_screen = CurrentScreen::Url;
                                    }
                                    KeyCode::Left | KeyCode::Char('h') => {
                                        app.values_screen = match app.values_screen {
                                            ValuesScreen::Headers => ValuesScreen::Body,
                                            ValuesScreen::Params => ValuesScreen::Headers,
                                            _ => app.values_screen,
                                        };
                                    }
                                    KeyCode::Right | KeyCode::Char('l') => {
                                        app.values_screen = match app.values_screen {
                                            ValuesScreen::Body => ValuesScreen::Headers,
                                            ValuesScreen::Headers => ValuesScreen::Params,
                                            _ => app.values_screen,
                                        };
                                    }
                                    KeyCode::Char('e') => {
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
                                    _ => {}
                                },
                                CurrentScreen::Url => match key.code {
                                    KeyCode::Down => {
                                        app.current_screen = CurrentScreen::Values;
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }

                            match key.code {
                                KeyCode::Tab => {
                                    app.next_tab();
                                }
                                KeyCode::BackTab => {
                                    app.prev_tab();
                                }
                                KeyCode::Enter => {
                                    let (status_code, headers, body) =
                                        app.tabs[app.selected_tab].request.send().await?;
                                    let response = Response::new(status_code, headers, body);
                                    app.tabs[app.selected_tab].response = Some(response);
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
                                    return Ok(true);
                                }
                                _ => {}
                            }
                        }
                    }
                }

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
                            app.add_header();
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
                            app.add_param();
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
