extern crate libc;
extern crate gtk_sys;
extern crate gobject_sys;
extern crate glib_sys;
extern crate toml;

pub mod command;
pub mod config;
pub mod ui;
mod webkitgtk_sys;

use ui::Window;
use libc::{c_char,c_int};
use toml::Value;

pub const WEBKITTEN_APP_ID: &'static str = "me.delisa.webkitten";
pub const WEBKITTEN_TITLE: &'static str = "webkitten";

pub struct Application {
    windows: Vec<Window>,
    config: Value,
    config_path: String
}

impl Application {

    /// Create a new application instance
    pub fn new(config_path: &str) -> Option<Self> {
        match config::parse_config_file(config_path.clone()) {
            Some(config) => Some(Application {
                windows: vec![],
                config: config,
                config_path: String::from(config_path)
            }),
            None => None
        }
    }

    pub fn run(&mut self) {
        with_gtk_app(|| if self.windows.is_empty() { self.add_window(); })
    }

    pub fn add_window(&mut self) {
        let mut window = Window::new(WEBKITTEN_TITLE);
        window.add_webview("http://delisa.me");
        window.show();
        self.windows.push(window);
    }
}

fn with_gtk_app<F>(callback: F) where F: FnOnce() -> () {
    unsafe { gtk_sys::gtk_init(0 as *mut c_int, 0 as *mut *mut *mut c_char); }
    callback();
    unsafe { gtk_sys::gtk_main(); }
}

