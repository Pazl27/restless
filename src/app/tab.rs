use crate::app::HttpMethod;

pub struct Tab {
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
}

impl Tab {
    pub fn new(name: String, url: String) -> Self {
        Tab {
            name,
            url,
            method: HttpMethod::Get,
        }
    }
}
