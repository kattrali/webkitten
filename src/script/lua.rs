extern crate hlua;

use std::error::Error;
use std::fs::File;

use self::hlua::{Lua,LuaError,function0,function1,function2,function3};
use self::hlua::any::AnyLuaValue;
use self::hlua::functions_read::LuaFunction;

use ui::{ApplicationUI,BrowserConfiguration,BufferEvent,WindowArea};
use config::Config;

use super::{ScriptingEngine,ScriptError,ScriptResult,NOT_FOUND};

#[allow(dead_code)]
pub struct LuaEngine;

const FILE_EXTENSION: &'static str = "lua";

impl ScriptingEngine for LuaEngine {

    fn file_extension() -> &'static str {
        FILE_EXTENSION
    }

    fn describe(file: File) -> ScriptResult<String> {
        let mut lua = Lua::new();
        if let Err(err) = lua.execute_from_reader::<(), _>(file) {
            Err(lua_to_script_error("script parsing failed", Some(err)))
        } else {
            let run: Option<LuaFunction<_>> = lua.get("description");
            if let Some(mut run) = run {
                resolve_script_output::<String>(run.call())
            } else {
                Err(lua_to_script_error("'description' method missing", None))
            }
        }
    }

    fn execute<T, S>(file: File, arguments: Vec<String>, ui: &T, config_path: &str) -> ScriptResult<bool>
        where T: ApplicationUI<S>,
              S: ScriptingEngine {
        let mut lua = create_runtime::<T, S>(ui, config_path.to_owned());
        lua.set("arguments", arguments);
        if let Err(err) = lua.execute_from_reader::<(), _>(file) {
            Err(lua_to_script_error("script parsing failed", Some(err)))
        } else {
            let run: Option<LuaFunction<_>> = lua.get("run");
            if let Some(mut run) = run {
                resolve_script_output::<bool>(run.call())
            } else {
                Err(lua_to_script_error("'run' method missing", None))
            }
        }
    }

    fn autocomplete<T, S>(file: File, arguments: Vec<String>, prefix: &str, ui: &T, config_path: &str) -> ScriptResult<Vec<String>>
        where T: ApplicationUI<S>,
              S: ScriptingEngine {
        let mut lua = create_runtime::<T, S>(ui, config_path.to_owned());
        lua.set("prefix", prefix);
        lua.set("arguments", arguments);
        if let Err(err) = lua.execute_from_reader::<(), _>(file) {
            Err(lua_to_script_error("script parsing failed", Some(err)))
        } else {
            let complete: Option<LuaFunction<_>> = lua.get("complete_command");
            if let Some(mut complete) = complete {
                resolve_script_output::<AnyLuaValue>(complete.call())
                    .and_then(|output| coerce_lua_array(output))
            } else {
                Err(lua_to_script_error("'complete_command' method missing", None))
            }
        }
    }

    fn on_buffer_event<T, S>(file: File, ui: &T, config_path: &str, window_index: u32,
                             webview_index: u32, requested_uri: Option<&str>,
                             event: &BufferEvent) -> ScriptResult<()>
        where T: ApplicationUI<S>,
              S: ScriptingEngine {
        let mut lua = create_runtime::<T, S>(ui, config_path.to_owned());
        if let Some(requested_uri) = requested_uri {
            lua.set("requested_uri", requested_uri);
        }
        lua.set("webview_index", webview_index);
        lua.set("window_index", window_index);
        if let Err(err) = lua.execute_from_reader::<(), _>(file) {
            Err(lua_to_script_error("script parsing failed", Some(err)))
        } else {
            let func: Option<LuaFunction<_>> = match event {
                &BufferEvent::Load => lua.get("on_load_uri"),
                &BufferEvent::Focus => lua.get("on_focus"),
                &BufferEvent::Request => lua.get("on_request_uri"),
                &BufferEvent::Fail(ref message) => {
                    lua.set("error_message", message.clone());
                    lua.get("on_fail_uri")
                },
            };
            if let Some(mut func) = func {
                resolve_script_output::<()>(func.call())
            } else {
                Err(lua_to_script_error(&format!("{:?} event method missing", event), None))
            }
        }
    }
}

