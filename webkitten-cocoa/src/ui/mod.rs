pub mod application;
pub mod window;

use std::env;
use std::fs::File;
use std::io::Read;

use webkitten::ui::{ApplicationUI,BrowserConfiguration,WindowArea};
use webkitten::Engine;
use webkitten::optparse::parse_opts;
use macos::foundation::{NSURLRequest,NSURL,NSString,NSAutoreleasePool};
use macos::appkit::{NSPasteboard,nsapp};
use macos::webkit::*;
use macos::{Id,nil};
use block::ConcreteBlock;

use runtime::log_error_description;


const DEFAULT_CONFIG_PATH: &'static str = ".config/webkitten/config.toml";

lazy_static! {
    pub static ref UI: CocoaUI = {
        if let Some(home_dir) = env::home_dir() {
            let default_config_path = &format!("{}/{}", home_dir.display(), DEFAULT_CONFIG_PATH);
            parse_opts(default_config_path)
                .and_then(|run_config| Engine::new(run_config))
                .and_then(|engine| CocoaUI::new(engine))
                .unwrap_or_else(|| panic!("Unable to initialize application"))
        } else {
            panic!("Unable to locate home directory");
        }
    };
}


pub struct CocoaUI {
    pub engine: Engine
}

impl CocoaUI {

    fn compile_content_extensions<F>(&self, completion: F)
        where F: Fn(bool) + 'static {
        let filter_path = self.engine.config.content_filter_path();
        if let Some(mut file) = filter_path.and_then(|p| File::open(p).ok()) {
            let mut contents = String::new();
            if let Some(_) = file.read_to_string(&mut contents).ok() {
                let block = ConcreteBlock::new(move |_: Id, err: Id| {
                    log_error_description(err);
                    completion(err == nil);
                });
                let store = _WKUserContentExtensionStore::default_store();
                store.compile_content_extension("filter", &contents, &block.copy());
            }
        }
    }

    fn open_first_window(&self) {
        if !self.engine.initial_pages().is_empty() {
            for page in self.engine.initial_pages() {
                self.open_window(Some(page.as_str()));
            }
        } else if let Some(page) = self.engine.config.start_page() {
            self.open_window(Some(&page));
        } else {
            self.open_window(None);
        }
    }

    pub fn create_request(uri: &str) -> NSURLRequest {
        let mut target = String::from(uri);
        if !target.contains("://") {
            target = format!("http://{}", target);
        }
        NSURLRequest::from(NSURL::from(NSString::from(&target)))
    }
}

impl ApplicationUI for CocoaUI {

    fn new(engine: Engine) -> Option<Self> {
        Some(CocoaUI { engine: engine })
    }

    fn event_handler(&self) -> &Engine {
       &self.engine
    }

    fn run(&self) {
        let pool = NSAutoreleasePool::new();
        self.compile_content_extensions(|_| {});
        let delegate = application::initialize_app_env();
        self.open_first_window();
        application::start_run_loop(&delegate);
        pool.drain();
    }

    fn copy(&self, text: &str) {
        NSPasteboard::general().copy(text);
    }

    fn open_window(&self, uri: Option<&str>) -> u32 {
        if uri.is_some() {
            window::open(uri)
        } else {
            window::open(self.engine.config.start_page())
        }
    }

    fn close_window(&self, index: u32) {
        window::close(index);
    }

    fn focused_window_index(&self) -> Option<u32> {
        window::focused_index()
    }

    fn focus_window(&self, index: u32) {
        window::focus(index);
    }

    fn focus_window_area(&self, index: u32, area: WindowArea) {
        window::focus_area(index, area);
    }

    fn window_count(&self) -> u32 {
        nsapp().ordered_windows().count() as u32
    }

    fn toggle_window(&self, window_index: u32, visible: bool) {
        window::toggle(window_index, visible);
    }

    fn resize_window(&self, window_index: u32, width: u32, height: u32) {
        window::resize(window_index, width, height);
    }

    fn command_field_text(&self, window_index: u32) -> String {
        window::command_field_text(window_index)
    }

