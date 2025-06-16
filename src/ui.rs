use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Tabs, Clear},
    Frame
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
    let block = Block::default().borders(Borders::ALL);

    let tabs: Vec<_> = app.tabs.iter().map(|tab| tab.name.clone()).collect();
    let tabs_widget = Tabs::new(tabs)
        .block(block)
        .select(app.selected_tab)
        .highlight_style(Style::default().fg(Color::Yellow));
    f.render_widget(tabs_widget, area);
}

fn render_url_input(f: &mut Frame, app: &App, area: Rect) {
    render_url_input_box(f, app, area);

    if let crate::app::CurrentScreen::Editing = app.current_screen {
        let x = area.x + 16 + app.url_input.len() as u16;
        let y = area.y + 1;
        let pos = Position { x, y };
        f.set_cursor_position(pos);
    }
}

fn render_url_input_box(f: &mut Frame, app: &App, area: Rect) {
    use crate::app::HttpMethod;

    let url = &app.url_input;
    let mut block = Block::default().borders(Borders::ALL);

    if let crate::app::CurrentScreen::Url = app.current_screen {
        block = block.border_style(Style::default().fg(Color::Green));
    }

    // Draw method box
    let method_str = match app.selected_method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE",
    };
    let method_color = match app.selected_method {
        HttpMethod::Get => Color::Green,
        HttpMethod::Post => Color::Blue,
        HttpMethod::Put => Color::Yellow,
        HttpMethod::Delete => Color::Red,
    };

    let method_block = Block::default().borders(Borders::ALL);
    let method_paragraph = Paragraph::new(Span::styled(method_str, Style::default().fg(method_color)))
        .style(Style::default())
        .block(method_block.title("Method"));

    // Layout: [Method][URL]
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
        .split(area);

    f.render_widget(method_paragraph, layout[0]);
    let url_input = Paragraph::new(format!("URL: {}", url)).block(block);
    f.render_widget(url_input, layout[1]);

    // Draw dropdown if open
    if app.method_dropdown_open {
        let methods = ["GET", "POST", "PUT", "DELETE"];
        let colors = [Color::Green, Color::Blue, Color::Yellow, Color::Red];
        let dropdown_height = methods.len() as u16;

        // Dropdown area (directly below the method box)
        let dropdown_area = ratatui::layout::Rect {
            x: layout[0].x,
            y: layout[0].y + 1,
            width: layout[0].width,
            height: dropdown_height + 2,
        };

        f.render_widget(Clear, dropdown_area);

        let dropdown_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Rgb(30,30,30)));
        f.render_widget(dropdown_block, dropdown_area);

        for (i, method) in methods.iter().enumerate() {
            let is_selected = i == app.method_dropdown_selected;
            let bg = if is_selected { Color::Rgb(60,60,60) } else { Color::Rgb(30,30,30) };
            let item_area = ratatui::layout::Rect {
                x: dropdown_area.x + 1,
                y: dropdown_area.y + 1 + i as u16,
                width: dropdown_area.width - 2,
                height: 1,
            };
            let item_paragraph = Paragraph::new(Span::styled(*method, Style::default().fg(colors[i])))
                .style(Style::default().bg(bg))
                .block(Block::default());
            f.render_widget(item_paragraph, item_area);
        }
    }
}

fn render_params_input(f: &mut Frame, _app: &App, area: Rect) {
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