fn lua_to_script_error(description: &str, error: Option<LuaError>) -> ScriptError {
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

fn coerce_lua_array(raw_value: AnyLuaValue) -> ScriptResult<Vec<String>> {
    if let AnyLuaValue::LuaString(value) = raw_value {
        if value.len() == 0 {
            Ok(vec![])
        } else {
            Ok(value.split(",").map(|v| String::from(v)).collect())
        }
    } else {
        Err(lua_to_script_error("Return type is not an string", None))
    }
}

fn resolve_script_output<T>(output: Result<T, LuaError>) -> ScriptResult<T> {
    output.map_err(|err| lua_to_script_error("script failed to execute", Some(err)))
}

fn create_runtime<T, S>(ui: &T, config_path: String) -> Lua
        where T: ApplicationUI<S>,
              S: ScriptingEngine {
    let mut lua = Lua::new();
    lua.openlibs();
    lua.set("NOT_FOUND", NOT_FOUND);
    lua.set("log_info", function1(|message: String| {
        info!("{}", message);
    }));
    lua.set("log_debug", function1(|message: String| {
        debug!("{}", message);
    }));
    lua.set("copy", function1(|message: String| {
        info!("copy");
        ui.copy(&message);
    }));
    lua.set("run_command", function2(|window_index: u32, command: String| {
        info!("run_command");
        ui.execute_command(coerce_optional_index(window_index), &command);
    }));
    lua.set("config_file_path", config_path);
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
    lua.set("focus_window", function1(|index: u32| {
        info!("focus_window: {}", index);
        ui.focus_window(index);
    }));
    lua.set("focus_webview_in_window", function1(|index: u32| {
        info!("focus_webview_in_window: {}", index);
        ui.focus_window_area(index, WindowArea::WebView);
    }));
    lua.set("focus_commandbar_in_window", function1(|index: u32| {
        info!("focus_commandbar_in_window: {}", index);
        ui.focus_window_area(index, WindowArea::CommandBar);
    }));
    lua.set("open_window", function1(|uri: String| {
        info!("open_window");
        ui.open_window::<_, Config>(coerce_optional_str(uri), None)
    }));
    lua.set("open_custom_window", function2(|uri: String, config: String| {
        info!("open_window");
        ui.open_window(coerce_optional_str(uri), Config::parse(&config))
    }));
    lua.set("close_window", function1(|window_index: u32| {
        info!("close_window: {}", window_index);
        ui.close_window(window_index);
    }));
    lua.set("window_count", function0(|| {
        info!("get window_count");
        ui.window_count()
    }));
    lua.set("focused_window_index", function0(|| {
        info!("get focused_window_index");
        ui.focused_window_index().unwrap_or(NOT_FOUND)
    }));
    lua.set("window_title", function1(|window_index: u32| {
        info!("window_title: {}", window_index);
        ui.window_title(window_index)
    }));
    lua.set("set_window_title", function2(|window_index: u32, title: String| {
        info!("set_window_title: {}", window_index);
        ui.set_window_title(window_index, &title);
    }));
    lua.set("hide_window", function1(|window_index: u32| {
        info!("hide_window: {}", window_index);
        ui.toggle_window(window_index, false);
    }));
    lua.set("show_window", function1(|window_index: u32| -> () {
        info!("show_window: {}", window_index);
        ui.toggle_window(window_index, true);
    }));
    lua.set("open_webview", function2(|window_index: u32, uri: String| {
        info!("open_webview: {}", window_index);
        ui.open_webview::<_, Config>(window_index, coerce_optional_str(uri), None);
    }));
    lua.set("open_custom_webview", function3(|window_index: u32, uri: String, config: String| {
        info!("open_custom_webview: {} {}", window_index, config);
        ui.open_webview::<_, Config>(window_index, coerce_optional_str(uri), Config::parse(&config));
    }));
    lua.set("webview_count", function1(|window_index: u32| {
        info!("get webview_count: {}", window_index);
        ui.webview_count(window_index)
    }));
    lua.set("set_command_field_visible", function2(|window_index: u32, visible: bool| {
        info!("set command_field_visible");
        ui.set_command_field_visible(window_index, visible);
    }));
    lua.set("set_command_field_text", function2(|window_index: u32, text: String| {
        info!("set command_field_text");
        ui.set_command_field_text(window_index, &text);
    }));
    lua.set("command_field_visible", function1(|window_index: u32| {
        info!("get command_field_visible");
        ui.command_field_visible(window_index)
    }));
    lua.set("command_field_text", function1(|window_index: u32| {
        info!("get command_field_text");
        ui.command_field_text(window_index)
    }));
    lua.set("focused_webview_index", function1(|window_index: u32| {
        info!("get focused_webview_index");
        ui.focused_webview_index(window_index).unwrap_or(NOT_FOUND)
    }));
    lua.set("resize_window", function3(|window_index: u32, width: u32, height: u32| {
        info!("resize_window: {} => ({}, {})", window_index, width, height);
        ui.resize_window(window_index, width, height);
    }));
    lua.set("close_webview", function2(|window_index: u32, webview_index: u32| {
        info!("close_webview: ({}, {})", window_index, webview_index);
        ui.close_webview(window_index, webview_index);
    }));
    lua.set("reload_webview", function3(|window_index: u32, webview_index: u32, disable_filters: bool| {
        info!("reload_webview: ({}, {})", window_index, webview_index);
        ui.reload_webview(window_index, webview_index, disable_filters);
    }));
    lua.set("focus_webview", function2(|window_index: u32, webview_index: u32| {
        info!("focus_webview: ({}, {})", window_index, webview_index);
        ui.focus_webview(window_index, webview_index);
    }));
    lua.set("load_uri", function3(|window_index: u32, webview_index: u32, uri: String| {
        info!("load_uri: ({}, {})", window_index, webview_index);
        ui.set_uri(window_index, webview_index, &uri);
    }));
    lua.set("go_back", function2(|window_index: u32, webview_index: u32| {
        info!("go_back: ({}, {})", window_index, webview_index);
        ui.go_back(window_index, webview_index);
    }));
    lua.set("go_forward", function2(|window_index: u32, webview_index: u32| {
        info!("go_forward: ({}, {})", window_index, webview_index);
        ui.go_forward(window_index, webview_index);
    }));
    lua.set("webview_uri", function2(|window_index: u32, webview_index: u32| {
        info!("get webview_uri: ({}, {})", window_index, webview_index);
        ui.uri(window_index, webview_index)
    }));
    lua.set("webview_title", function2(|window_index: u32, webview_index: u32| {
        info!("get webview_title: ({}, {})", window_index, webview_index);
        ui.webview_title(window_index, webview_index)
    }));
    lua.set("find", function3(|window_index: u32, webview_index: u32, query: String| {
        info!("find: ({}, {})", window_index, webview_index);
        ui.find_string(window_index, webview_index, &query);
    }));
    lua.set("hide_find", function2(|window_index: u32, webview_index: u32| {
        info!("hide_find: ({}, {})", window_index, webview_index);
        ui.hide_find_results(window_index, webview_index)
    }));
    lua.set("run_javascript", function3(|window_index: u32, webview_index: u32, script: String| {
        info!("run_javascript: ({}, {})", window_index, webview_index);
        ui.run_javascript(window_index, webview_index, &script);
    }));
    lua.set("add_styles", function3(|window_index: u32, webview_index: u32, styles: String| {
        info!("add_styles: ({}, {})", window_index, webview_index);
        ui.apply_styles(window_index, webview_index, &styles);
    }));
    lua
}

fn coerce_optional_index(value: u32) -> Option<u32> {
    if value == NOT_FOUND {
        None
    } else {
        Some(value)
    }
}

fn coerce_optional_str<T: Into<String>>(value: T) -> Option<String> {
    let string = value.into();
    if string.is_empty() {
        return None;
    }
    Some(string)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{File,remove_file};
    use std::io::Write;
    use std::path::PathBuf;
    use std::slice;
    use script::{ScriptingEngine,LuaEngine};

    #[test]
    fn describe_missing_method() {
        let path = create_script("mail_missing", r#"
            function not_description()
                return "Sends mail"
            end
        "#);
        let file = File::open(path.clone()).ok().unwrap();
        let result = LuaEngine::describe(file);
        assert!(result.is_err());
        cleanup_script(path);
    }

    #[test]
    fn describe_invalid_string() {
        let path = create_script("mail_invalid", r#"
            function description()
            end
        "#);
        let file = File::open(path.clone()).ok().unwrap();
        let result = LuaEngine::describe(file);
        assert!(result.is_err());
        cleanup_script(path);
    }

    #[test]
    fn describe_valid_command() {
        let path = create_script("mail_valid", r#"
            function description()
                return "Sends mail"
            end
        "#);
        let file = File::open(path.clone()).ok().unwrap();
        let result = LuaEngine::describe(file).ok().unwrap();
        assert_eq!(String::from("Sends mail"), result);
        cleanup_script(path);
    }

    #[allow(unused_must_use)]
    fn cleanup_script(path: PathBuf) {
        remove_file(path);
    }

    #[allow(unused_must_use)]
    fn create_script(name: &str, contents: &str) -> PathBuf {
        let mut dir = env::temp_dir();
        dir.push(&format!("{}.lua", name));

        let mut file = File::create(dir.clone()).ok().unwrap();
        unsafe {
            let slice = slice::from_raw_parts(contents.as_ptr(),
                                              contents.len());
            file.write_all(slice);
        }
        dir
    }
}
