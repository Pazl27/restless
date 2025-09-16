//! User Interface module for Restless
//! 
//! This module contains all UI-related functionality, organized into separate
//! components for better maintainability and testing.

pub mod components;
pub mod popups;
pub mod renderer;
pub mod layouts;

pub use renderer::ui;
pub use components::*;
pub use popups::*;
pub use layouts::*;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, Tabs},
    Frame,
};

use crate::app::{App, ValuesScreen};

/// Common UI constants and utilities
pub const BORDER_COLOR_ACTIVE: Color = Color::Green;
pub const BORDER_COLOR_INACTIVE: Color = Color::White;
pub const BORDER_COLOR_EDITING: Color = Color::Yellow;
pub const BORDER_COLOR_ERROR: Color = Color::Red;

pub const TEXT_COLOR_NORMAL: Color = Color::White;
pub const TEXT_COLOR_HIGHLIGHT: Color = Color::Yellow;
pub const TEXT_COLOR_ERROR: Color = Color::Red;
pub const TEXT_COLOR_SUCCESS: Color = Color::Green;
pub const TEXT_COLOR_INFO: Color = Color::Blue;
pub const TEXT_COLOR_MUTED: Color = Color::Gray;

/// Creates a styled block with appropriate border color based on state
pub fn create_block(title: &str, is_active: bool, is_editing: bool) -> Block {
    let border_color = if is_editing {
        BORDER_COLOR_EDITING
    } else if is_active {
        BORDER_COLOR_ACTIVE
    } else {
        BORDER_COLOR_INACTIVE
    };

    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
}

/// Creates a styled block for error display
pub fn create_error_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER_COLOR_ERROR))
}

/// Utility function to calculate centered popup area
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Utility function to calculate fixed size centered popup
pub fn centered_rect_fixed(width: u16, height: u16, area: Rect) -> Rect {
    let popup_width = std::cmp::min(width, area.width.saturating_sub(2));
    let popup_height = std::cmp::min(height, area.height.saturating_sub(2));
    
    Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    }
}

/// Creates a status line with method color coding
pub fn get_method_color(method: &crate::logic::HttpMethod) -> Color {
    match method {
        crate::logic::HttpMethod::GET => Color::Green,
        crate::logic::HttpMethod::POST => Color::Blue,
        crate::logic::HttpMethod::PUT => Color::Yellow,
        crate::logic::HttpMethod::DELETE => Color::Red,
    }
}

/// Creates styled text for HTTP methods
pub fn method_text(method: &crate::logic::HttpMethod) -> Span {
    let method_str = match method {
        crate::logic::HttpMethod::GET => "GET",
        crate::logic::HttpMethod::POST => "POST",
        crate::logic::HttpMethod::PUT => "PUT",
        crate::logic::HttpMethod::DELETE => "DELETE",
    };
    
    Span::styled(
        method_str,
        Style::default().fg(get_method_color(method))
    )
}

/// Truncates text to fit within a given width
pub fn truncate_text(text: &str, max_width: usize) -> String {
    if text.len() <= max_width {
        text.to_string()
    } else if max_width <= 3 {
        "...".to_string()
    } else {
        format!("{}...", &text[..max_width.saturating_sub(3)])
    }
}

/// Wraps text to multiple lines with a given width
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    
    text.chars()
        .collect::<Vec<char>>()
        .chunks(width)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("hello", 10), "hello");
        assert_eq!(truncate_text("hello world", 8), "hello...");
        assert_eq!(truncate_text("hi", 2), "hi");
        assert_eq!(truncate_text("hello", 3), "...");
    }

    #[test]
    fn test_wrap_text() {
        let result = wrap_text("hello world", 5);
        assert_eq!(result, vec!["hello", " worl", "d"]);
    }

    #[test]
    fn test_centered_rect_fixed() {
        let area = Rect::new(0, 0, 100, 50);
        let popup = centered_rect_fixed(60, 20, area);
        
        assert_eq!(popup.width, 60);
        assert_eq!(popup.height, 20);
        assert_eq!(popup.x, 20);
        assert_eq!(popup.y, 15);
    }
}