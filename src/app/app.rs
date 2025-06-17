use crate::app::tab::Tab;

pub enum CurrentScreen {
    Url,
    Values,
    Response,

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

    pub fn save_current_tab_state(&mut self) {
        if let Some(tab) = self.tabs.get_mut(self.selected_tab) {
            tab.url = self.url_input.clone();
            tab.method = self.selected_method;
        }
    }

    pub fn restore_current_tab_state(&mut self) {
        if let Some(tab) = self.tabs.get(self.selected_tab) {
            self.url_input = tab.url.clone();
            self.selected_method = tab.method;
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
