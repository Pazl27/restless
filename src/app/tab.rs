pub struct Tab {
    pub name: String,
    pub url: String,
}

impl Tab {
    pub fn new(name: String, url: String) -> Self {
        Tab { name, url }
    }
}
