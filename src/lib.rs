extern crate url;
extern crate toml;
extern crate getopts;
#[macro_use]
extern crate log;

pub mod command;
pub mod config;
pub mod ui;
pub mod optparse;
pub mod script;
mod keybinding;

use ui::*;
use script::ScriptingEngine;

/// Application identifier for apps built with webkitten core
pub const WEBKITTEN_APP_ID: &'static str = "me.delisa.Webkitten";
/// Application title for apps built with webkitten core
pub const WEBKITTEN_TITLE: &'static str = "webkitten";

/// The core of a webkitten application. The engine handles configuration options
/// and responding to lifecycle and user events from the UI.
pub struct Engine {
    pub config: config::Config,
    run_config: optparse::RunConfiguration,
}

impl Engine {

    /// Create a new application engine
    pub fn new(runtime: optparse::RunConfiguration) -> Option<Self> {
        config::Config::parse_file(&runtime.path).and_then(|config| {
            info!("Creating application engine with config path: {}", &runtime.path);
            Some(Engine {
                config: config,
                run_config: runtime
            })
        })
    }

    /// Any arguments specified at launch to be opened
    pub fn initial_pages<'a>(&'a self) -> &'a Vec<String> {
        &self.run_config.start_pages
    }

    /// Reload configuration from path
    pub fn reload(&mut self) -> bool {
        self.config.load(&self.run_config.path)
    }

    fn use_argument_completion(&self, prefix: &str) -> bool {
        prefix.contains(" ")
    }
}

impl EventHandler for Engine {

    fn on_new_frame_request<T, S>(&self, ui: &T, window_index: u32, uri: &str)
        where T: ApplicationUI<S>,
              S: ScriptingEngine {
        if self.config.new_frame_uses_focused_window() {
            ui.open_webview::<_, config::Config>(window_index, Some(uri), None);
        } else {
            ui.open_window::<_, config::Config>(Some(uri), None);
        }
    }

    fn execute_command<T, S>(&self, ui: &T, window_index: Option<u32>, text: &str)
        where T: ApplicationUI<S>,
              S: ScriptingEngine {
        if let Some(text) = self.config.command_matching_prefix(text) {
            return self.execute_command(ui, window_index, &text);
        } else if let Some(command) = command::Command::parse(text, &self.config, S::file_extension()) {
            info!("Found command match: {}", command.path);
            if let Some(file) = command.file() {
                match S::execute::<T, S>(file, command.arguments, ui, &self.run_config.path) {
                    Err(err) => warn!("{}", err),
                    Ok(success) => if let (true, Some(index)) = (success, window_index) {
                        ui.set_command_field_text(index, "")
                    }
                }
            }
        } else if let Some(default) = self.config.default_command() {
            if !text.starts_with(&default) {
                let mut command = String::from(default);
                command.push_str(" ");
                command.push_str(text);
                info!("Running the default command: {}", command);
                return self.execute_command(ui, window_index, &command);
            }
        }
    }

    fn close<T, S>(&self, _ui: &T)
        where T: ApplicationUI<S>,
              S: ScriptingEngine {}

    fn command_completions<T, S>(&self, ui: &T, prefix: &str) -> Vec<String>
        where T: ApplicationUI<S>,
              S: ScriptingEngine {
        if self.use_argument_completion(prefix) {
            if let Some(command) = command::Command::parse(prefix, &self.config, S::file_extension()) {
                info!("Found command match for completion: {}", prefix);
                if let Some(file) = command.file() {
                    info!("Completing command text using {}", command.path);
                    return match S::autocomplete::<T, S>(file, command.arguments, prefix, ui, &self.run_config.path) {
                        Err(err) => {
                            warn!("{}", err);
                            vec![]
                        },
                        Ok(completions) => completions
                    }
                }
            }
        }
        command::Command::list_commands(prefix, &self.config)
    }

    fn on_uri_event<T, S>(&self, ui: &T, window_index: u32, webview_index: u32, uri: Option<&str>, event: URIEvent)
        where T: ApplicationUI<S>,
              S: ScriptingEngine {
        for name in self.config.on_uri_event_commands(&event) {
            if let Some(command) = command::Command::parse(&name, &self.config, S::file_extension()) {
                if let Some(file) = command.file() {
                    match S::on_uri_event::<T, S>(file, ui, &self.run_config.path, window_index, webview_index, uri, &event) {
                        Err(err) => warn!("{}", err),
                        Ok(_) => (),
                    }
                }
            }
        }
    }
}

