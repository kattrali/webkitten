extern crate toml;
extern crate getopts;
#[macro_use]
extern crate log;

pub mod command;
pub mod config;
pub mod ui;
pub mod optparse;
mod script;

use std::path::{Path,PathBuf};
use toml::Value;
use ui::{ApplicationUI,EventHandler,CommandOutput,AddressUpdateOutput};

/// Application identifier for apps built with webkitten core
pub const WEBKITTEN_APP_ID: &'static str = "me.delisa.webkitten";
/// Application title for apps built with webkitten core
pub const WEBKITTEN_TITLE: &'static str = "webkitten";

/// Placeholder used in webkitten configuration to represent the configuration
/// property `general.config-dir`.
const CONFIG_DIR: &'static str = "CONFIG_DIR";

pub struct Engine {
    pub config: Value,
    config_path: String
}

impl Engine {

    /// Create a new application engine
    pub fn new(config_path: &str) -> Option<Self> {
        config::parse_config_file(config_path).and_then(|config| {
            info!("Creating application engine with config path: {}", config_path);
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

    fn command_search_paths(&self) -> Vec<String> {
        let mut config_dir: Option<&str> = None;
        let mut resolved_paths: Vec<String> = vec![];
        if let Some(path) = self.config.lookup("general.config-dir") {
            config_dir = path.as_str();
        }
        if let Some(load_paths) = self.config.lookup("commands.search-paths").and_then(|p| p.as_slice()) {
            for mut value_path in load_paths {
                if let Some(path) = value_path.as_str() {
                    if let Some(config_dir) = config_dir {
                        resolved_paths.push(path.replace(CONFIG_DIR, config_dir));
                    } else {
                        resolved_paths.push(String::from(path))
                    }
                }
            }
        }
        resolved_paths
    }
}

impl EventHandler for Engine {

    fn execute_command<T: ApplicationUI>(&self, ui: &T, window_index: u8, webview_index: u8, text: &str)
        -> CommandOutput {
        let search_paths = self.command_search_paths();
        if let Some(command) = command::Command::parse(text, search_paths) {
            info!("Found command match: {}", text);
            if let Some(file) = command.file() {
                info!("Running a command: {}", command.path);
                if script::execute::<T>(file, command.arguments, ui) {
                    ui.set_command_field_text(window_index, "");
                }
            }
        }
        CommandOutput { error: None, message: None }
    }

    fn update_address<T: ApplicationUI>(&self, ui: &T, window_index: u8, webview_index: u8, text: &str)
        -> AddressUpdateOutput {
        info!("Updating the address with: {}", text);
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

