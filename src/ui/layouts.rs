//! Layout management for the Restless UI
//!
//! This module handles the creation and management of different UI layouts,
//! providing a clean separation between layout logic and component rendering.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Main application layout structure
///
/// This struct holds all the areas for the main application components,
/// making it easy to pass layout information between functions.
#[derive(Debug, Clone)]
pub struct MainLayout {
    pub tabs_area: Rect,
    pub url_area: Rect,
    pub values_area: Rect,
    pub response_area: Rect,
    pub status_area: Rect,
}

/// Creates the main application layout
///
/// This function splits the terminal area into sections for different
/// UI components. The layout is responsive and will adjust to different
/// terminal sizes.
pub fn create_main_layout(area: Rect) -> MainLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs section
            Constraint::Length(3), // URL input section
            Constraint::Min(8),    // Values section (expandable)
            Constraint::Min(8),    // Response section (expandable)
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    MainLayout {
        tabs_area: chunks[0],
        url_area: chunks[1],
        values_area: chunks[2],
        response_area: chunks[3],
        status_area: chunks[4],
    }
}

/// Creates a two-column layout for the URL input section
///
/// Splits the URL area into method selector and URL input field.
pub fn create_url_layout(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12), // Method selector
            Constraint::Min(20),    // URL input field
        ])
        .split(area);

    (chunks[0], chunks[1])
}

/// Creates layout for the values section with tabs
///
/// Separates the tab bar from the content area in the values section.
pub fn create_values_layout(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Min(5),    // Content area
        ])
        .split(area);

    (chunks[0], chunks[1])
}

/// Creates layout for the response section with tabs
///
/// Separates the tab bar from the content area in the response section.
pub fn create_response_layout(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Tab bar (smaller than values)
            Constraint::Min(7),    // Content area with scrollbar
        ])
        .split(area);

    (chunks[0], chunks[1])
}

/// Creates a popup layout with specified dimensions
///
/// This is used for dialogs, help screens, and error messages.
pub fn create_popup_layout(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_percent) / 2),
            Constraint::Percentage(height_percent),
            Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_percent) / 2),
            Constraint::Percentage(width_percent),
            Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Creates a fixed-size popup layout
///
/// This is useful when you know the exact size needed for a popup.
pub fn create_fixed_popup_layout(area: Rect, width: u16, height: u16) -> Rect {
    let popup_width = std::cmp::min(width, area.width.saturating_sub(2));
    let popup_height = std::cmp::min(height, area.height.saturating_sub(2));

    Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    }
}

/// Creates a method dropdown layout positioned below the method selector
pub fn create_method_dropdown_layout(method_area: Rect) -> Rect {
    let methods_count = 4; // GET, POST, PUT, DELETE
    Rect {
        x: method_area.x,
        y: method_area.y + method_area.height,
        width: method_area.width,
        height: methods_count + 2, // Space for methods + borders
    }
}

/// Layout configuration for different screen sizes
#[derive(Debug, Clone)]
#[cfg(test)]
pub struct LayoutConfig {
    pub min_width: u16,
    pub min_height: u16,
    pub tabs_height: u16,
    pub url_height: u16,
    pub status_height: u16,
    #[allow(dead_code)]
    pub method_width: u16,
}

#[cfg(test)]
impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            min_width: 80,
            min_height: 24,
            tabs_height: 3,
            url_height: 3,
            status_height: 3,
            method_width: 12,
        }
    }
}

/// Validates that the terminal area is large enough for the UI
#[cfg(test)]
pub fn validate_terminal_size(area: Rect, config: &LayoutConfig) -> Result<(), String> {
    if area.width < config.min_width {
        return Err(format!(
            "Terminal width too small: {} (minimum: {})",
            area.width, config.min_width
        ));
    }

    if area.height < config.min_height {
        return Err(format!(
            "Terminal height too small: {} (minimum: {})",
            area.height, config.min_height
        ));
    }

    Ok(())
}

