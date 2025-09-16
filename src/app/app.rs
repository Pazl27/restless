use crate::app::tab::Tab;
use crate::logic::HttpMethod;

#[derive(Eq, PartialEq)]
pub enum CurrentScreen {
    Url,
    Values,
    Response,

    EditingUrl,
    EditingBody,
    EditingHeaders,
    EditingParams,
    Exiting,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ValuesScreen {
    Body,
    Headers,
    Params,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub values_screen: ValuesScreen,

    pub tabs: Vec<Tab>,
    pub selected_tab: usize,
    pub url_input: String,

    pub selected_method: HttpMethod,
    pub method_dropdown_open: bool,
    pub method_dropdown_selected: usize,

    pub body_input: String,
    pub headers_input: Vec<(String, String)>,
    pub params_input: Vec<(String, String)>,
    pub current_header_key: String,
    pub current_header_value: String,
    pub current_param_key: String,
    pub current_param_value: String,
    pub editing_header_index: Option<usize>,
    pub editing_param_index: Option<usize>,

    pub response_tab_selected: usize,
    pub response_scroll: usize,
    pub response_scroll_state: ratatui::widgets::ScrollbarState,
}

impl App {
    pub fn new() -> App {
        let tabs = vec![
            // Testing with some initial tabs
            Tab::new("Tab 1".to_string(), "http://example.com".to_string()),
            Tab::new("Tab 2".to_string(), "http://example.org".to_string()),
            Tab::new("Tab 3".to_string(), "http://example.net".to_string()),
        ];
        App {
            current_screen: CurrentScreen::Values,
            values_screen: ValuesScreen::Body,
            tabs,
            selected_tab: 0,
            url_input: String::new(),
            selected_method: HttpMethod::GET,
            method_dropdown_open: false,
            method_dropdown_selected: 0,
            body_input: String::new(),
            headers_input: Vec::new(),
            params_input: Vec::new(),
            current_header_key: String::new(),
            current_header_value: String::new(),
            current_param_key: String::new(),
            current_param_value: String::new(),
            editing_header_index: None,
            editing_param_index: None,
            response_tab_selected: 1,
            response_scroll: 0,
            response_scroll_state: ratatui::widgets::ScrollbarState::default(),
        }
    }

    pub fn add_header(&mut self) {
        if !self.current_header_key.is_empty() {
            if self.current_header_key.contains(':') {
                let parts: Vec<&str> = self.current_header_key.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    self.headers_input.push((key, value));
                }
            } else if !self.current_header_value.is_empty() {
                self.headers_input.push((self.current_header_key.clone(), self.current_header_value.clone()));
            }
            self.current_header_key.clear();
            self.current_header_value.clear();
        }
    }

    pub fn add_param(&mut self) {
        if !self.current_param_key.is_empty() {
            if self.current_param_key.contains('=') {
                let parts: Vec<&str> = self.current_param_key.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    self.params_input.push((key, value));
                }
            } else if !self.current_param_value.is_empty() {
                self.params_input.push((self.current_param_key.clone(), self.current_param_value.clone()));
            }
            self.current_param_key.clear();
            self.current_param_value.clear();
        }
    }

    pub fn remove_header(&mut self, index: usize) {
        if index < self.headers_input.len() {
            self.headers_input.remove(index);
        }
    }

    pub fn remove_param(&mut self, index: usize) {
        if index < self.params_input.len() {
            self.params_input.remove(index);
        }
    }

    pub fn save_current_tab_state(&mut self) {
        if let Some(tab) = self.tabs.get_mut(self.selected_tab) {
            tab.request.url = self.url_input.clone();
            tab.request.method = (&self.selected_method).into();
            tab.request.body = if self.body_input.is_empty() { None } else { Some(self.body_input.clone()) };
            tab.request.headers = self.headers_input.clone();
            tab.request.params = self.params_input.clone();
        }
    }

    pub fn restore_current_tab_state(&mut self) {
        if let Some(tab) = self.tabs.get(self.selected_tab) {
            self.url_input = tab.request.url.clone();
            self.selected_method = HttpMethod::try_from(&tab.request.method).unwrap_or(HttpMethod::GET);
            self.body_input = tab.request.body.clone().unwrap_or_default();
            self.headers_input = tab.request.headers.clone();
            self.params_input = tab.request.params.clone();
        }
    }

    pub fn next_tab(&mut self) {
        self.save_current_tab_state();
        self.selected_tab = (self.selected_tab + 1) % self.tabs.len();
        self.restore_current_tab_state();
    }

    pub fn prev_tab(&mut self) {
        self.save_current_tab_state();
        if self.selected_tab == 0 {
            self.selected_tab = self.tabs.len() - 1;
        } else {
            self.selected_tab -= 1;
        }
        self.restore_current_tab_state();
    }
}
