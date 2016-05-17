use super::gtk_sys;
use super::gobject_sys::{GObject,g_object_ref,g_object_unref};
use super::gtk_sys::GtkWidget;
use super::super::webkitgtk_sys::*;
use super::{GtkWidgetConvertible,GObjectConvertible};
use std::ffi::CString;
use std::mem::transmute;
use std::ops::Drop;

pub struct WebView {
    gtk_widget: *mut GtkWidget,
}

impl WebView {

    pub fn new() -> Self {
        WebView { gtk_widget: create_gtk_widget() }
    }

    pub fn load_uri(&self, uri: &'static str) {
        let raw_uri = CString::new(uri).unwrap().as_ptr();
        unsafe { webkit_web_view_load_uri(self.webkit_webview(), raw_uri); }
    }

    pub fn go_back(&self) {
        unsafe { webkit_web_view_go_back(self.webkit_webview()) }
    }

    pub fn go_forward(&self) {
        unsafe { webkit_web_view_go_forward(self.webkit_webview()) }
    }

    pub fn focus(&self) {
        unsafe { gtk_sys::gtk_widget_show_all(self.gtk_widget) }
    }

    fn webkit_webview(&self) -> *mut WebKitWebView {
        unsafe { transmute(self.gtk_widget) }
    }
}

impl GObjectConvertible for WebView {

    fn gobject(&self) -> *mut GObject {
        unsafe { transmute(self.gtk_widget) }
    }
}

impl GtkWidgetConvertible for WebView {

    fn gtk_widget(&self) -> *mut GtkWidget {
        self.gtk_widget
    }
}

impl Drop for WebView {

    fn drop(&mut self) {
        unsafe { g_object_unref(self.gobject()); }
    }
}

fn create_gtk_widget() -> *mut GtkWidget {
    return unsafe {
        let widget = webkit_web_view_new();
        let gobject: *mut GObject = transmute(widget);
        g_object_ref(gobject);
        widget
    };
}
