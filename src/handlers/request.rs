//! Request handling module for HTTP operations
//!
//! This module contains logic for handling HTTP requests, including validation,
//! sending, and response processing. It provides a clean interface between
//! the application state and the HTTP client.

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::app::App;
use crate::error::{RestlessError, Result};
use crate::logic::response::Response;

/// Handles sending an HTTP request for the current tab
#[cfg(test)]
pub async fn send_current_request(app: &mut App) -> Result<Option<String>> {
    // Validate request before sending
    if let Err(e) = app.validate_current_request() {
        return Ok(Some(format!("Validation error: {}", e)));
    }

    // Get the current tab
    let current_tab_index = app.selected_tab;
    if current_tab_index >= app.tabs.len() {
        return Ok(Some("Invalid tab index".to_string()));
    }

    // Update the request with current app state
    sync_request_with_app_state(app)?;

    // Send the request
    match app.tabs[current_tab_index].request.send().await {
        Ok((status_code, headers, body)) => {
            handle_successful_response(app, current_tab_index, status_code, headers, body).await
        }
        Err(e) => handle_request_error(e),
    }
}

/// Synchronizes the request object with the current application state
#[cfg(test)]
fn sync_request_with_app_state(app: &mut App) -> Result<()> {
    let current_tab = app
        .tabs
        .get_mut(app.selected_tab)
        .ok_or_else(|| RestlessError::app_state("Invalid tab index".to_string()))?;

    // Update request fields from app state
    current_tab.request.url = app.url_input.clone();
    current_tab.request.method = (&app.selected_method).into();
    current_tab.request.body = if app.body_input.is_empty() {
        None
    } else {
        Some(app.body_input.clone())
    };
    current_tab.request.headers = app.headers_input.clone();
    current_tab.request.params = app.params_input.clone();

    Ok(())
}

/// Handles a successful HTTP response
#[cfg(test)]
async fn handle_successful_response(
    app: &mut App,
    tab_index: usize,
    status_code: u16,
    headers: String,
    body: String,
) -> Result<Option<String>> {
    match Response::new(status_code, headers.clone(), body.clone()) {
        Ok(response) => {
            app.tabs[tab_index].response = Some(response);
            Ok(None) // No error message
        }
        Err(e) => {
            // Still create response with unchecked method for display
            let response = Response::new_unchecked(status_code, headers, body);
            app.tabs[tab_index].response = Some(response);
            Ok(Some(format!("Response parsing warning: {}", e)))
        }
    }
}

/// Handles request errors and returns appropriate error messages
#[cfg(test)]
fn handle_request_error(error: anyhow::Error) -> Result<Option<String>> {
    // Convert to our error type and format for display
    let restless_error: RestlessError = error.into();
    Ok(Some(format!("Request failed: {}", restless_error)))
}

/// Validates that all required fields are present for a request
#[cfg(test)]
pub fn validate_request_completeness(app: &App) -> Result<()> {
    if app.url_input.trim().is_empty() {
        return Err(RestlessError::invalid_url("URL cannot be empty"));
    }

    // Additional validation can be added here
    Ok(())
}

/// Clears the response for the current tab
#[cfg(test)]
pub fn clear_current_response(app: &mut App) -> Result<()> {
    let current_tab = app
        .tabs
        .get_mut(app.selected_tab)
        .ok_or_else(|| RestlessError::app_state("Invalid tab index".to_string()))?;

    current_tab.response = None;
    Ok(())
}

/// Gets the content type from the current request headers
#[cfg(test)]
pub fn get_request_content_type(app: &App) -> Option<String> {
    app.headers_input
        .iter()
        .find(|(key, _)| key.to_lowercase() == "content-type")
        .map(|(_, value)| value.clone())
}

/// Sets the content type header for the current request
#[cfg(test)]
pub fn set_request_content_type(app: &mut App, content_type: &str) -> Result<()> {
    // Remove existing content-type header
    app.headers_input
        .retain(|(key, _)| key.to_lowercase() != "content-type");

    // Add new content-type header
    app.headers_input
        .push(("Content-Type".to_string(), content_type.to_string()));

    Ok(())
}

