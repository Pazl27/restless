use anyhow::Result;
use reqwest::{Client, Method, Response as ReqwestResponse};

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
    let mut request_builder = client.request((&req.method).into(), &req.url);

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
}
