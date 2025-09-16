use crate::error::ResponseError;
use serde_json::{to_string_pretty, Value};

pub struct Response {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, headers: String, body: String) -> Result<Self, ResponseError> {
        let parsed_headers = Self::split_headers(&headers)?;
        let formatted_body = Self::pretty_print_json(&body)?;

        Ok(Response {
            status_code,
            headers: parsed_headers,
            body: formatted_body,
        })
    }

    pub fn new_unchecked(status_code: u16, headers: String, body: String) -> Self {
        Response {
            status_code,
            headers: Self::split_headers(&headers).unwrap_or_default(),
            body: Self::pretty_print_json(&body).unwrap_or_else(|_| body),
        }
    }

    fn pretty_print_json(raw_json: &str) -> Result<String, ResponseError> {
        if raw_json.trim().is_empty() {
            return Ok(String::new());
        }

        // Try to parse as JSON
        match serde_json::from_str::<Value>(raw_json.trim()) {
            Ok(json_value) => to_string_pretty(&json_value).map_err(ResponseError::JsonFormatting),
            Err(_) => {
                // If it's not JSON, return as-is
                Ok(raw_json.to_string())
            }
        }
    }

    fn split_headers(header_str: &str) -> Result<Vec<(String, String)>, ResponseError> {
        if header_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut headers = Vec::new();

        for (line_num, line) in header_str.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.splitn(2, ':');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => {
                    let key = key.trim();
                    let value = value.trim();

                    if key.is_empty() {
                        return Err(ResponseError::header_parsing(format!(
                            "Empty header key on line {}: '{}'",
                            line_num + 1,
                            line
                        )));
                    }

                    headers.push((key.to_string(), value.to_string()));
                }
                _ => {
                    // Skip malformed headers but log them
                    eprintln!(
                        "Warning: Skipping malformed header on line {}: '{}'",
                        line_num + 1,
                        line
                    );
                }
            }
        }

        Ok(headers)
    }

    #[cfg(test)]
    pub fn is_json(&self) -> bool {
        self.headers.iter().any(|(key, value)| {
            key.to_lowercase() == "content-type"
                && value.to_lowercase().contains("application/json")
        })
    }

    #[cfg(test)]
    pub fn is_xml(&self) -> bool {
        self.headers.iter().any(|(key, value)| {
            key.to_lowercase() == "content-type"
                && (value.to_lowercase().contains("application/xml")
                    || value.to_lowercase().contains("text/xml"))
        })
    }

    #[cfg(test)]
    pub fn content_type(&self) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.to_lowercase() == "content-type")
            .map(|(_, value)| value.as_str())
    }

    #[cfg(test)]
    pub fn content_length(&self) -> Option<usize> {
        self.headers
            .iter()
            .find(|(key, _)| key.to_lowercase() == "content-length")
            .and_then(|(_, value)| value.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_new_with_invalid_json() {
        let response = Response::new(404, "X-Test: test".to_string(), "not a json".to_string())
            .expect("Should create response successfully");
        assert_eq!(response.status_code, 404);
        assert_eq!(response.headers.len(), 1);
        assert_eq!(response.headers[0].0, "X-Test");
        assert_eq!(response.headers[0].1, "test");
        assert_eq!(response.body, "not a json");
    }

    #[test]
    fn test_response_new_unchecked() {
        let response = Response::new_unchecked(
            200,
            "Content-Type: application/json".to_string(),
            r#"{"test": "value"}"#.to_string(),
        );
        assert_eq!(response.status_code, 200);
        assert!(response.is_json());
    }

    #[test]
    fn test_split_headers_with_empty_string() {
        let headers = Response::split_headers("").expect("Should handle empty string");
        assert!(headers.is_empty());
    }

    #[test]
    fn test_split_headers_with_malformed_lines() {
        let headers = Response::split_headers("Good: header\nMalformedHeader\nAnother: one")
            .expect("Should handle malformed headers");
        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0], ("Good".to_string(), "header".to_string()));
        assert_eq!(headers[1], ("Another".to_string(), "one".to_string()));
    }

    #[test]
    fn test_split_headers_with_extra_colons() {
        let headers = Response::split_headers("Foo: bar: baz").expect("Should handle extra colons");
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0], ("Foo".to_string(), "bar: baz".to_string()));
    }

    #[test]
    fn test_split_headers_with_empty_key() {
        let result = Response::split_headers(": value");
        assert!(result.is_err());
    }

    #[test]
    fn test_pretty_print_json_with_invalid_json() {
        let raw = "not json";
        let pretty = Response::pretty_print_json(raw).expect("Should handle non-JSON");
        assert_eq!(pretty, "not json");
    }

    #[test]
    fn test_pretty_print_json_with_array() {
        let raw = r#"[{"a":1},{"b":2}]"#;
        let pretty = Response::pretty_print_json(raw).expect("Should format JSON");
        assert_eq!(
            pretty,
            "[\n  {\n    \"a\": 1\n  },\n  {\n    \"b\": 2\n  }\n]"
        );
    }

    #[test]
    fn test_pretty_print_json_with_empty_string() {
        let pretty = Response::pretty_print_json("").expect("Should handle empty string");
        assert_eq!(pretty, "");
    }

    #[test]
    fn test_content_type_detection() {
        let response = Response::new_unchecked(
            200,
            "Content-Type: application/json; charset=utf-8".to_string(),
            "{}".to_string(),
        );
        assert!(response.is_json());
        assert!(!response.is_xml());
        assert_eq!(
            response.content_type(),
            Some("application/json; charset=utf-8")
        );
    }

    #[test]
    fn test_content_length() {
        let response = Response::new_unchecked(
            200,
            "Content-Length: 123".to_string(),
            "test body".to_string(),
        );
        assert_eq!(response.content_length(), Some(123));
    }
}
