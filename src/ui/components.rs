//! Individual UI components for the Restless application
//!
//! This module contains the rendering logic for individual UI components,
//! each focused on a specific part of the interface. This modular approach
//! makes the code more maintainable and testable.

use ratatui::{
    layout::{Alignment, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, Tabs,
    },
    Frame,
};

use super::{
    create_block, create_response_layout, create_url_layout, create_values_layout, method_text,
    truncate_text, TEXT_COLOR_HIGHLIGHT, TEXT_COLOR_MUTED,
};
use crate::app::{App, CurrentScreen, ValuesScreen};

/// Renders the tab bar at the top of the application
pub fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL);

    let tab_titles: Vec<Line> = app
        .tabs
        .iter()
        .map(|tab| Line::from(tab.name.clone()))
        .collect();

    let tabs_widget = Tabs::new(tab_titles)
        .block(block)
        .select(app.selected_tab)
        .highlight_style(Style::default().fg(TEXT_COLOR_HIGHLIGHT));

    f.render_widget(tabs_widget, area);
}

/// Renders the URL input section with method selector
pub fn render_url_input(f: &mut Frame, app: &App, area: Rect) {
    let (method_area, url_area) = create_url_layout(area);

    // Render method selector
    render_method_selector(f, app, method_area);

    // Render URL input field
    render_url_field(f, app, url_area);

    // Render method dropdown if open
    if app.method_dropdown_open {
        render_method_dropdown(f, app, method_area);
    }

    // Set cursor position when editing URL
    if let CurrentScreen::EditingUrl = app.current_screen {
        let cursor_x = url_area.x + 6 + app.url_input.len() as u16; // "URL: " = 5 chars + space
        let cursor_y = url_area.y + 1;
        f.set_cursor_position(Position {
            x: cursor_x,
            y: cursor_y,
        });
    }
}

/// Renders the HTTP method selector
fn render_method_selector(f: &mut Frame, app: &App, area: Rect) {
    let is_active = matches!(app.current_screen, CurrentScreen::Url);
    let block = create_block("Method", is_active, false);

    let method_paragraph = Paragraph::new(method_text(&app.selected_method))
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(method_paragraph, area);
}

/// Renders the URL input field
fn render_url_field(f: &mut Frame, app: &App, area: Rect) {
    let is_active = matches!(app.current_screen, CurrentScreen::Url);
    let is_editing = matches!(app.current_screen, CurrentScreen::EditingUrl);
    let block = create_block("URL", is_active, is_editing);

    let url_text = if app.url_input.is_empty() && !is_editing {
        "Enter URL (press 'u' to edit)".to_string()
    } else {
        format!("URL: {}", app.url_input)
    };

    let url_paragraph = Paragraph::new(url_text).block(block);
    f.render_widget(url_paragraph, area);
}

/// Renders the method dropdown menu
fn render_method_dropdown(f: &mut Frame, app: &App, method_area: Rect) {
    let methods = ["GET", "POST", "PUT", "DELETE"];
    let method_colors = [Color::Green, Color::Blue, Color::Yellow, Color::Red];

    let dropdown_area = Rect {
        x: method_area.x,
        y: method_area.y + method_area.height,
        width: method_area.width,
        height: methods.len() as u16 + 2,
    };

    // Clear background
    f.render_widget(Clear, dropdown_area);

    // Render dropdown container
    let dropdown_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().bg(Color::Rgb(30, 30, 30)));
    f.render_widget(dropdown_block, dropdown_area);

    // Render method options
    for (i, method) in methods.iter().enumerate() {
        let is_selected = i == app.method_dropdown_selected;
        let bg_color = if is_selected {
            Color::Rgb(60, 60, 60)
        } else {
            Color::Rgb(30, 30, 30)
        };

        let item_area = Rect {
            x: dropdown_area.x + 1,
            y: dropdown_area.y + 1 + i as u16,
            width: dropdown_area.width.saturating_sub(2),
            height: 1,
        };

        let method_span = Span::styled(
            *method,
            Style::default()
                .fg(method_colors[i])
                .bg(bg_color)
                .add_modifier(if is_selected {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        );

        let method_paragraph = Paragraph::new(method_span).alignment(Alignment::Center);
        f.render_widget(method_paragraph, item_area);
    }
}

/// Renders the values section (Body/Headers/Params)
pub fn render_values_section(f: &mut Frame, app: &App, area: Rect) {
    let (tabs_area, content_area) = create_values_layout(area);

    // Render values tabs
    render_values_tabs(f, app, tabs_area);

    // Render content based on selected tab
    match app.values_screen {
        ValuesScreen::Body => render_body_content(f, app, content_area),
        ValuesScreen::Headers => render_headers_content(f, app, content_area),
        ValuesScreen::Params => render_params_content(f, app, content_area),
    }
}

/// Renders the tabs for the values section
fn render_values_tabs(f: &mut Frame, app: &App, area: Rect) {
    let tab_titles = vec![
        Line::from("Body"),
        Line::from("Headers"),
        Line::from("Params"),
    ];

    let selected_tab = match app.values_screen {
        ValuesScreen::Body => 0,
        ValuesScreen::Headers => 1,
        ValuesScreen::Params => 2,
    };

    let tabs = Tabs::new(tab_titles)
        .select(selected_tab)
        .highlight_style(Style::default().fg(TEXT_COLOR_HIGHLIGHT))
        .divider(" ")
        .padding("", "");

    f.render_widget(tabs, area);
}

/// Renders the body content area
fn render_body_content(f: &mut Frame, app: &App, area: Rect) {
    let is_active = matches!(app.current_screen, CurrentScreen::Values)
        && matches!(app.values_screen, ValuesScreen::Body);
    let is_editing = matches!(app.current_screen, CurrentScreen::EditingBody);
    let block = create_block("Request Body", is_active, is_editing);

    let content = if app.body_input.is_empty() {
        if is_active && !is_editing {
            "Press 'i' to edit body...\n\nTip: Use JSON, XML, or plain text\nNavigation: Ctrl+j/k between sections, h/l for tabs".to_string()
        } else {
            "Body (empty)".to_string()
        }
    } else {
        app.body_input.clone()
    };

    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, area);

    // Set cursor position when editing
    if is_editing {
        let lines: Vec<&str> = app.body_input.lines().collect();
        let last_line = lines.last().unwrap_or(&"");
        let cursor_y = area.y + 1 + lines.len().saturating_sub(1) as u16;
        let cursor_x = area.x + 1 + last_line.len() as u16;
        f.set_cursor_position(Position {
            x: cursor_x,
            y: cursor_y,
        });
    }
}

