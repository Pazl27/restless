//! Tab management handlers for the Restless application
//!
//! This module contains handlers for tab-related operations such as creating,
//! closing, and switching between tabs.

use crate::app::App;
use crate::error::Result;

/// Handles creating a new tab
pub fn handle_new_tab(app: &mut App) -> Result<Option<String>> {
    if let Err(e) = app.add_new_tab() {
        Ok(Some(format!("Tab error: {}", e)))
    } else {
        Ok(None)
    }
}

/// Handles closing the current tab
pub fn handle_close_tab(app: &mut App) -> Result<Option<String>> {
    if let Err(e) = app.close_current_tab() {
        Ok(Some(format!("Tab error: {}", e)))
    } else {
        Ok(None)
    }
}

/// Handles switching to the next tab
pub fn handle_next_tab(app: &mut App) -> Result<Option<String>> {
    if let Err(e) = app.next_tab() {
        Ok(Some(format!("Tab error: {}", e)))
    } else {
        Ok(None)
    }
}

/// Handles switching to the previous tab
pub fn handle_prev_tab(app: &mut App) -> Result<Option<String>> {
    if let Err(e) = app.prev_tab() {
        Ok(Some(format!("Tab error: {}", e)))
    } else {
        Ok(None)
    }
}

/// Handles switching to a specific tab by index
pub fn handle_switch_to_tab(app: &mut App, index: usize) -> Result<Option<String>> {
    if index >= app.tabs.len() {
        return Ok(Some(format!("Invalid tab index: {}", index)));
    }

    if let Err(e) = app.save_current_tab_state() {
        return Ok(Some(format!("Failed to save current tab: {}", e)));
    }

    app.selected_tab = index;

    if let Err(e) = app.restore_current_tab_state() {
        return Ok(Some(format!("Failed to restore tab: {}", e)));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_new_tab() {
        let mut app = App::new();
        let initial_count = app.tabs.len();

        let result = handle_new_tab(&mut app).unwrap();
        assert!(result.is_none());
        assert_eq!(app.tabs.len(), initial_count + 1);
    }

    #[test]
    fn test_handle_close_tab_multiple_tabs() {
        let mut app = App::new();
        app.add_new_tab().unwrap();

        let result = handle_close_tab(&mut app).unwrap();
        assert!(result.is_none());
        assert_eq!(app.tabs.len(), 1);
    }

    #[test]
    fn test_handle_close_tab_last_tab() {
        let mut app = App::new();

        let result = handle_close_tab(&mut app).unwrap();
        assert!(result.is_some());
        assert_eq!(app.tabs.len(), 1);
    }
}
