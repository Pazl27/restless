use anyhow::Result;
use reqwest::{Client, Method, Response as ReqwestResponse};

pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub params: Vec<(String, String)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl From<&HttpMethod> for Method {
    fn from(method: &HttpMethod) -> Self {
        match method {
            HttpMethod::GET => Method::GET,
            HttpMethod::POST => Method::POST,
            HttpMethod::PUT => Method::PUT,
            HttpMethod::DELETE => Method::DELETE,
        }
    }
}

impl TryFrom<&Method> for HttpMethod {
    type Error = ();

    fn try_from(method: &Method) -> Result<Self, Self::Error> {
        match *method {
            Method::GET => Ok(HttpMethod::GET),
            Method::POST => Ok(HttpMethod::POST),
            Method::PUT => Ok(HttpMethod::PUT),
            Method::DELETE => Ok(HttpMethod::DELETE),
            _ => Err(()),
        }
    }
}

impl Request {
    pub async fn send(&self) -> Result<(u16, String, String)> {
        send_request(self).await
    }
}

pub async fn send_request(req: &Request) -> Result<(u16, String, String)> {
    let client = Client::new();
    
    // Build URL with query parameters
    let mut url = req.url.clone();
    if !req.params.is_empty() {
        let query_string: String = req.params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        if url.contains('?') {
            url.push('&');
        } else {
            url.push('?');
        }
        url.push_str(&query_string);
    }
    
    let mut request_builder = client.request((&req.method).into(), &url);

    for (key, value) in &req.headers {
        request_builder = request_builder.header(key, value);
    }

    if let Some(body) = &req.body {
        request_builder = request_builder.body(body.clone());
    }

    let response: ReqwestResponse = request_builder.send().await?;
    let status_code = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or("")))
        .collect::<Vec<_>>()
        .join("\n");
    let body = response.text().await?;

    Ok((status_code, headers, body))
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_request_get() {
        let req = Request {
            url: "http://httpbin.org/get".to_string(),
            method: Method::GET,
            headers: vec![],
            body: None,
            params: vec![],
        };

        let response = send_request(&req).await.unwrap();
        let (status, headers, body) = response;
        assert_eq!(status, 200);
        assert!(headers.contains("content-type"));
        assert!(body.contains("\"url\""));
    }

    #[tokio::test]
    async fn test_send_request_post_with_body() {
        let req = Request {
            url: "http://httpbin.org/post".to_string(),
            method: Method::POST,
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: Some("{\"foo\": \"bar\"}".to_string()),
            params: vec![],
        };

        let response = send_request(&req).await.unwrap();
        let (status, headers, body) = response;
        assert_eq!(status, 200);
        assert!(headers.contains("content-type"));
        assert!(body.contains("\"foo\": \"bar\""));
    }

    #[test]
    fn test_http_method_conversion() {
        let methods = [
            HttpMethod::GET,
            HttpMethod::POST,
            HttpMethod::PUT,
            HttpMethod::DELETE,
        ];
        for m in &methods {
            let reqwest_method: Method = m.into();
            let back = HttpMethod::try_from(&reqwest_method).unwrap();
            assert_eq!(*m, back);
        }
    }

    #[test]
    fn test_url_building_with_params() {
        let req = Request {
            url: "https://api.example.com/users".to_string(),
            method: Method::GET,
            headers: vec![],
            body: None,
            params: vec![
                ("limit".to_string(), "10".to_string()),
                ("page".to_string(), "1".to_string()),
                ("search".to_string(), "john doe".to_string()),
            ],
        };

        // Test URL building logic (we can't easily test the full request without network)
        let mut url = req.url.clone();
        if !req.params.is_empty() {
            let query_string: String = req.params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
        
            if url.contains('?') {
                url.push('&');
            } else {
                url.push('?');
            }
            url.push_str(&query_string);
        }

        assert_eq!(url, "https://api.example.com/users?limit=10&page=1&search=john%20doe");
    }

    #[test]
    fn test_url_building_with_existing_query() {
        let req = Request {
            url: "https://api.example.com/users?existing=true".to_string(),
            method: Method::GET,
            headers: vec![],
            body: None,
            params: vec![
                ("limit".to_string(), "10".to_string()),
            ],
        };

        let mut url = req.url.clone();
        if !req.params.is_empty() {
            let query_string: String = req.params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
        
            if url.contains('?') {
                url.push('&');
            } else {
                url.push('?');
            }
            url.push_str(&query_string);
        }

        assert_eq!(url, "https://api.example.com/users?existing=true&limit=10");
    }
}
