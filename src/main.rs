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

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main | CurrentScreen::Url | CurrentScreen::Values | CurrentScreen::Response => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Tab => {
                        app.tabs[app.selected_tab].url = app.url_input.clone();
                        app.selected_tab = (app.selected_tab + 1) % app.tabs.len();
                        app.url_input = app.tabs[app.selected_tab].url.clone();
                    }
                    KeyCode::Char('{') => {
                        app.current_screen = match app.current_screen {
                            CurrentScreen::Url => CurrentScreen::Values,
                            CurrentScreen::Values => CurrentScreen::Response,
                            CurrentScreen::Response => CurrentScreen::Url,
                            _ => CurrentScreen::Url,
                        };
                    }
                    KeyCode::Char('}') => {
                        app.current_screen = match app.current_screen {
                            CurrentScreen::Url => CurrentScreen::Response,
                            CurrentScreen::Values => CurrentScreen::Url,
                            CurrentScreen::Response => CurrentScreen::Values,
                            _ => CurrentScreen::Url,
                        };
                    }
                    KeyCode::Char('i') => {
                        if let CurrentScreen::Url = app.current_screen {
                            app.current_screen = CurrentScreen::Editing;
                        }
                    }
                    _ => {}
                },

                CurrentScreen::Editing => match key.code {
                    KeyCode::Enter => {
                        app.tabs[app.selected_tab].url = app.url_input.clone();
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
