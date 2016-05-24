extern crate toml;

pub mod command;
pub mod config;
pub mod ui;
mod script;

use toml::Value;
use ui::{ApplicationUI,EventHandler,CommandOutput,AddressUpdateOutput};

pub const WEBKITTEN_APP_ID: &'static str = "me.delisa.webkitten";
pub const WEBKITTEN_TITLE: &'static str = "webkitten";

pub struct Engine {
    config: Value,
    config_path: String
}

impl Engine {

    /// Create a new running application
    pub fn run<T: ApplicationUI>(config_path: &str) -> Option<T> {
        match config::parse_config_file(config_path.clone()) {
            Some(config) => {
                let engine = Engine {
                    config: config,
                    config_path: String::from(config_path)
                };
                let ui = T::new(engine);
                if ui.is_some() {
                    Some(ui.unwrap())
                } else {
                    None
                }
            },
            None => None
        }
    }
}

impl EventHandler for Engine {

    fn execute_command<T: ApplicationUI>(&self, ui: &T, window_index: u8, webview_index: u8, text: &str)
        -> CommandOutput {
        CommandOutput { error: None, message: None }
    }

    fn update_address<T: ApplicationUI>(&self, ui: &T, window_index: u8, webview_index: u8, text: &str)
        -> AddressUpdateOutput {
        AddressUpdateOutput { error: None, message: None }
    }

    fn close<T: ApplicationUI>(&self, ui: &T) {
    }

    fn command_completions<T: ApplicationUI>(&self, ui: &T, prefix: &str)
        -> Vec<String> {
        vec![]
    }

    fn address_completions<T: ApplicationUI>(&self, ui: &T, prefix: &str)
        -> Vec<String> {
        vec![]
    }
}
