mod lua;

pub use self::lua::LuaEngine;

use std::error::Error;
use std::fs::File;
use std::fmt;

use super::ui::{ApplicationUI,BufferEvent};

/// A sentinel value for representing empty optional numbers to scripting
/// languages without optionals
pub const NOT_FOUND: u32 = 483600;

pub type ScriptResult<T> = Result<T, ScriptError>;

#[derive(Debug)]
pub struct ScriptError {
    description: String
}

impl Error for ScriptError {

    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

/// A scripting runtime and event handler capable of evaluating file contents
/// within the runtime, converting between internal runtime types and Rust
/// types, and providing an interface to interaction with the UI.
pub trait ScriptingEngine {

    /// The file extension to use when searching for command matches for this
    /// engine
    fn file_extension() -> &'static str;

    /// Evaluate the contents of a file withn the scripting runtime and execute
    /// the description event trigger
    fn describe(file: File) -> ScriptResult<String>;

    /// Evaluate the contents of a file within the scripting runtime and execute
    /// the event trigger for running a command directly, providing the
    /// arguments to the scope
    fn execute<T, S>(file: File, arguments: Vec<String>, ui: &T, config_path: &str) -> ScriptResult<bool>
        where T: ApplicationUI<S>,
              S: ScriptingEngine;

    /// Evaluate the contents of a file within the scripting runtime and execute
    /// the event trigger for getting autocompletion results, providing the
    /// arguments and prefix to the scope
    fn autocomplete<T, S>(file: File, arguments: Vec<String>, prefix: &str, ui: &T, config_path: &str) -> ScriptResult<Vec<String>>
        where T: ApplicationUI<S>,
              S: ScriptingEngine;

    /// Evaluate the contents of a file within the scripting runtime and execute
    /// the event trigger matching the BufferEvent, provided the window index,
    /// webview index, and requested URI to the scope.
    fn on_buffer_event<T, S>(file: File, ui: &T, config_path: &str, window_index: u32,
                             webview_index: u32, requested_uri: Option<&str>,
                             event: &BufferEvent) -> ScriptResult<()>
        where T: ApplicationUI<S>,
              S: ScriptingEngine;
}
