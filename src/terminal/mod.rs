//! Terminal management module for Restless
//!
//! This module handles all terminal-related functionality including initialization,
//! cleanup, and error handling. It provides a clean abstraction over the
//! crossterm terminal management functions.

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stderr};

use crate::error::RestlessError;

/// Terminal manager that handles setup and cleanup
pub struct TerminalManager {
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl TerminalManager {
    /// Creates a new terminal manager and initializes the terminal
    pub fn new() -> Result<Self, RestlessError> {
        let terminal = Self::setup_terminal()?;
        Ok(Self { terminal })
    }

    /// Sets up the terminal for the application
    fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stderr>>, RestlessError> {
        // Enable raw mode
        enable_raw_mode()
            .map_err(|e| RestlessError::terminal(format!("Failed to enable raw mode: {}", e)))?;

        // Setup terminal backend
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)
            .map_err(|e| RestlessError::terminal(format!("Failed to setup terminal: {}", e)))?;

        // Create terminal instance
        let backend = CrosstermBackend::new(stderr);
        let terminal = Terminal::new(backend)
            .map_err(|e| RestlessError::terminal(format!("Failed to create terminal: {}", e)))?;

        Ok(terminal)
    }

    /// Gets a mutable reference to the terminal
    pub fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<Stderr>> {
        &mut self.terminal
    }

    /// Gets an immutable reference to the terminal
    pub fn terminal(&self) -> &Terminal<CrosstermBackend<Stderr>> {
        &self.terminal
    }

    /// Validates that the terminal size is adequate for the application
    pub fn validate_size(&self) -> Result<(), RestlessError> {
        let size = self
            .terminal
            .size()
            .map_err(|e| RestlessError::terminal(format!("Failed to get terminal size: {}", e)))?;

        const MIN_WIDTH: u16 = 80;
        const MIN_HEIGHT: u16 = 24;

        if size.width < MIN_WIDTH {
            return Err(RestlessError::terminal(format!(
                "Terminal width too small: {} (minimum: {})",
                size.width, MIN_WIDTH
            )));
        }

        if size.height < MIN_HEIGHT {
            return Err(RestlessError::terminal(format!(
                "Terminal height too small: {} (minimum: {})",
                size.height, MIN_HEIGHT
            )));
        }

        Ok(())
    }

    /// Cleanly shuts down the terminal
    pub fn cleanup(mut self) -> Result<(), RestlessError> {
        self.cleanup_terminal()
    }

    /// Internal cleanup function
    fn cleanup_terminal(&mut self) -> Result<(), RestlessError> {
        // Disable raw mode
        disable_raw_mode()
            .map_err(|e| RestlessError::terminal(format!("Failed to disable raw mode: {}", e)))?;

        // Restore terminal
        execute!(
            self.terminal.backend_mut(),
            DisableMouseCapture,
            LeaveAlternateScreen
        )
        .map_err(|e| RestlessError::terminal(format!("Failed to cleanup terminal: {}", e)))?;

        // Show cursor
        self.terminal
            .show_cursor()
            .map_err(|e| RestlessError::terminal(format!("Failed to show cursor: {}", e)))?;

        Ok(())
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        // Attempt cleanup on drop, but don't panic if it fails
        if let Err(e) = self.cleanup_terminal() {
            eprintln!("Warning: Failed to cleanup terminal during drop: {}", e);
        }
    }
}

/// Configuration for terminal setup
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    pub min_width: u16,
    pub min_height: u16,
    pub enable_mouse: bool,
    pub use_alternate_screen: bool,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            min_width: 80,
            min_height: 24,
            enable_mouse: true,
            use_alternate_screen: true,
        }
    }
}

/// Advanced terminal manager with configuration options
pub struct ConfigurableTerminalManager {
    terminal: Terminal<CrosstermBackend<Stderr>>,
    config: TerminalConfig,
}

impl ConfigurableTerminalManager {
    /// Creates a new configurable terminal manager
    pub fn new(config: TerminalConfig) -> Result<Self, RestlessError> {
        let terminal = Self::setup_terminal_with_config(&config)?;
        Ok(Self { terminal, config })
    }

    /// Sets up terminal with the given configuration
    fn setup_terminal_with_config(
        config: &TerminalConfig,
    ) -> Result<Terminal<CrosstermBackend<Stderr>>, RestlessError> {
        // Enable raw mode
        enable_raw_mode()
            .map_err(|e| RestlessError::terminal(format!("Failed to enable raw mode: {}", e)))?;

        let mut stderr = io::stderr();

        // Setup terminal features based on config
        if config.use_alternate_screen && config.enable_mouse {
            execute!(stderr, EnterAlternateScreen, EnableMouseCapture)
                .map_err(|e| RestlessError::terminal(format!("Failed to setup terminal: {}", e)))?;
        } else if config.use_alternate_screen {
            execute!(stderr, EnterAlternateScreen).map_err(|e| {
                RestlessError::terminal(format!("Failed to enter alternate screen: {}", e))
            })?;
        } else if config.enable_mouse {
            execute!(stderr, EnableMouseCapture).map_err(|e| {
                RestlessError::terminal(format!("Failed to enable mouse capture: {}", e))
            })?;
        }

        // Create terminal
        let backend = CrosstermBackend::new(stderr);
        let terminal = Terminal::new(backend)
            .map_err(|e| RestlessError::terminal(format!("Failed to create terminal: {}", e)))?;

        Ok(terminal)
    }

