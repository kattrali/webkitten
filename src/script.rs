extern crate hlua;

use self::hlua::{Lua,LuaError,function0,function1,function2,function3,function4};
use self::hlua::functions_read::LuaFunction;
use std::fs::File;
use super::ui::ApplicationUI;

const INVALID_RESULT: u8 = 247;

pub fn execute<T: ApplicationUI>(file: File, arguments: Vec<String>, ui: &T) -> bool {
    let mut lua = create_runtime::<T>(ui);
    lua.openlibs();
    lua.set("arguments", arguments);
    lua.execute_from_reader::<(), _>(file);
    let run: Option<LuaFunction<_>> = lua.get("run");
    if run.is_some() {
        let output: Result<bool, LuaError> = run.unwrap().call();
        if let Err(err) = output {
            match err {
                LuaError::SyntaxError(err) => warn!("syntax error: {}", err),
                LuaError::ExecutionError(err) => warn!("execution error: {}", err),
                LuaError::ReadError(err) => warn!("read error: {}", err),
                _ => {}
            }
            return false
        }
        return true
    }
    return false
}

fn create_runtime<T: ApplicationUI>(ui: &T) -> Lua {
    let mut lua = Lua::new();
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
    lua.set("register_content_filters", function2(|identifier: String, rules: String| {
        ui.register_content_filters(&identifier, &rules);
    }));
    lua
}
