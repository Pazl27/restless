use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, Tabs},
    Frame,
};

use crate::app::{App, ValuesScreen};

pub fn ui(f: &mut Frame, app: &mut App) {
    render_content(f, app, f.area());
}

fn render_content(f: &mut Frame, app: &mut App, area: Rect) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Tabs
                Constraint::Length(3), // URL input
                Constraint::Min(5),    // Params/body/headers input
                Constraint::Min(0),    // Response output
                Constraint::Length(2), // Help bar
            ]
            .as_ref(),
        )
        .split(area);

    render_help_bar(f, app, content_chunks[4]);
    render_response_output(f, app, content_chunks[3]);
    render_params_input(f, app, content_chunks[2]);
    render_url_input(f, app, content_chunks[1]);
    render_tabs(f, app, content_chunks[0]);
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

    if let crate::app::CurrentScreen::EditingUrl = app.current_screen {
        let x = area.x + 16 + app.url_input.len() as u16;
        let y = area.y + 1;
        let pos = Position { x, y };
        f.set_cursor_position(pos);
    }
}

fn render_url_input_box(f: &mut Frame, app: &App, area: Rect) {
    use crate::logic::HttpMethod;

    let url = &app.url_input;
    let mut block = Block::default().borders(Borders::ALL);

    if let crate::app::CurrentScreen::Url = app.current_screen {
        block = block.border_style(Style::default().fg(Color::Green));
    }

    // Draw method box
    let method_str = match app.selected_method {
        HttpMethod::GET => "GET",
        HttpMethod::POST => "POST",
        HttpMethod::PUT => "PUT",
        HttpMethod::DELETE => "DELETE",
    };
    let method_color = match app.selected_method {
        HttpMethod::GET => Color::Green,
        HttpMethod::POST => Color::Blue,
        HttpMethod::PUT => Color::Yellow,
        HttpMethod::DELETE => Color::Red,
    };

    let method_block = Block::default().borders(Borders::ALL);
    let method_paragraph =
        Paragraph::new(Span::styled(method_str, Style::default().fg(method_color)))
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
            .style(Style::default().bg(Color::Rgb(30, 30, 30)));
        f.render_widget(dropdown_block, dropdown_area);

        for (i, method) in methods.iter().enumerate() {
            let is_selected = i == app.method_dropdown_selected;
            let bg = if is_selected {
                Color::Rgb(60, 60, 60)
            } else {
                Color::Rgb(30, 30, 30)
            };
            let item_area = ratatui::layout::Rect {
                x: dropdown_area.x + 1,
                y: dropdown_area.y + 1 + i as u16,
                width: dropdown_area.width - 2,
                height: 1,
            };
            let item_paragraph =
                Paragraph::new(Span::styled(*method, Style::default().fg(colors[i])))
                    .style(Style::default().bg(bg))
                    .block(Block::default());
            f.render_widget(item_paragraph, item_area);
        }
    }
}

fn render_params_input(f: &mut Frame, app: &App, area: Rect) {
    // Create tabs for Body, Headers, Params
    let tabs_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: 3,
    };

    let content_area = Rect {
        x: area.x,
        y: area.y + 3,
        width: area.width,
        height: area.height.saturating_sub(3),
    };

    // Render tabs
    let titles = vec![
        Line::from("Body"),
        Line::from("Headers"),
        Line::from("Params")
    ];

    let selected_tab = match app.values_screen {
        ValuesScreen::Body => 0,
        ValuesScreen::Headers => 1,
        ValuesScreen::Params => 2,
    };

    let tabs = Tabs::new(titles)
        .select(selected_tab)
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(" ")
        .padding("", "");

    f.render_widget(tabs, tabs_area);

    // Render content based on selected tab
    match app.values_screen {
        ValuesScreen::Body => render_body_input(f, app, content_area),
        ValuesScreen::Headers => render_headers_input(f, app, content_area),
        ValuesScreen::Params => render_params_input_content(f, app, content_area),
    }
}

