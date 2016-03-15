extern crate libc;
extern crate gtk_sys;

use libc::c_char;
use std::ffi::CString;
use std::mem;
use gtk_sys::{GtkApplication,GtkWidget,GtkWindow};

enum WebKitWebView {}

pub const WEBKITTEN_APP_ID: &'static str = "me.delisa.webkitten";
pub const WEBKITTEN_TITLE: &'static str = "webkitten";

#[link(name="webkit2gtk-4.0")]
extern "C" {
    fn webkit_web_view_new() -> *mut WebKitWebView;
    fn webkit_web_view_load_uri(webview: *mut WebKitWebView, uri: *const c_char);
}

pub struct Application {
    app: *mut GtkApplication,
    windows: Vec<Window>,
}

pub struct Window {
    gtk_window: *mut GtkWidget,
    webviews: Vec<*mut WebKitWebView>,
}

impl Application {

    pub fn new(app: *mut GtkApplication) -> Self {
        Application { app: app, windows: vec![] }
    }

    pub fn show(&mut self) {
        if self.windows.is_empty() {
            self.add_window()
        }
    }

    pub fn add_window(&mut self) {
        let mut window = Window::new(self.app, WEBKITTEN_TITLE);
        window.add_webview("http://delisa.me");
        window.show();
        self.windows.push(window);
    }
}

impl Window {

    pub fn new(app: *mut GtkApplication, title: &'static str) -> Self {
        let widget = unsafe {
            let widget = gtk_sys::gtk_application_window_new(app);
            let window: *mut GtkWindow = mem::transmute(widget);
            gtk_sys::gtk_window_set_title(window,  CString::new(title).unwrap().as_ptr());
            gtk_sys::gtk_window_set_default_size(window, 800, 600);
            widget
        };
        return Window { gtk_window: widget, webviews: vec![] };
    }

    pub fn add_webview(&mut self, uri: &'static str) {
        let webview = unsafe {
            let webview = webkit_web_view_new();
            let webview_widget: *mut GtkWidget = mem::transmute(webview);
            let container: *mut gtk_sys::GtkContainer = mem::transmute(self.gtk_window);
            gtk_sys::gtk_container_add(container, webview_widget);

            let uri = CString::new(uri).unwrap().as_ptr();
            webkit_web_view_load_uri(webview, uri);

            gtk_sys::gtk_widget_grab_focus(webview_widget);
            webview
        };
        self.webviews.push(webview);
    }

    pub fn show(&mut self) {
        unsafe { gtk_sys::gtk_widget_show_all(self.gtk_window); }
    }
}