    fn command_field_visible(&self, window_index: u32) -> bool {
        window::command_field_visible(window_index)
    }

    fn set_command_field_text(&self, window_index: u32, text: &str) {
        window::set_command_field_text(window_index, text);
    }

    fn set_command_field_visible(&self, window_index: u32, visible: bool) {
        window::set_command_field_visible(window_index, visible);
    }

    fn window_title(&self, window_index: u32) -> String {
        window::title(window_index)
    }

    fn set_window_title(&self, window_index: u32, title: &str) {
        window::set_title(window_index, title);
    }

    fn focused_webview_index(&self, window_index: u32) -> Option<u32> {
        window::focused_webview_index(window_index)
    }

    fn webview_count(&self, window_index: u32) -> u32 {
        window::webview_count(window_index)
    }

    fn open_webview(&self, window_index: u32, uri: Option<&str>) {
        if let Some(uri) = uri {
            window::open_webview(window_index, uri);
        } else if let Some(uri) = self.engine.config.start_page() {
            window::open_webview(window_index, uri);
        } else {
            warn!("Skipping opening an empty buffer");
        }
    }

    fn close_webview(&self, window_index: u32, webview_index: u32) {
        window::close_webview(window_index, webview_index);
    }

    fn focus_webview(&self, window_index: u32, webview_index: u32) {
        window::focus_webview(window_index, webview_index);
    }

    fn reload_webview(&self, window_index: u32, webview_index: u32, disable_filters: bool) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            match disable_filters {
                true  => webview.reload_without_content_blockers(),
                false => webview.reload()
            }
        }
    }

    fn set_uri(&self, window_index: u32, webview_index: u32, uri: &str) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview.load_request(CocoaUI::create_request(uri));
        }
    }

    fn go_back(&self, window_index: u32, webview_index: u32) -> bool {
        if let Some(webview) = window::webview(window_index, webview_index) {
            if webview.can_go_back() {
                webview.go_back();
                return true;
            }
        }
        false
    }

    fn go_forward(&self, window_index: u32, webview_index: u32) -> bool {
        if let Some(webview) = window::webview(window_index, webview_index) {
            if webview.can_go_forward() {
                webview.go_forward();
                return true;
            }
        }
        false
    }

    fn uri(&self, window_index: u32, webview_index: u32) -> String {
        String::from(window::webview(window_index, webview_index)
            .and_then(|webview| webview.url())
            .and_then(|u| u.absolute_string().as_str())
            .unwrap_or(""))
    }

    fn webview_title(&self, window_index: u32, webview_index: u32) -> String {
        String::from(window::webview(window_index, webview_index)
            .and_then(|webview| webview.title())
            .and_then(|title| title.as_str())
            .unwrap_or(""))
    }

    fn find_string(&self, window_index: u32, webview_index: u32, query: &str) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview.find_string(query)
        }
    }

    fn hide_find_results(&self, window_index: u32, webview_index: u32) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview.hide_find_results()
        }
    }

    fn run_javascript(&self, window_index: u32, webview_index: u32, script: &str) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            webview.evaluate_javascript(script)
        }
    }

    fn apply_styles(&self, window_index: u32, webview_index: u32, styles: &str) {
        if let Some(webview) = window::webview(window_index, webview_index) {
            let controller = webview.configuration().user_content_controller();
            if controller.can_add_user_style_sheet() {
                let sheet = _WKUserStyleSheet::new(styles);
                controller.add_user_style_sheet(sheet);
            } else {
                info!("Using fallback stylesheet");
                let formatted_style = styles.replace("\"", "\\\"").replace("\n", "");
                let script = format!(r#"
                    var head = document.getElementsByTagName('head')[0],
                        style = document.createElement('style'),
                        content = document.createTextNode('{}');
                    style.appendChild(content);
                    if (head != undefined) {{
                        head.appendChild(style);
                        }}"#, formatted_style);
                webview.evaluate_javascript(&script);
            }
        }
    }
}