/// Creates a responsive layout that adapts to the terminal size
#[cfg(test)]
pub fn create_responsive_layout(area: Rect) -> Result<MainLayout, String> {
    let config = LayoutConfig::default();
    validate_terminal_size(area, &config)?;

    // Adjust constraints based on available space
    let (values_min, response_min) = if area.height >= 30 {
        (10, 10) // Larger areas for bigger terminals
    } else if area.height >= 24 {
        (8, 8) // Standard areas
    } else {
        (5, 5) // Minimal areas for small terminals
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(config.tabs_height),
            Constraint::Length(config.url_height),
            Constraint::Min(values_min),
            Constraint::Min(response_min),
            Constraint::Length(config.status_height),
        ])
        .split(area);

    Ok(MainLayout {
        tabs_area: chunks[0],
        url_area: chunks[1],
        values_area: chunks[2],
        response_area: chunks[3],
        status_area: chunks[4],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_main_layout() {
        let area = Rect::new(0, 0, 80, 24);
        let layout = create_main_layout(area);

        assert_eq!(layout.tabs_area.height, 3);
        assert_eq!(layout.url_area.height, 2); // Ratatui adjusts to fit Min constraints
        assert_eq!(layout.status_area.height, 3);
        assert_eq!(layout.values_area.height, 8);
        assert_eq!(layout.response_area.height, 8);
    }

    #[test]
    fn test_create_main_layout_large_terminal() {
        // Test with larger terminal to verify normal behavior
        let area = Rect::new(0, 0, 80, 40);
        let layout = create_main_layout(area);

        assert_eq!(layout.tabs_area.height, 3);
        assert_eq!(layout.url_area.height, 3);
        assert_eq!(layout.status_area.height, 3);
        assert!(layout.values_area.height >= 8);
        assert!(layout.response_area.height >= 8);
        // Should have extra space distributed between values and response
        assert_eq!(layout.values_area.height + layout.response_area.height, 31); // 40 - 9 = 31
    }

    #[test]
    fn test_create_url_layout() {
        let area = Rect::new(0, 0, 80, 3);
        let (method_area, url_area) = create_url_layout(area);

        assert_eq!(method_area.width, 12);
        assert_eq!(url_area.width, 68);
    }

    #[test]
    fn test_validate_terminal_size() {
        let config = LayoutConfig::default();

        // Valid size
        let valid_area = Rect::new(0, 0, 80, 24);
        assert!(validate_terminal_size(valid_area, &config).is_ok());

        // Too narrow
        let narrow_area = Rect::new(0, 0, 70, 24);
        assert!(validate_terminal_size(narrow_area, &config).is_err());

        // Too short
        let short_area = Rect::new(0, 0, 80, 20);
        assert!(validate_terminal_size(short_area, &config).is_err());
    }

    #[test]
    fn test_create_popup_layout() {
        let area = Rect::new(0, 0, 100, 50);
        let popup = create_popup_layout(area, 80, 60);

        assert_eq!(popup.width, 80);
        assert_eq!(popup.height, 30); // 60% of 50
        assert_eq!(popup.x, 10); // Centered
        assert_eq!(popup.y, 10); // Centered
    }

    #[test]
    fn test_create_fixed_popup_layout() {
        let area = Rect::new(0, 0, 100, 50);
        let popup = create_fixed_popup_layout(area, 60, 20);

        assert_eq!(popup.width, 60);
        assert_eq!(popup.height, 20);
        assert_eq!(popup.x, 20);
        assert_eq!(popup.y, 15);
    }

    #[test]
    fn test_create_responsive_layout() {
        // Test with large terminal
        let large_area = Rect::new(0, 0, 120, 40);
        let layout = create_responsive_layout(large_area).unwrap();
        assert!(layout.values_area.height >= 10);

        // Test with small terminal
        let small_area = Rect::new(0, 0, 80, 24);
        let layout = create_responsive_layout(small_area).unwrap();
        assert!(layout.values_area.height >= 5);

        // Test with too small terminal
        let tiny_area = Rect::new(0, 0, 60, 20);
        assert!(create_responsive_layout(tiny_area).is_err());
    }
}
