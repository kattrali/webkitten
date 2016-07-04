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
/// File extension used by command files
const COMMAND_FILE_SUFFIX: &'static str = "lua";

/// The core of a webkitten application. The engine handles configuration options
/// and responding to lifecycle and user events from the UI.
pub struct Engine {
    pub config: config::Config,
    config_path: String
}

impl Engine {

    /// Create a new application engine
    pub fn new(config_path: &str) -> Option<Self> {
        config::Config::parse_file(config_path).and_then(|config| {
            info!("Creating application engine with config path: {}", config_path);
            Some(Engine {
                config: config,
                config_path: String::from(config_path)
            })
        })
    }

    /// Reload configuration from path
    pub fn reload(&mut self) -> bool {
        self.config.load(&self.config_path)
    }

    /// Paths searched for script commands
    fn command_search_paths(&self) -> Vec<String> {
        if let Some(paths) = self.config.lookup_path_slice("commands.search-paths") {
            paths
        } else {
            vec![]
        }
    }

    /// The configuration section values for `alias`
    fn command_aliases(&self) -> Option<&Value> {
        self.config.lookup("alias")
    }
}

impl EventHandler for Engine {

    fn execute_command<T: ApplicationUI>(&self, ui: &T, window_index: u8, webview_index: u8, text: &str)
        -> CommandOutput {
        let search_paths = self.command_search_paths();
        if let Some(command) = command::Command::parse(text, search_paths, self.command_aliases(), COMMAND_FILE_SUFFIX) {
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

