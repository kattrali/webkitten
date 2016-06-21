mod application;
mod webview;
mod window;

use webkitten::ui::{ApplicationUI,EventHandler};
use webkitten::Engine;

pub struct CocoaUI {
    pub engine: Engine
}

impl ApplicationUI for CocoaUI {

    fn new(engine: Engine) -> Option<Self> {
        Some(CocoaUI {engine: engine})
    }

    fn event_handler(&self) -> &Engine {
       &self.engine
    }

    fn run(&self) {
        if let Some(start_page) = self.event_handler().config.lookup("window.start-page") {
            self.open_window(start_page.as_str());
        }
        application::start_run_loop();
    }

    fn register_content_filters(&self, identifier: &str, rules: &str) {
    }

    fn open_window(&self, uri: Option<&str>) {
        window::open(uri);
    }

    fn focused_window_index(&self) -> u8 {
        0
    }

    fn focus_window(&self, index: u8) {
    }

    fn window_count(&self) -> u8 {
        0
    }

    fn toggle_window(&self, window_index: u8, visible: bool) {
    }

    fn resize_window(&self, window_index: u8, width: u32, height: u32) {
    }

    fn address_field_text(&self, window_index: u8) -> String {
        String::new()
    }

    fn set_address_field_text(&self, window_index: u8, text: &str) {
    }

    fn command_field_text(&self, window_index: u8) -> String {
        String::new()
    }

    fn set_command_field_text(&self, window_index: u8, text: &str) {
    }

    fn window_title(&self, window_index: u8) -> String {
        String::new()
    }

    fn set_window_title(&self, window_index: u8, title: &str) {
    }

    fn focused_webview_index(&self, window_index: u8) -> u8 {
        0
    }

    fn open_webview(&self, window_index: u8, uri: &str) {
    }

    fn close_webview(&self, window_index: u8, webview_index: u8) {
        window::close_webview(window_index, webview_index);
    }

    fn focus_webview(&self, window_index: u8, webview_index: u8) {
    }

    fn set_uri(&self, window_index: u8, webview_index: u8, uri: &str) {
        println!("Setting URI");
        if let Some(webview) = window::webview(window_index, webview_index) {
            println!("Loading URI: {}", uri);
            webview::load_uri(webview, uri);
        }
    }

    fn go_back(&self, window_index: u8, webview_index: u8) -> bool {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::go_back(webview)
        } else {
            false
        }
    }

    fn go_forward(&self, window_index: u8, webview_index: u8) -> bool {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::go_forward(webview)
        } else {
            false
        }
    }

    fn raw_html(&self, window_index: u8, webview_index: u8, uri: &str) -> String {
        String::new()
    }

    fn uri(&self, window_index: u8, webview_index: u8) -> String {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::title(webview)
        } else {
            String::new()
        }
    }

    fn webview_title(&self, window_index: u8, webview_index: u8) -> String {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::uri(webview)
        } else {
            String::new()
        }
    }

    fn run_javascript(&self, window_index: u8, webview_index: u8, script: &str) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview::run_javascript(webview, script)
        }
    }

    fn apply_styles(&self, window_index: u8, webview_index: u8, styles: &str) {
    }
}

