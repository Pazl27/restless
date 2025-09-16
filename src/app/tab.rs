use crate::logic::{request::Request, response::Response, HttpMethod};

pub struct Tab {
    pub name: String,
    pub request: Request,
    pub response: Option<Response>,
}

impl Tab {
    pub fn new(name: String, url: String) -> Self {
        Tab {
            name,
            request: Request {
                url: url.clone(),
                method: (&HttpMethod::GET).into(),
                headers: vec![],
                body: None,
                params: vec![],
            },
            response: None,
        }
    }

    pub fn method(&self) -> HttpMethod {
        HttpMethod::try_from(&self.request.method).unwrap_or(HttpMethod::GET)
    }

    pub fn ulr(&self) -> &str {
        &self.request.url
    }
}
