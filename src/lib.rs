extern crate toml;
extern crate getopts;

pub mod command;
pub mod config;
pub mod ui;
pub mod optparse;
mod script;

use std::path::{Path,PathBuf};
use toml::Value;
use ui::{ApplicationUI,EventHandler,CommandOutput,AddressUpdateOutput};

pub const WEBKITTEN_APP_ID: &'static str = "me.delisa.webkitten";
pub const WEBKITTEN_TITLE: &'static str = "webkitten";

pub struct Engine {
    pub config: Value,
    config_path: String
}

impl Engine {

    /// Create a new application engine
    pub fn new(config_path: &str) -> Option<Self> {
        config::parse_config_file(config_path).and_then(|config| {
            Some(Engine {
                config: config,
                config_path: String::from(config_path)
            })
        })
    }

    /// Reload configuration from path
    pub fn reload(&mut self) -> bool {
        if let Some(config) = config::parse_config_file(&self.config_path) {
            self.config = config;
            return true;
        }
        false
    }

    fn command_search_path(&self) -> Option<PathBuf> {
        let path = Path::new(&self.config_path);
        path.parent().and_then(|path| Some(path.join("scripts")))
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
