extern crate hlua;

use std::error::Error;
use std::fs::File;
use std::fmt;

use self::hlua::{Lua,LuaError,function0,function1,function2,function3};
use self::hlua::any::AnyLuaValue;
use self::hlua::functions_read::LuaFunction;
use super::ui::ApplicationUI;

const INVALID_RESULT: u8 = 247;

pub enum CompletionType {
    Address,
    Command,
}


pub type ScriptResult<T> = Result<T, ScriptError>;

#[derive(Debug)]
pub struct ScriptError {
    description: String
}

impl ScriptError {

    fn new(description: &str, error: Option<LuaError>) -> Self {
        let mut full_description = String::from(description);
        if let Some(error) = error {
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

pub fn autocomplete<T: ApplicationUI>(file: File, arguments: Vec<String>, prefix: &str, variant: CompletionType, ui: &T) -> ScriptResult<Vec<String>> {
    let mut lua = create_runtime::<T>(ui);
    lua.set("prefix", prefix);
    lua.set("arguments", arguments);
    if let Err(err) = lua.execute_from_reader::<(), _>(file) {
        Err(ScriptError::new("script parsing failed", Some(err)))
    } else {
        let method = match variant {
            CompletionType::Address => "complete_address",
            CompletionType::Command => "complete_command",
        };
        let complete: Option<LuaFunction<_>> = lua.get(method);
        if let Some(mut complete) = complete {
            resolve_script_output::<AnyLuaValue>(complete.call())
                .and_then(|output| coerce_lua_array(output))
        } else {
            Err(ScriptError::new(&format!("'{}' method missing", method), None))
        }
    }
}

fn coerce_lua_array(raw_value: AnyLuaValue) -> ScriptResult<Vec<String>> {
    if let AnyLuaValue::LuaString(value) = raw_value {
        Ok(value.split(",").map(|v| String::from(v)).collect())
    } else {
        Err(ScriptError::new("Return type is not an string", None))
    }
}

fn resolve_script_output<T>(output: Result<T, LuaError>) -> ScriptResult<T> {
    match output {
        Err(err) => Err(ScriptError::new("script failed to execute", Some(err))),
        Ok(value) => Ok(value)
    }
}

fn create_runtime<T: ApplicationUI>(ui: &T) -> Lua {
    let mut lua = Lua::new();
    lua.openlibs();
    lua.set("INVALID_RESULT", INVALID_RESULT);
    lua.set("focus_window", function1(|index: u8| {
        ui.focus_window(index);
    }));
    lua.set("open_window", function1(|uri: String| {
        if uri.len() > 0 {
            ui.open_window(Some(&uri));
        } else {
            ui.open_window(None);
        }
    }));
    lua.set("close_window", function1(|window_index: u8| {
        ui.close_window(window_index);
    }));
    lua.set("log_info", function1(|message: String| {
        info!("lua: {}", message);
    }));
    lua.set("log_debug", function1(|message: String| {
        debug!("lua: {}", message);
    }));
    lua.set("window_count", function0(|| {
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
        ui.open_webview(window_index, &uri);
    }));
    lua.set("webview_count", function1(|window_index: u8| {
        ui.webview_count(window_index)
    }));
    lua.set("set_address_field_text", function2(|window_index: u8, text: String| {
        ui.set_address_field_text(window_index, &text);
    }));
    lua.set("set_command_field_text", function2(|window_index: u8, text: String| {
        ui.set_command_field_text(window_index, &text);
    }));
    lua.set("address_field_text", function1(|window_index: u8| {
        ui.address_field_text(window_index)
    }));
    lua.set("command_field_text", function1(|window_index: u8| {
        ui.command_field_text(window_index)
    }));
    lua.set("focused_webview_index", function1(|window_index: u8| {
        info!("get focused_webview_index");
        ui.focused_webview_index(window_index)
    }));
    lua.set("resize_window", function3(|window_index: u8, width: u32, height: u32| {
        ui.resize_window(window_index, width, height);
    }));
    lua.set("close_webview", function2(|window_index: u8, webview_index: u8| {
        ui.close_webview(window_index, webview_index);
    }));
    lua.set("focus_webview", function2(|window_index: u8, webview_index: u8| {
        ui.focus_webview(window_index, webview_index);
    }));
    lua.set("load_uri", function3(|window_index: u8, webview_index: u8, uri: String| {
        info!("load_uri: ({}, {}) => {}", window_index, webview_index, uri);
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
    lua.set("webview_title", function2(|window_index: u8, webview_index: u8| {
        ui.webview_title(window_index, webview_index)
    }));
    lua.set("run_javascript", function3(|window_index: u8, webview_index: u8, script: String| {
        ui.run_javascript(window_index, webview_index, &script);
    }));
    lua.set("add_styles", function3(|window_index: u8, webview_index: u8, styles: String| {
        ui.apply_styles(window_index, webview_index, &styles);
    }));
    lua
}
