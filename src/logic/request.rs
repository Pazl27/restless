use anyhow::Result;
use reqwest::{Client, Method};

pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
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

pub async fn send_request(req: &Request) -> Result<String> {
    let client = Client::new();
    let mut request_builder = client.request((&req.method).into(), &req.url);

    for (key, value) in &req.headers {
        request_builder = request_builder.header(key, value);
    }

    if !req.body.is_none() {
        request_builder = request_builder.body(req.body.clone().unwrap_or_default());
    }

    let response = request_builder.send().await?;
    let text = response.text().await?;
    Ok(text)
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_request() {
        let req = Request {
            url: "http://httpbin.org/get".to_string(),
            method: Method::GET,
            headers: vec![],
            body: None,
        };

        let response = send_request(&req).await;
        assert!(response.is_ok());
        assert!(response.unwrap().contains("url"));
    }
}