/// Renders the headers content area
fn render_headers_content(f: &mut Frame, app: &App, area: Rect) {
    let is_active = matches!(app.current_screen, CurrentScreen::Values)
        && matches!(app.values_screen, ValuesScreen::Headers);
    let is_editing = matches!(app.current_screen, CurrentScreen::EditingHeaders);
    let block = create_block("Headers", is_active, is_editing);

    let mut items: Vec<ListItem> = app
        .headers_input
        .iter()
        .map(|(key, value)| ListItem::new(Line::from(format!("{}: {}", key, value))))
        .collect();

    // Add current input line if editing
    if is_editing {
        let current_input = if app.current_header_value.is_empty() {
            format!("{}:", app.current_header_key)
        } else {
            format!("{}: {}", app.current_header_key, app.current_header_value)
        };
        items.push(ListItem::new(Line::from(Span::styled(
            current_input,
            Style::default().fg(TEXT_COLOR_HIGHLIGHT),
        ))));
    } else if items.is_empty() {
        if is_active {
            items.push(ListItem::new(Line::from("Press 'i' to add headers...")));
            items.push(ListItem::new(Line::from("Format: Key: Value")));
            items.push(ListItem::new(Line::from(
                "Example: Content-Type: application/json",
            )));
            items.push(ListItem::new(Line::from("Use h/l to switch tabs")));
        } else {
            items.push(ListItem::new(Line::from("No headers")));
        }
    }

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

/// Renders the parameters content area
fn render_params_content(f: &mut Frame, app: &App, area: Rect) {
    let is_active = matches!(app.current_screen, CurrentScreen::Values)
        && matches!(app.values_screen, ValuesScreen::Params);
    let is_editing = matches!(app.current_screen, CurrentScreen::EditingParams);
    let block = create_block("Query Parameters", is_active, is_editing);

    let mut items: Vec<ListItem> = app
        .params_input
        .iter()
        .map(|(key, value)| ListItem::new(Line::from(format!("{}={}", key, value))))
        .collect();

    // Add current input line if editing
    if is_editing {
        let current_input = if app.current_param_value.is_empty() {
            format!("{}=", app.current_param_key)
        } else {
            format!("{}={}", app.current_param_key, app.current_param_value)
        };
        items.push(ListItem::new(Line::from(Span::styled(
            current_input,
            Style::default().fg(TEXT_COLOR_HIGHLIGHT),
        ))));
    } else if items.is_empty() {
        if is_active {
            items.push(ListItem::new(Line::from("Press 'i' to add parameters...")));
            items.push(ListItem::new(Line::from("Format: key=value")));
            items.push(ListItem::new(Line::from("Example: limit=10")));
            items.push(ListItem::new(Line::from("Use h/l to switch tabs")));
        } else {
            items.push(ListItem::new(Line::from("No parameters")));
        }
    }

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

/// Renders the response section
pub fn render_response_section(f: &mut Frame, app: &App, area: Rect) {
    let tab = &app.tabs[app.selected_tab];

    if let Some(response) = &tab.response {
        let (tabs_area, content_area) = create_response_layout(area);

        // Render response tabs
        render_response_tabs(f, app, tabs_area);

        // Render response content
        render_response_content(f, app, response, content_area);
    } else {
        render_empty_response(f, app, area);
    }
}

/// Renders the response tabs (Headers/Body)
fn render_response_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = [Line::from("Headers"), Line::from("Body")];
    let tabs = Tabs::new(titles)
        .select(app.response_tab_selected)
        .highlight_style(Style::default().fg(TEXT_COLOR_HIGHLIGHT))
        .divider(" ")
        .padding("", "");
    f.render_widget(tabs, area);
}

/// Renders the response content with scrolling
fn render_response_content(
    f: &mut Frame,
    app: &App,
    response: &crate::logic::response::Response,
    area: Rect,
) {
    let is_active = matches!(app.current_screen, CurrentScreen::Response);

    // Status code in title
    let title = format!("Response - Status: {}", response.status_code);
    let block = create_block(&title, is_active, false);

    // Select content based on active tab
    let content: Vec<Line> = if app.response_tab_selected == 0 {
        // Headers
        if response.headers.is_empty() {
            vec![Line::from("No headers")]
        } else {
            response
                .headers
                .iter()
                .map(|(k, v)| Line::from(format!("{}: {}", k, v)))
                .collect()
        }
    } else {
        // Body
        response
            .body
            .lines()
            .map(|line| Line::from(line.to_string()))
            .collect()
    };

    let scroll_offset = app.response_scroll as u16;
    let paragraph = Paragraph::new(content)
        .block(block)
        .scroll((scroll_offset, 0));
    f.render_widget(paragraph, area);

    // Render scrollbar for body content
    if app.response_tab_selected == 1 && !response.body.is_empty() {
        let content_height = response.body.lines().count();
        let mut scroll_state = app
            .response_scroll_state
            .clone()
            .content_length(content_height);

        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut scroll_state,
        );
    }
}