    /// Gets a mutable reference to the terminal
    pub fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<Stderr>> {
        &mut self.terminal
    }

    /// Gets the configuration
    pub fn config(&self) -> &TerminalConfig {
        &self.config
    }

    /// Validates terminal size against configuration
    pub fn validate_size(&self) -> Result<(), RestlessError> {
        let size = self
            .terminal
            .size()
            .map_err(|e| RestlessError::terminal(format!("Failed to get terminal size: {}", e)))?;

        if size.width < self.config.min_width {
            return Err(RestlessError::terminal(format!(
                "Terminal width too small: {} (minimum: {})",
                size.width, self.config.min_width
            )));
        }

        if size.height < self.config.min_height {
            return Err(RestlessError::terminal(format!(
                "Terminal height too small: {} (minimum: {})",
                size.height, self.config.min_height
            )));
        }

        Ok(())
    }

    /// Cleanup with configuration awareness
    pub fn cleanup(mut self) -> Result<(), RestlessError> {
        self.cleanup_terminal()
    }

    fn cleanup_terminal(&mut self) -> Result<(), RestlessError> {
        // Disable raw mode
        disable_raw_mode()
            .map_err(|e| RestlessError::terminal(format!("Failed to disable raw mode: {}", e)))?;

        // Cleanup based on what was enabled
        if self.config.use_alternate_screen && self.config.enable_mouse {
            execute!(
                self.terminal.backend_mut(),
                DisableMouseCapture,
                LeaveAlternateScreen
            )
            .map_err(|e| RestlessError::terminal(format!("Failed to cleanup terminal: {}", e)))?;
        } else if self.config.use_alternate_screen {
            execute!(self.terminal.backend_mut(), LeaveAlternateScreen).map_err(|e| {
                RestlessError::terminal(format!("Failed to leave alternate screen: {}", e))
            })?;
        } else if self.config.enable_mouse {
            execute!(self.terminal.backend_mut(), DisableMouseCapture).map_err(|e| {
                RestlessError::terminal(format!("Failed to disable mouse capture: {}", e))
            })?;
        }

        // Show cursor
        self.terminal
            .show_cursor()
            .map_err(|e| RestlessError::terminal(format!("Failed to show cursor: {}", e)))?;

        Ok(())
    }
}

impl Drop for ConfigurableTerminalManager {
    fn drop(&mut self) {
        if let Err(e) = self.cleanup_terminal() {
            eprintln!("Warning: Failed to cleanup terminal during drop: {}", e);
        }
    }
}

/// Utility functions for terminal operations
pub mod utils {
    use super::*;

    /// Gets the current terminal size
    pub fn get_terminal_size() -> Result<(u16, u16), RestlessError> {
        let size = crossterm::terminal::size()
            .map_err(|e| RestlessError::terminal(format!("Failed to get terminal size: {}", e)))?;
        Ok((size.0, size.1))
    }

    /// Checks if the terminal supports colors
    pub fn supports_color() -> bool {
        // This is a simplified check - in reality you might want to check
        // environment variables like TERM, COLORTERM, etc.
        true
    }

    /// Checks if we're running in a terminal
    pub fn is_terminal() -> bool {
        atty::is(atty::Stream::Stdout) && atty::is(atty::Stream::Stderr)
    }

    /// Emergency cleanup function that can be called from signal handlers
    pub fn emergency_cleanup() {
        let _ = disable_raw_mode();
        let _ = execute!(io::stderr(), DisableMouseCapture, LeaveAlternateScreen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_config_default() {
        let config = TerminalConfig::default();
        assert_eq!(config.min_width, 80);
        assert_eq!(config.min_height, 24);
        assert!(config.enable_mouse);
        assert!(config.use_alternate_screen);
    }

    #[test]
    fn test_terminal_config_custom() {
        let config = TerminalConfig {
            min_width: 100,
            min_height: 30,
            enable_mouse: false,
            use_alternate_screen: false,
        };
        assert_eq!(config.min_width, 100);
        assert_eq!(config.min_height, 30);
        assert!(!config.enable_mouse);
        assert!(!config.use_alternate_screen);
    }

    #[test]
    fn test_utils_supports_color() {
        // This should always return true in our simplified implementation
        assert!(utils::supports_color());
    }

    // Note: Terminal manager tests are difficult to run in a test environment
    // as they require actual terminal interaction. In a real project, you might
    // want to create mock backends for testing.
}