/// Formats request information for display
#[cfg(test)]
pub fn format_request_info(app: &App) -> String {
    let method = match app.selected_method {
        crate::logic::HttpMethod::GET => "GET",
        crate::logic::HttpMethod::POST => "POST",
        crate::logic::HttpMethod::PUT => "PUT",
        crate::logic::HttpMethod::DELETE => "DELETE",
    };

    let url = if app.url_input.is_empty() {
        "<no URL>"
    } else {
        &app.url_input
    };

    let headers_count = app.headers_input.len();
    let params_count = app.params_input.len();
    let has_body = !app.body_input.is_empty();

    format!(
        "{} {} (Headers: {}, Params: {}, Body: {})",
        method,
        url,
        headers_count,
        params_count,
        if has_body { "Yes" } else { "No" }
    )
}

/// Prepares request for sending by performing final validations and setup
#[cfg(test)]
pub async fn prepare_request(app: &mut App) -> Result<()> {
    // Sync app state to request
    sync_request_with_app_state(app)?;

    // Validate the complete request
    app.validate_current_request()?;

    // Additional preparation steps can be added here
    // For example: adding default headers, encoding body, etc.

    Ok(())
}

/// Handles request cancellation
#[cfg(test)]
pub fn cancel_request(_app: &mut App) -> Result<()> {
    // In a real implementation, this would cancel any ongoing HTTP request
    // For now, we just clear any pending state

    // Reset any loading states
    // This could be expanded to include cancellation tokens in the future

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::HttpMethod;

    fn create_test_app() -> App {
        let mut app = App::new();
        app.url_input = "https://httpbin.org/get".to_string();
        app.selected_method = HttpMethod::GET;
        app
    }

    #[test]
    fn test_validate_request_completeness() {
        let mut app = create_test_app();

        // Valid request
        assert!(validate_request_completeness(&app).is_ok());

        // Empty URL
        app.url_input.clear();
        assert!(validate_request_completeness(&app).is_err());
    }

    #[test]
    fn test_get_request_content_type() {
        let mut app = create_test_app();

        // No content type header
        assert!(get_request_content_type(&app).is_none());

        // Add content type header
        app.headers_input
            .push(("Content-Type".to_string(), "application/json".to_string()));
        assert_eq!(
            get_request_content_type(&app),
            Some("application/json".to_string())
        );

        // Case insensitive
        app.headers_input.clear();
        app.headers_input
            .push(("content-type".to_string(), "text/plain".to_string()));
        assert_eq!(
            get_request_content_type(&app),
            Some("text/plain".to_string())
        );
    }

    #[test]
    fn test_set_request_content_type() {
        let mut app = create_test_app();

        // Set content type
        set_request_content_type(&mut app, "application/json").unwrap();
        assert_eq!(
            get_request_content_type(&app),
            Some("application/json".to_string())
        );

        // Replace existing content type
        set_request_content_type(&mut app, "text/plain").unwrap();
        assert_eq!(
            get_request_content_type(&app),
            Some("text/plain".to_string())
        );
        assert_eq!(app.headers_input.len(), 1); // Should replace, not add
    }

    #[test]
    fn test_format_request_info() {
        let mut app = create_test_app();
        app.headers_input
            .push(("Accept".to_string(), "application/json".to_string()));
        app.params_input
            .push(("limit".to_string(), "10".to_string()));
        app.body_input = "test body".to_string();

        let info = format_request_info(&app);
        assert!(info.contains("GET"));
        assert!(info.contains("https://httpbin.org/get"));
        assert!(info.contains("Headers: 1"));
        assert!(info.contains("Params: 1"));
        assert!(info.contains("Body: Yes"));
    }

    #[test]
    fn test_clear_current_response() {
        let mut app = create_test_app();

        // Set a dummy response
        app.tabs[0].response = Some(crate::logic::response::Response::new_unchecked(
            200,
            "Content-Type: application/json".to_string(),
            "{}".to_string(),
        ));

        assert!(app.tabs[0].response.is_some());

        // Clear response
        clear_current_response(&mut app).unwrap();
        assert!(app.tabs[0].response.is_none());
    }

    #[tokio::test]
    async fn test_sync_request_with_app_state() {
        let mut app = create_test_app();
        app.body_input = "test body".to_string();
        app.headers_input
            .push(("Accept".to_string(), "application/json".to_string()));
        app.params_input
            .push(("limit".to_string(), "10".to_string()));

        sync_request_with_app_state(&mut app).unwrap();

        let request = &app.tabs[0].request;
        assert_eq!(request.url, "https://httpbin.org/get");
        assert_eq!(request.body, Some("test body".to_string()));
        assert_eq!(request.headers.len(), 1);
        assert_eq!(request.params.len(), 1);
    }
}