/// Renders empty response placeholder
fn render_empty_response(f: &mut Frame, app: &App, area: Rect) {
    let is_active = matches!(app.current_screen, CurrentScreen::Response);
    let block = create_block("Response", is_active, false);

    let help_text = if matches!(app.current_screen, CurrentScreen::Response) {
        "No response yet.\n\nPress Enter to send request\nPress ? for help"
    } else {
        "No response yet."
    };

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

/// Renders the status bar at the bottom
pub fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let help_text = "Press ? for help | Enter: Send Request | q: Quit";

    // Show current tab info if multiple tabs
    let tab_info = if app.tabs.len() > 1 {
        format!(" | Tab {}/{}", app.selected_tab + 1, app.tabs.len())
    } else {
        String::new()
    };

    // Show current screen info
    let screen_info = match app.current_screen {
        CurrentScreen::EditingUrl => " | Editing URL",
        CurrentScreen::EditingBody => " | Editing Body",
        CurrentScreen::EditingHeaders => " | Editing Headers",
        CurrentScreen::EditingParams => " | Editing Params",
        CurrentScreen::Help => " | Help",
        _ => "",
    };

    let status_text = format!("{}{}{}", help_text, tab_info, screen_info);
    let truncated_text = truncate_text(&status_text, area.width.saturating_sub(4) as usize);

    let status_paragraph = Paragraph::new(truncated_text)
        .style(Style::default().fg(TEXT_COLOR_MUTED))
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(status_paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    fn create_test_app() -> App {
        App::new()
    }

    #[test]
    fn test_render_tabs() {
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = create_test_app();

        terminal
            .draw(|f| {
                render_tabs(f, &app, f.area());
            })
            .unwrap();
    }

    #[test]
    fn test_render_url_input() {
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = create_test_app();

        terminal
            .draw(|f| {
                render_url_input(f, &app, f.area());
            })
            .unwrap();
    }

    #[test]
    fn test_render_values_section() {
        let backend = TestBackend::new(80, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = create_test_app();

        terminal
            .draw(|f| {
                render_values_section(f, &app, f.area());
            })
            .unwrap();
    }

    #[test]
    fn test_render_response_section() {
        let backend = TestBackend::new(80, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = create_test_app();

        terminal
            .draw(|f| {
                render_response_section(f, &app, f.area());
            })
            .unwrap();
    }

    #[test]
    fn test_render_status_bar() {
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = create_test_app();

        terminal
            .draw(|f| {
                render_status_bar(f, &app, f.area());
            })
            .unwrap();
    }
}