fn render_body_input(f: &mut Frame, app: &App, area: Rect) {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title("Request Body");

    if let crate::app::CurrentScreen::Values = app.current_screen {
        if let ValuesScreen::Body = app.values_screen {
            block = block.border_style(Style::default().fg(Color::Green));
        }
    }

    if let crate::app::CurrentScreen::EditingBody = app.current_screen {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    let content = if app.body_input.is_empty() {
        if let crate::app::CurrentScreen::Values = app.current_screen {
            if let ValuesScreen::Body = app.values_screen {
                "Press 'i' to edit body...\n\nTip: Use JSON, XML, or plain text\nNavigation: Ctrl+j/k between sections, h/l for tabs".to_string()
            } else {
                "Body (empty)".to_string()
            }
        } else {
            "Body (empty)".to_string()
        }
    } else {
        app.body_input.clone()
    };

    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, area);

    // Set cursor position when editing
    if let crate::app::CurrentScreen::EditingBody = app.current_screen {
        let lines: Vec<&str> = app.body_input.lines().collect();
        let last_line = lines.last().unwrap_or(&"");
        let cursor_y = area.y + 1 + lines.len().saturating_sub(1) as u16;
        let cursor_x = area.x + 1 + last_line.len() as u16;
        f.set_cursor_position(Position { x: cursor_x, y: cursor_y });
    }
}

