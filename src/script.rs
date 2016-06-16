extern crate hlua;

use self::hlua::{Lua,LuaError,function0,function1,function2,function3};
use self::hlua::functions_read::LuaFunction;
use std::fs::File;
use super::Engine;
use super::ui::{ApplicationUI,BrowserWindow,WebView};

const INVALID_RESULT: u8 = 247;

pub fn execute<T, B, V>(file: File, arguments: Vec<String>, ui: &T)
    where T: ApplicationUI,
          B: BrowserWindow,
          V: WebView {
    let mut lua = create_runtime::<T, B, V>(ui);
    lua.set("arguments", arguments);
    lua.execute_from_reader::<(), _>(file);
    let run: Option<LuaFunction<_>> = lua.get("run");
    if run.is_some() {
        let output: Result<bool, _> = run.unwrap().call();
    }
}

fn create_runtime<T, B, V>(ui: &T) -> Lua
    where T: ApplicationUI,
          B: BrowserWindow,
          V: WebView {
    let mut lua = Lua::new();
    lua.set("INVALID_RESULT", INVALID_RESULT);
    lua.set("focus_window", function1(|index: u8| {
        ui.focus_window(index);
    }));
    lua.set("open_window", function1(|uri: String| {
        ui.open_window(&uri);
    }));
    lua.set("window_count", function0(|| {
        ui.window_count()
    }));
    lua.set("focused_window_index", function0(|| {
        ui.focused_window_index()
    }));
    lua.set("hide_window", function1(|window_index: u8| {
        with_window::<T, B, _>(ui, window_index, |ref mut w| w.hide());
    }));
    lua.set("show_window", function1(|window_index: u8| -> () {
        with_window::<T, B, _>(ui, window_index, |w| w.show());
    }));
    lua.set("open_webview", function2(|window_index: u8, uri: String| {
        with_window::<T, B, _>(ui, window_index, |w| w.open_webview(uri));
    }));
    lua.set("set_address_field_text", function2(|window_index: u8, uri: String| {
        with_window::<T, B, _>(ui, window_index, |w| w.set_address_field_text(uri));
    }));
    lua.set("set_command_field_text", function2(|window_index: u8, uri: String| {
        with_window::<T, B, _>(ui, window_index, |w| w.set_command_field_text(uri));
    }));
    lua.set("address_field_text", function1(|window_index: u8| {
        match ui.window::<B>(window_index) {
            Some(window) => window.address_field_text(),
            None => String::new()
        }
    }));
    lua.set("command_field_text", function1(|window_index: u8| {
        match ui.window::<B>(window_index) {
            Some(window) => window.command_field_text(),
            None => String::new()
        }
    }));
    lua.set("focused_webview_index", function1(|window_index: u8| {
        match ui.window::<B>(window_index) {
            Some(window) => window.focused_webview_index(),
            None => INVALID_RESULT
        }
    }));
    lua.set("resize_window", function3(|window_index: u8, width: u32, height: u32| {
        with_window::<T, B, _>(ui, window_index, |w| w.resize(width, height));
    }));
    lua.set("close_webview", function2(|window_index: u8, webview_index: u8| {
        with_window::<T, B, _>(ui, window_index, |w| w.close_webview(webview_index));
    }));
    lua.set("focus_webview", function2(|window_index: u8, webview_index: u8| {
        with_window::<T, B, _>(ui, window_index, |w| w.focus_webview(webview_index));
    }));
    lua.set("load_uri", function3(|window_index: u8, webview_index: u8, uri: String| {
        with_window::<T, B, _>(ui, window_index, |w| {
            with_webview::<B, V, _>(w, webview_index, move |v| v.load_uri(uri.as_str()));
        });
    }));
    lua.set("go_back", function2(|window_index: u8, webview_index: u8| {
        with_window::<T, B, _>(ui, window_index, |w| {
            with_webview::<B, V, _>(w, webview_index, |v| v.go_back());
        });
    }));
    lua.set("go_forward", function2(|window_index: u8, webview_index: u8| {
        with_window::<T, B, _>(ui, window_index, |w| {
            with_webview::<B, V, _>(w, webview_index, |v| v.go_forward());
        });
    }));
    lua.set("focus", function2(|window_index: u8, webview_index: u8| {
        with_window::<T, B, _>(ui, window_index, |w| {
            with_webview::<B, V, _>(w, webview_index, |v| v.focus());
        });
    }));
    lua.set("run_javascript", function3(|window_index: u8, webview_index: u8, script: String| {
        with_window::<T, B, _>(ui, window_index, |w| {
            with_webview::<B, V, _>(w, webview_index, |v| v.apply_javascript(script.as_str()));
        });
    }));
    lua.set("add_styles", function3(|window_index: u8, webview_index: u8, styles: String| {
        with_window::<T, B, _>(ui, window_index, |w| {
            with_webview::<B, V, _>(w, webview_index, |v| v.apply_styles(styles.as_str()));
        });
    }));
    lua.set("add_content_blockers", function3(|window_index: u8, webview_index: u8, blockers: String| {
        with_window::<T, B, _>(ui, window_index, |w| {
            with_webview::<B, V, _>(w, webview_index, |v| v.apply_content_blockers(blockers.as_str()));
        });
    }));
    lua
}

fn with_window<T, W, F>(ui: &T, window_index: u8, callback: F)
    where T: ApplicationUI,
          W: BrowserWindow,
          F: FnOnce(&W) -> () {
    if let Some(window) = ui.window(window_index) {
        callback(window);
    }
}

fn with_webview<W, V, F>(window: &W, webview_index: u8, callback: F)
    where W: BrowserWindow,
          V: WebView,
          F: FnOnce(&V) -> () {
    if let Some(webview) = window.webview(webview_index) {
        callback(webview);
    }
}
