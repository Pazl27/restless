use crate::app::tab::Tab;
use crate::logic::HttpMethod;

#[derive(Eq, PartialEq)]
pub enum CurrentScreen {
    Url,
    Values,
    Response,

    Editing,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,

    pub tabs: Vec<Tab>,
    pub selected_tab: usize,
    pub url_input: String,

    pub selected_method: HttpMethod,
    pub method_dropdown_open: bool,
    pub method_dropdown_selected: usize,

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
            tabs,
            selected_tab: 0,
            url_input: String::new(),
            selected_method: HttpMethod::GET,
            method_dropdown_open: false,
            method_dropdown_selected: 0,
            response_tab_selected: 1,
            response_scroll: 0,
            response_scroll_state: ratatui::widgets::ScrollbarState::default(),
        }
    }

    pub fn save_current_tab_state(&mut self) {
        if let Some(tab) = self.tabs.get_mut(self.selected_tab) {
            tab.request.url = self.url_input.clone();
            tab.request.method = (&self.selected_method).into();
        }
    }

    pub fn restore_current_tab_state(&mut self) {
        if let Some(tab) = self.tabs.get(self.selected_tab) {
            self.url_input = tab.request.url.clone();
            self.selected_method = HttpMethod::try_from(&tab.request.method).unwrap_or(HttpMethod::GET);
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
