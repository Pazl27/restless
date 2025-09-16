//! Popup components for modal dialogs and overlays
//! 
//! This module contains popup windows that appear over the main UI,
//! including help screens, error dialogs, and confirmation dialogs.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use super::{
    create_error_block, create_popup_layout, create_fixed_popup_layout,
    TEXT_COLOR_HIGHLIGHT, TEXT_COLOR_MUTED, TEXT_COLOR_NORMAL,
};

/// Renders the help popup with key bindings and navigation help
pub fn render_help_popup(f: &mut Frame, app: &App) {
    // Calculate popup area (80% of screen width, 80% of height)
    let popup_area = create_popup_layout(f.area(), 80, 80);

    // Clear the background
    f.render_widget(Clear, popup_area);

    // Create help content
    let help_items = app.get_help_content();
    let mut lines = Vec::new();
    
    for (key, description) in help_items.iter().skip(app.help_scroll) {
        if key.is_empty() && description.is_empty() {
            lines.push(Line::from(""));
        } else if description.is_empty() {
            // Section header
            lines.push(Line::from(Span::styled(
                key.to_string(),
                Style::default()
                    .fg(TEXT_COLOR_HIGHLIGHT)
                    .add_modifier(Modifier::BOLD)
            )));
        } else {
            // Key binding
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:15}", key),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                ),
                Span::raw(" "),
                Span::styled(
                    description.to_string(),
                    Style::default().fg(TEXT_COLOR_NORMAL)
                ),
            ]));
        }
    }

    let help_block = Block::default()
        .title(" Restless - Key Bindings ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TEXT_COLOR_HIGHLIGHT));

    let help_paragraph = Paragraph::new(lines)
        .block(help_block)
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, popup_area);

    // Add scroll indicator at the bottom
    render_help_scroll_indicator(f, app, popup_area, help_items.len());
}

/// Renders scroll indicator for the help popup
fn render_help_scroll_indicator(f: &mut Frame, app: &App, popup_area: Rect, total_items: usize) {
    if app.help_scroll > 0 || app.help_scroll < total_items.saturating_sub(1) {
        let scroll_info = format!(
            "j/k to scroll, Esc to close ({}/{})",
            app.help_scroll + 1,
            total_items
        );
        
        let scroll_area = Rect {
            x: popup_area.x + 2,
            y: popup_area.y + popup_area.height.saturating_sub(1),
            width: popup_area.width.saturating_sub(4),
            height: 1,
        };
        
        let scroll_text = Paragraph::new(scroll_info)
            .style(Style::default().fg(TEXT_COLOR_MUTED))
            .alignment(Alignment::Center);
        f.render_widget(scroll_text, scroll_area);
    }
}

/// Renders an error popup with the given error message
pub fn render_error_popup(f: &mut Frame, error_message: &str) {
    // Calculate popup area - smaller than help popup
    let popup_area = create_fixed_popup_layout(f.area(), 60, 8);

    // Clear the background
    f.render_widget(Clear, popup_area);

    // Create error content
    let error_block = create_error_block(" Error ");

    // Split error message into lines that fit the popup width
    let max_width = popup_area.width.saturating_sub(4) as usize;
    let error_lines = wrap_error_text(error_message, max_width);

    let error_paragraph = Paragraph::new(error_lines)
        .block(error_block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    f.render_widget(error_paragraph, popup_area);

    // Add instruction to close
    render_error_close_instruction(f, popup_area);
}

/// Wraps error text to fit within the popup width
fn wrap_error_text(text: &str, max_width: usize) -> Vec<Line> {
    let mut lines = Vec::new();
    
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            lines.push(Line::from(""));
            continue;
        }
        
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();
        
        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };
            
            if test_line.len() <= max_width {
                current_line = test_line;
            } else {
                if !current_line.is_empty() {
                    lines.push(Line::from(current_line));
                }
                current_line = word.to_string();
            }
        }
        
        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }
    }
    
    lines
}

/// Renders instruction to close the error popup
fn render_error_close_instruction(f: &mut Frame, popup_area: Rect) {
    let instruction_area = Rect {
        x: popup_area.x + 2,
        y: popup_area.y + popup_area.height.saturating_sub(1),
        width: popup_area.width.saturating_sub(4),
        height: 1,
    };
    
    let instruction_text = Paragraph::new("Press any key to dismiss")
        .style(Style::default().fg(TEXT_COLOR_MUTED))
        .alignment(Alignment::Center);
    f.render_widget(instruction_text, instruction_area);
}

