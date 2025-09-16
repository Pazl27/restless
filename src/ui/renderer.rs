//! Main UI renderer that coordinates all UI components
//!
//! This module serves as the main entry point for rendering the application UI.
//! It coordinates between different UI components and handles the overall layout.

use super::{
    components::{
        render_response_section, render_status_bar, render_tabs, render_url_input,
        render_values_section,
    },
    layouts::create_main_layout,
    popups::{render_error_popup, render_help_popup},
};
use crate::app::App;
use ratatui::Frame;

/// Main UI rendering function
///
/// This is the entry point for all UI rendering. It coordinates the rendering
/// of all UI components and handles popups.
pub fn ui(f: &mut Frame, app: &mut App, error_message: &Option<String>) {
    // Create the main application layout
    let layout = create_main_layout(f.area());

    // Render main application components
    render_main_content(f, app, &layout);

    // Render popups on top of main content
    render_popups(f, app, error_message);
}

/// Renders the main application content in the provided layout
fn render_main_content(f: &mut Frame, app: &mut App, layout: &crate::ui::layouts::MainLayout) {
    // Render components in order from top to bottom
    render_tabs(f, app, layout.tabs_area);
    render_url_input(f, app, layout.url_area);
    render_values_section(f, app, layout.values_area);
    render_response_section(f, app, layout.response_area);
    render_status_bar(f, app, layout.status_area);
}

/// Renders any active popups over the main content
fn render_popups(f: &mut Frame, app: &App, error_message: &Option<String>) {
    // Help popup takes precedence over error popup
    if app.help_visible {
        render_help_popup(f, app);
    } else if let Some(error) = error_message {
        render_error_popup(f, error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use ratatui::{backend::TestBackend, layout::Rect, Terminal};

    #[test]
    fn test_ui_rendering_no_crash() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        let error_message = None;

        // This test ensures UI rendering doesn't crash
        terminal.draw(|f| ui(f, &mut app, &error_message)).unwrap();
    }

    #[test]
    fn test_ui_rendering_with_error() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        let error_message = Some("Test error message".to_string());

        // This test ensures error popup rendering doesn't crash
        terminal.draw(|f| ui(f, &mut app, &error_message)).unwrap();
    }

    #[test]
    fn test_ui_rendering_with_help() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        app.show_help();
        let error_message = None;

        // This test ensures help popup rendering doesn't crash
        terminal.draw(|f| ui(f, &mut app, &error_message)).unwrap();
    }
}
