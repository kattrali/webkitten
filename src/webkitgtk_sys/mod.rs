extern crate libc;
extern crate gtk_sys;

use libc::c_char;
use gtk_sys::GtkWidget;

pub enum WebKitWebView {}

#[link(name="webkit2gtk-4.0")]
extern "C" {
    pub fn webkit_web_view_new() -> *mut GtkWidget;
    pub fn webkit_web_view_load_uri(webview: *mut WebKitWebView, uri: *const c_char);
    pub fn webkit_web_view_go_back(webview: *mut WebKitWebView);
    pub fn webkit_web_view_go_forward(webview: *mut WebKitWebView);
}

