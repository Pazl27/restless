use crate::app::tab::Tab;

pub enum CurrentScreen {
    Url,
    Values,
    Response,

    Main,
    Editing,
    Exiting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

pub struct App {
    pub current_screen: CurrentScreen,

    pub tabs: Vec<Tab>,
    pub selected_tab: usize,
    pub url_input: String,

    pub selected_method: HttpMethod,
    pub method_dropdown_open: bool,
    pub method_dropdown_selected: usize,
}

impl App {
    pub fn new() -> App {
        let tabs = vec![
            Tab::new("Tab 1".to_string(), "http://example.com".to_string()),
            Tab::new("Tab 2".to_string(), "http://example.org".to_string()),
            Tab::new("Tab 3".to_string(), "http://example.net".to_string()),
        ];
        App {
            current_screen: CurrentScreen::Url,
            tabs,
            selected_tab: 0,
            url_input: String::new(),
            selected_method: HttpMethod::Get,
            method_dropdown_open: false,
            method_dropdown_selected: 0,
        }
    }
}
