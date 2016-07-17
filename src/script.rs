extern crate hlua;

use std::error::Error;
use std::fs::File;
use std::fmt;

use self::hlua::{Lua,LuaError,function0,function1,function2,function3};
use self::hlua::any::AnyLuaValue;
use self::hlua::functions_read::LuaFunction;
use super::ui::{ApplicationUI,BrowserConfiguration,URIEvent};
use super::config::Config;


const INVALID_RESULT: u8 = 247;

pub type ScriptResult<T> = Result<T, ScriptError>;

#[derive(Debug)]
pub struct ScriptError {
    description: String
}

impl ScriptError {

    fn new(description: &str, error: Option<LuaError>) -> Self {
        let mut full_description = String::from(description);
        if let Some(error) = error {
            full_description.push_str(": ");
            match error {
                LuaError::SyntaxError(err) => full_description.push_str(&err),
                LuaError::ExecutionError(err) => full_description.push_str(&err),
                LuaError::ReadError(err) => full_description.push_str(err.description()),
                LuaError::WrongType => full_description.push_str("incorrect data type"),
            };
        }
        ScriptError { description: full_description }
    }
}

impl Error for ScriptError {

    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

pub fn execute<T: ApplicationUI>(file: File, arguments: Vec<String>, ui: &T) -> ScriptResult<bool> {
    let mut lua = create_runtime::<T>(ui);
    lua.set("arguments", arguments);
    if let Err(err) = lua.execute_from_reader::<(), _>(file) {
        Err(ScriptError::new("script parsing failed", Some(err)))
    } else {
        let run: Option<LuaFunction<_>> = lua.get("run");
        if let Some(mut run) = run {
            resolve_script_output::<bool>(run.call())
        } else {
            Err(ScriptError::new("'run' method missing", None))
        }
    }
}

pub fn autocomplete<T: ApplicationUI>(file: File, arguments: Vec<String>, prefix: &str, ui: &T) -> ScriptResult<Vec<String>> {
    let mut lua = create_runtime::<T>(ui);
    lua.set("prefix", prefix);
    lua.set("arguments", arguments);
    if let Err(err) = lua.execute_from_reader::<(), _>(file) {
        Err(ScriptError::new("script parsing failed", Some(err)))
    } else {
        let complete: Option<LuaFunction<_>> = lua.get("complete_command");
        if let Some(mut complete) = complete {
            resolve_script_output::<AnyLuaValue>(complete.call())
                .and_then(|output| coerce_lua_array(output))
        } else {
            Err(ScriptError::new("'complete_command' method missing", None))
        }
    }
}

pub fn on_uri_event<T: ApplicationUI>(file: File,
                                      ui: &T,
                                      window_index: u8,
                                      webview_index: u8,
                                      uri: &str,
                                      event: URIEvent) -> ScriptResult<()> {
    let mut lua = create_runtime::<T>(ui);
    lua.set("requested_uri", uri);
    lua.set("webview_index", webview_index);
    lua.set("window_index", window_index);
    if let Err(err) = lua.execute_from_reader::<(), _>(file) {
        Err(ScriptError::new("script parsing failed", Some(err)))
    } else {
        let func: Option<LuaFunction<_>> = match event {
            URIEvent::Load => lua.get("on_load_uri"),
            URIEvent::Request => lua.get("on_request_uri"),
            URIEvent::Fail => lua.get("on_fail_uri"),
        };
        if let Some(mut func) = func {
            resolve_script_output::<()>(func.call())
        } else {
            Err(ScriptError::new(&format!("{:?} event method missing", event), None))
        }
    }
}

fn coerce_lua_array(raw_value: AnyLuaValue) -> ScriptResult<Vec<String>> {
    if let AnyLuaValue::LuaString(value) = raw_value {
        if value.len() == 0 {
            Ok(vec![])
        } else {
            Ok(value.split(",").map(|v| String::from(v)).collect())
        }
    } else {
        Err(ScriptError::new("Return type is not an string", None))
    }
}

fn resolve_script_output<T>(output: Result<T, LuaError>) -> ScriptResult<T> {
    output.map_err(|err| ScriptError::new("script failed to execute", Some(err)))
}

fn create_runtime<T: ApplicationUI>(ui: &T) -> Lua {
    let mut lua = Lua::new();
    lua.openlibs();
    lua.set("INVALID_RESULT", INVALID_RESULT);
    lua.set("log_info", function1(|message: String| {
        info!("lua: {}", message);
    }));
    lua.set("log_debug", function1(|message: String| {
        debug!("lua: {}", message);
    }));
    lua.set("copy", function1(|message: String| {
        info!("copy");
        ui.copy(&message);
    }));
    lua.set("config_file_path", ui.event_handler().run_config.path.clone());
    lua.set("lookup_bool", function2(|config_path: String, key: String| {
        info!("lookup_bool ({}): {}", config_path, key);
        if let Some(config) = Config::parse_file(&config_path) {
            return config.lookup_bool(&key).unwrap_or(false)
        }
        false
    }));
    lua.set("lookup_strings", function2(|config_path: String, key: String| {
        info!("lookup_str ({}): {}", config_path, key);
        if let Some(config) = Config::parse_file(&config_path) {
            return config.lookup_str_vec(&key).unwrap_or(vec![])
        }
        vec![]
    }));
    lua.set("lookup_string", function2(|config_path: String, key: String| {
        info!("lookup_str ({}): {}", config_path, key);
        if let Some(config) = Config::parse_file(&config_path) {
            return config.lookup_str(&key).unwrap_or(String::new())
        }
        String::new()
    }));
    lua.set("focus_window", function1(|index: u8| {
        info!("focus_window: {}", index);
        ui.focus_window(index);
    }));
    lua.set("open_window", function1(|uri: String| {
        info!("open_window");
        if uri.len() > 0 {
            ui.open_window(Some(&uri));
        } else {
            ui.open_window(None);
        }
    }));
    lua.set("close_window", function1(|window_index: u8| {
        info!("close_window: {}", window_index);
        ui.close_window(window_index);
    }));
    lua.set("window_count", function0(|| {
        info!("get window_count");
        ui.window_count()
    }));
    lua.set("focused_window_index", function0(|| {
        info!("get focused_window_index");
        ui.focused_window_index()
    }));
    lua.set("hide_window", function1(|window_index: u8| {
        info!("hide_window: {}", window_index);
        ui.toggle_window(window_index, false);
    }));
    lua.set("show_window", function1(|window_index: u8| -> () {
        info!("show_window: {}", window_index);
        ui.toggle_window(window_index, true);
    }));
    lua.set("open_webview", function2(|window_index: u8, uri: String| {
        info!("open_webview: {}", window_index);
        ui.open_webview(window_index, &uri);
    }));
    lua.set("webview_count", function1(|window_index: u8| {
        info!("get webview_count: {}", window_index);
        ui.webview_count(window_index)
    }));
    lua.set("set_command_field_text", function2(|window_index: u8, text: String| {
        info!("set command_field_text");
        ui.set_command_field_text(window_index, &text);
    }));
    lua.set("command_field_text", function1(|window_index: u8| {
        info!("get command_field_text");
        ui.command_field_text(window_index)
    }));
    lua.set("focused_webview_index", function1(|window_index: u8| {
        info!("get focused_webview_index");
        ui.focused_webview_index(window_index)
    }));
    lua.set("resize_window", function3(|window_index: u8, width: u32, height: u32| {
        info!("resize_window: {} => ({}, {})", window_index, width, height);
        ui.resize_window(window_index, width, height);
    }));
    lua.set("close_webview", function2(|window_index: u8, webview_index: u8| {
        info!("close_webview: ({}, {})", window_index, webview_index);
        ui.close_webview(window_index, webview_index);
    }));
    lua.set("reload_webview", function3(|window_index: u8, webview_index: u8, disable_filters: bool| {
        info!("reload_webview: ({}, {})", window_index, webview_index);
        ui.reload_webview(window_index, webview_index, disable_filters);
    }));
    lua.set("focus_webview", function2(|window_index: u8, webview_index: u8| {
        info!("focus_webview: ({}, {})", window_index, webview_index);
        ui.focus_webview(window_index, webview_index);
    }));
    lua.set("load_uri", function3(|window_index: u8, webview_index: u8, uri: String| {
        info!("load_uri: ({}, {})", window_index, webview_index);
        ui.set_uri(window_index, webview_index, &uri);
    }));
    lua.set("go_back", function2(|window_index: u8, webview_index: u8| {
        info!("go_back: ({}, {})", window_index, webview_index);
        ui.go_back(window_index, webview_index);
    }));
    lua.set("go_forward", function2(|window_index: u8, webview_index: u8| {
        info!("go_forward: ({}, {})", window_index, webview_index);
        ui.go_forward(window_index, webview_index);
    }));
    lua.set("webview_uri", function2(|window_index: u8, webview_index: u8| {
        info!("get webview_uri: ({}, {})", window_index, webview_index);
        ui.uri(window_index, webview_index)
    }));
    lua.set("webview_title", function2(|window_index: u8, webview_index: u8| {
        info!("get webview_title: ({}, {})", window_index, webview_index);
        ui.webview_title(window_index, webview_index)
    }));
    lua.set("find", function3(|window_index: u8, webview_index: u8, query: String| {
        info!("find: ({}, {})", window_index, webview_index);
        ui.find_string(window_index, webview_index, &query);
    }));
    lua.set("hide_find", function2(|window_index: u8, webview_index: u8| {
        info!("hide_find: ({}, {})", window_index, webview_index);
        ui.hide_find_results(window_index, webview_index)
    }));
    lua.set("run_javascript", function3(|window_index: u8, webview_index: u8, script: String| {
        info!("run_javascript: ({}, {})", window_index, webview_index);
        ui.run_javascript(window_index, webview_index, &script);
    }));
    lua.set("add_styles", function3(|window_index: u8, webview_index: u8, styles: String| {
        ui.apply_styles(window_index, webview_index, &styles);
    }));
    lua
}
