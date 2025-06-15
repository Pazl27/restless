use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect}, style::{Color, Style}, widgets::{Block, Borders, Paragraph, Tabs}, Frame
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)].as_ref())
        .split(f.area());

    render_nav(f, chunks[0]);
    render_content(f, app, chunks[1]);
}

fn render_nav(f: &mut Frame, area: Rect) {
    let nav_block = Block::default().title("Navigation").borders(Borders::ALL);
    f.render_widget(nav_block, area);
}

fn render_content(f: &mut Frame, app: &mut App, area: Rect) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Length(3), // URL input
            Constraint::Min(5),    // Params/body/headers input
            Constraint::Min(0),    // Response output
        ].as_ref())
        .split(area);

    render_tabs(f, app, content_chunks[0]);
    render_url_input(f, app, content_chunks[1]);
    render_params_input(f, app, content_chunks[2]);
    render_response_output(f,app, content_chunks[3]);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let mut block = Block::default().borders(Borders::ALL);
    if let crate::app::CurrentScreen::Main = app.current_screen {
        block = block.border_style(Style::default().fg(Color::Green));
    }

    let tabs: Vec<_> = app.tabs.iter().map(|tab| tab.name.clone()).collect();
    let tabs_widget = Tabs::new(tabs)
        .block(block)
        .select(app.selected_tab)
        .highlight_style(Style::default().fg(Color::Yellow));
    f.render_widget(tabs_widget, area);
}

fn render_url_input(f: &mut Frame, app: &App, area: Rect) {
    let url = &app.url_input;
    let mut block = Block::default().borders(Borders::ALL);

    if let crate::app::CurrentScreen::Url = app.current_screen {
        block = block.border_style(Style::default().fg(Color::Green));
    }

    let url_input = Paragraph::new(format!("URL: {}", url)).block(block);
    f.render_widget(url_input, area);

    if let crate::app::CurrentScreen::Editing = app.current_screen {
        let x = area.x + 5 + url.len() as u16;
        let y = area.y + 1;
        let pos = Position { x, y };
        f.set_cursor_position(pos);
    }

}

fn render_params_input(f: &mut Frame, _app: &App, area: Rect) {
    // green if selected, otherwise default
    let mut block = Block::default().borders(Borders::ALL);

    if let crate::app::CurrentScreen::Values = _app.current_screen {
        block = block.border_style(Style::default().fg(Color::Green));
    }

    let params_input = Paragraph::new("Params/Body/Headers input...")
        .block(block);
    f.render_widget(params_input, area);
}

fn render_response_output(f: &mut Frame, _app: &App, area: Rect) {
    let mut block = Block::default().borders(Borders::ALL);

    if let crate::app::CurrentScreen::Response = _app.current_screen {
        block = block.border_style(Style::default().fg(Color::Green));
    }

    let response_output = Paragraph::new("Response output...")
        .block(block);
    f.render_widget(response_output, area);
}
