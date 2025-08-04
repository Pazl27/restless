use std::io;
use anyhow::Result;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

mod app;
use app::{App, CurrentScreen};

mod ui;
use ui::ui;

mod logic;
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
                        match key.code {
                            // navigation between tabs
                            KeyCode::Tab => {
                                app.next_tab();
                            }
                            KeyCode::BackTab => {
                                app.prev_tab();
                            }
                            KeyCode::Enter => {
                                let response = app.tabs[app.selected_tab].request.send().await?;
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
                            KeyCode::Char('u') => {
                                if let CurrentScreen::Url = app.current_screen {
                                    app.current_screen = CurrentScreen::Editing;
                                }
                            }
                            _ => {}
                        }
                    }
                },

                CurrentScreen::Editing => match key.code {
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
                _ => {}
            }
        }
    }
}
