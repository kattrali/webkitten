extern crate toml;

pub mod command;
pub mod config;
pub mod ui;

use toml::Value;
use ui::{ApplicationUI,BrowserWindow,EventHandler};

pub const WEBKITTEN_APP_ID: &'static str = "me.delisa.webkitten";
pub const WEBKITTEN_TITLE: &'static str = "webkitten";

pub struct Application<T: ApplicationUI> {
    ui: T,
    config: Value,
    config_path: String
}

struct Messenger<'a, T: 'a + ApplicationUI> {
    app: &'a Application<T>
}

impl<T> Application<T> where T: ApplicationUI {

    /// Create a new application instance
    pub fn new(config_path: &str) -> Option<Self> {
        let ui = ApplicationUI::new();
        if ui.is_some() {
            match config::parse_config_file(config_path.clone()) {
                Some(config) => Some(Application {
                    ui: ui.unwrap(),
                    config: config,
                    config_path: String::from(config_path)
                }),
                None => None
            }
        } else {
            None
        }
    }
}