fn render_headers_input(f: &mut Frame, app: &App, area: Rect) {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title("Headers");

    if let crate::app::CurrentScreen::Values = app.current_screen {
        if let ValuesScreen::Headers = app.values_screen {
            block = block.border_style(Style::default().fg(Color::Green));
        }
    }

    if let crate::app::CurrentScreen::EditingHeaders = app.current_screen {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    // Create list items for existing headers
    let mut items: Vec<ListItem> = app.headers_input
        .iter()
        .map(|(key, value)| {
            ListItem::new(Line::from(format!("{}: {}", key, value)))
        })
        .collect();

    // Add current input line if editing
    if let crate::app::CurrentScreen::EditingHeaders = app.current_screen {
        let current_input = if app.current_header_value.is_empty() {
            format!("{}:", app.current_header_key)
        } else {
            format!("{}: {}", app.current_header_key, app.current_header_value)
        };
        items.push(ListItem::new(Line::from(Span::styled(
            current_input,
            Style::default().fg(Color::Yellow)
        ))));
    } else if items.is_empty() {
        if let crate::app::CurrentScreen::Values = app.current_screen {
            if let ValuesScreen::Headers = app.values_screen {
                items.push(ListItem::new(Line::from("Press 'i' to add headers...")));
                items.push(ListItem::new(Line::from("Format: Key: Value")));
                items.push(ListItem::new(Line::from("Example: Content-Type: application/json")));
                items.push(ListItem::new(Line::from("Use h/l to switch tabs")));
            } else {
                items.push(ListItem::new(Line::from("No headers")));
            }
        } else {
            items.push(ListItem::new(Line::from("No headers")));
        }
    }

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn render_params_input_content(f: &mut Frame, app: &App, area: Rect) {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title("Query Parameters");

    if let crate::app::CurrentScreen::Values = app.current_screen {
        if let ValuesScreen::Params = app.values_screen {
            block = block.border_style(Style::default().fg(Color::Green));
        }
    }

    if let crate::app::CurrentScreen::EditingParams = app.current_screen {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }

    // Create list items for existing params
    let mut items: Vec<ListItem> = app.params_input
        .iter()
        .map(|(key, value)| {
            ListItem::new(Line::from(format!("{}={}", key, value)))
        })
        .collect();

    // Add current input line if editing
    if let crate::app::CurrentScreen::EditingParams = app.current_screen {
        let current_input = if app.current_param_value.is_empty() {
            format!("{}=", app.current_param_key)
        } else {
            format!("{}={}", app.current_param_key, app.current_param_value)
        };
        items.push(ListItem::new(Line::from(Span::styled(
            current_input,
            Style::default().fg(Color::Yellow)
        ))));
    } else if items.is_empty() {
        if let crate::app::CurrentScreen::Values = app.current_screen {
            if let ValuesScreen::Params = app.values_screen {
                items.push(ListItem::new(Line::from("Press 'i' to add parameters...")));
                items.push(ListItem::new(Line::from("Format: key=value")));
                items.push(ListItem::new(Line::from("Example: limit=10")));
                items.push(ListItem::new(Line::from("Use h/l to switch tabs")));
            } else {
                items.push(ListItem::new(Line::from("No parameters")));
            }
        } else {
            items.push(ListItem::new(Line::from("No parameters")));
        }
    }

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn render_help_bar(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.current_screen {
        crate::app::CurrentScreen::Url => {
            "u: Edit URL | m: Method | Tab: Switch request tabs | Ctrl+j/k: Navigate sections | Enter: Execute | q: Quit"
        }
        crate::app::CurrentScreen::Values => {
            match app.values_screen {
                ValuesScreen::Body => "i: Insert body | h/l: Switch tabs | Tab: Switch request tabs | Ctrl+j/k: Navigate sections | Enter: Execute",
                ValuesScreen::Headers => "i: Insert header | h/l: Switch tabs | Tab: Switch request tabs | Ctrl+j/k: Navigate sections | Enter: Execute",
                ValuesScreen::Params => "i: Insert param | h/l: Switch tabs | Tab: Switch request tabs | Ctrl+j/k: Navigate sections | Enter: Execute",
            }
        }
        crate::app::CurrentScreen::Response => {
            "j/k: Scroll | h/b: Headers/Body | Tab: Switch request tabs | Ctrl+j/k: Navigate sections | Enter: Execute | q: Quit"
        }
        crate::app::CurrentScreen::EditingUrl => {
            "Type URL | Enter: Save | Esc: Cancel"
        }
        crate::app::CurrentScreen::EditingBody => {
            "Type body content | Enter: New line | Esc: Finish editing"
        }
        crate::app::CurrentScreen::EditingHeaders => {
            "Format: Key: Value | Enter: Add header | Esc: Finish editing"
        }
        crate::app::CurrentScreen::EditingParams => {
            "Format: key=value | Enter: Add param | Esc: Finish editing"
        }
        _ => "q: Quit application"
    };

    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(help_paragraph, area);
}

fn render_response_output(f: &mut Frame, app: &App, area: Rect) {
    let tab = &app.tabs[app.selected_tab];

    if let Some(response) = &tab.response {
        // Layout: [Tabs][Response Box]
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(area);

        // Tabs: Headers/Body (above the box)
        let titles = [Line::from("Headers"), Line::from("Body")];
        let tabs = Tabs::new(titles)
            .select(app.response_tab_selected)
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(" ")
            .padding("", "");
        f.render_widget(tabs, chunks[0]);

        // Status code in top right
        let status_span = Span::styled(
            format!("Status: {}", response.status_code),
            Style::default().fg(Color::Green),
        );

        // If selected, make the box border green
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Right)
            .title(status_span);

        if let crate::app::CurrentScreen::Response = app.current_screen {
            block = block.border_style(Style::default().fg(Color::Green));
        }

        // Select content for tab
        let content: Vec<Line> = if app.response_tab_selected == 0 {
            // Headers
            if response.headers.is_empty() {
                vec![Line::from("No headers")]
            } else {
                response
                    .headers
                    .iter()
                    .map(|(k, v)| Line::from(Span::raw(format!("{}: {}", k, v))))
                    .collect()
            }
        } else {
            // Body
            response
                .body
                .lines()
                .map(|l| Line::from(Span::raw(l.to_string())))
                .collect()
        };

        // For scrolling, you may want to add a scroll offset to App (e.g., app.response_scroll)
        let scroll = app.response_scroll as u16;
        let content_height = content.len();

        // Update scrollbar state
        let mut scroll_state = app
            .response_scroll_state
            .clone()
            .content_length(content_height);

        // Scrollable paragraph
        let paragraph = Paragraph::new(content).block(block).scroll((scroll, 0));
        f.render_widget(paragraph, chunks[1]);

        // Draw vertical scrollbar
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            chunks[1],
            &mut scroll_state,
        );
    } else {
        // If no response, render a disabled box
        let mut block = Block::default().borders(Borders::ALL).title("Response");
        if let crate::app::CurrentScreen::Response = app.current_screen {
            block = block.border_style(Style::default().fg(Color::Green));
        }
        let paragraph = Paragraph::new(vec![Line::from("No response yet.")]).block(block);
        f.render_widget(paragraph, area);
    }
}