/// Renders a confirmation dialog with Yes/No options
pub fn render_confirmation_popup(f: &mut Frame, title: &str, message: &str, selected: bool) {
    let popup_area = create_fixed_popup_layout(f.area(), 50, 10);

    // Clear the background
    f.render_widget(Clear, popup_area);

    // Split popup into message and buttons areas
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),      // Message area
            Constraint::Length(3),   // Button area
        ])
        .split(popup_area);

    // Render message area
    let message_block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TEXT_COLOR_HIGHLIGHT));

    let message_paragraph = Paragraph::new(message)
        .block(message_block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(message_paragraph, popup_layout[0]);

    // Render button area
    render_confirmation_buttons(f, popup_layout[1], selected);
}

/// Renders Yes/No buttons for confirmation dialog
fn render_confirmation_buttons(f: &mut Frame, area: Rect, yes_selected: bool) {
    let button_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    // Yes button
    let yes_style = if yes_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };

    let yes_button = Paragraph::new("Yes")
        .style(yes_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(yes_button, button_layout[0]);

    // No button
    let no_style = if !yes_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };

    let no_button = Paragraph::new("No")
        .style(no_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(no_button, button_layout[1]);
}

/// Renders a loading popup with a spinner
pub fn render_loading_popup(f: &mut Frame, message: &str, spinner_state: usize) {
    let popup_area = create_fixed_popup_layout(f.area(), 40, 6);

    // Clear the background
    f.render_widget(Clear, popup_area);

    let loading_block = Block::default()
        .title(" Loading ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));

    // Spinner animation
    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let spinner_char = spinner_chars[spinner_state % spinner_chars.len()];

    let loading_text = format!("{} {}", spinner_char, message);
    let loading_paragraph = Paragraph::new(loading_text)
        .block(loading_block)
        .alignment(Alignment::Center);

    f.render_widget(loading_paragraph, popup_area);
}

/// Renders an information popup with just a message
pub fn render_info_popup(f: &mut Frame, title: &str, message: &str) {
    let popup_area = create_fixed_popup_layout(f.area(), 50, 8);

    // Clear the background
    f.render_widget(Clear, popup_area);

    let info_block = Block::default()
        .title(format!(" {} ", title))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));

    let info_paragraph = Paragraph::new(message)
        .block(info_block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(info_paragraph, popup_area);

    // Add instruction to close
    let instruction_area = Rect {
        x: popup_area.x + 2,
        y: popup_area.y + popup_area.height.saturating_sub(1),
        width: popup_area.width.saturating_sub(4),
        height: 1,
    };
    
    let instruction_text = Paragraph::new("Press any key to continue")
        .style(Style::default().fg(TEXT_COLOR_MUTED))
        .alignment(Alignment::Center);
    f.render_widget(instruction_text, instruction_area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};
    use crate::app::App;

    #[test]
    fn test_wrap_error_text() {
        let text = "This is a very long error message that should be wrapped";
        let lines = wrap_error_text(text, 20);
        
        assert!(!lines.is_empty());
        for line in &lines {
            assert!(line.width() <= 20);
        }
    }

    #[test]
    fn test_wrap_error_text_with_newlines() {
        let text = "Line 1\nLine 2\nLine 3";
        let lines = wrap_error_text(text, 50);
        
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_render_error_popup() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let error_message = "Test error message";

        terminal.draw(|f| {
            render_error_popup(f, error_message);
        }).unwrap();
    }

    #[test]
    fn test_render_help_popup() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        terminal.draw(|f| {
            render_help_popup(f, &app);
        }).unwrap();
    }

    #[test]
    fn test_render_confirmation_popup() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            render_confirmation_popup(f, "Confirm", "Are you sure?", true);
        }).unwrap();
    }

    #[test]
    fn test_render_loading_popup() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            render_loading_popup(f, "Sending request...", 0);
        }).unwrap();
    }

    #[test]
    fn test_render_info_popup() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            render_info_popup(f, "Information", "This is an info message");
        }).unwrap();
    }
}