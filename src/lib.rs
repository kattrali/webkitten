extern crate url;
extern crate toml;
extern crate getopts;
#[macro_use]
extern crate log;

pub mod command;
pub mod config;
pub mod ui;
pub mod optparse;
mod script;

use ui::*;

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

    /// Reload configuration from path
    pub fn reload(&mut self) -> bool {
        self.config.load(&self.run_config.path)
    }

    fn use_argument_completion(&self, prefix: &str) -> bool {
        prefix.contains(" ")
    }
}

impl EventHandler for Engine {

    fn execute_command<T: ApplicationUI>(&self,
                                         ui: &T,
                                         window_index: u8,
                                         text: &str)
                                         -> CommandOutput {
        if let Some(text) = self.config.command_matching_prefix(text) {
            return self.execute_command(ui, window_index, &text);
        } else if let Some(command) = command::Command::parse(text, &self.config, COMMAND_FILE_SUFFIX) {
            info!("Found command match: {}", command.path);
            if let Some(file) = command.file() {
                match script::execute::<T>(file, command.arguments, ui) {
                    Err(err) => warn!("{}", err),
                    Ok(success) => if success { ui.set_command_field_text(window_index, "") }
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
        CommandOutput { error: None, message: None }
    }

    fn close<T: ApplicationUI>(&self, ui: &T) {
        unimplemented!()
    }

    fn command_completions<T: ApplicationUI>(&self, ui: &T, prefix: &str)
        -> Vec<String> {
        if self.use_argument_completion(prefix) {
            if let Some(command) = command::Command::parse(prefix, &self.config, COMMAND_FILE_SUFFIX) {
                info!("Found command match for completion: {}", prefix);
                if let Some(file) = command.file() {
                    info!("Completing command text using {}", command.path);
                    return match script::autocomplete::<T>(file, command.arguments, prefix, ui) {
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

    fn on_uri_event<T: ApplicationUI>(&self,
                                      ui: &T,
                                      window_index: u8,
                                      webview_index: u8,
                                      uri: &str,
                                      event: URIEvent) {
        for name in self.config.on_uri_event_commands(event) {
            if let Some(command) = command::Command::parse(&name, &self.config, COMMAND_FILE_SUFFIX) {
                if let Some(file) = command.file() {
                    match script::on_uri_event::<T>(file, ui, window_index, webview_index, uri, event) {
                        Err(err) => warn!("{}", err),
                        Ok(_) => (),
                    }
                }
            }
        }
    }
}

