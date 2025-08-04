use serde_json::{Value, to_string_pretty};

pub struct Response {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, headers: String, body: String) -> Self {
        Response {
            status_code,
            headers: Response::split_headers(&headers),
            body: Response::pretty_print_json(&body),
        }
    }

    fn pretty_print_json(raw_json: &str) -> String {
        match serde_json::from_str::<Value>(raw_json) {
            Ok(json_value) => {
                to_string_pretty(&json_value).unwrap_or_else(|_| raw_json.to_string())
            }
            Err(_) => raw_json.to_string(),
        }
    }

    fn split_headers(header: &str) -> Vec<(String, String)> {
        header
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, ':');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    Some((key.trim().to_string(), value.trim().to_string()))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_new_with_invalid_json() {
        let response = Response::new(
            404,
            "X-Test: test".to_string(),
            "not a json".to_string(),
        );
        assert_eq!(response.status_code, 404);
        assert_eq!(response.headers.len(), 1);
        assert_eq!(response.headers[0].0, "X-Test");
        assert_eq!(response.headers[0].1, "test");
        assert_eq!(response.body, "not a json");
    }

    #[test]
    fn test_split_headers_with_empty_string() {
        let headers = Response::split_headers("");
        assert!(headers.is_empty());
    }

    #[test]
    fn test_split_headers_with_malformed_lines() {
        let headers = Response::split_headers("Good: header\nMalformedHeader\nAnother: one");
        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0], ("Good".to_string(), "header".to_string()));
        assert_eq!(headers[1], ("Another".to_string(), "one".to_string()));
    }

    #[test]
    fn test_split_headers_with_extra_colons() {
        let headers = Response::split_headers("Foo: bar: baz");
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0], ("Foo".to_string(), "bar: baz".to_string()));
    }

    #[test]
    fn test_pretty_print_json_with_invalid_json() {
        let raw = "not json";
        let pretty = Response::pretty_print_json(raw);
        assert_eq!(pretty, "not json");
    }

    #[test]
    fn test_pretty_print_json_with_array() {
        let raw = r#"[{"a":1},{"b":2}]"#;
        let pretty = Response::pretty_print_json(raw);
        assert_eq!(pretty, "[\n  {\n    \"a\": 1\n  },\n  {\n    \"b\": 2\n  }\n]");
    }
}
