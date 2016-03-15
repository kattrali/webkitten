extern crate libc;
extern crate gtk_sys;
extern crate gobject_sys;
extern crate glib_sys;

use libc::{c_char,c_int};
use std::ffi::CString;
use std::mem;
use gtk_sys::{GtkWidget,GtkWindow};

pub const WEBKITTEN_APP_ID: &'static str = "me.delisa.webkitten";
pub const WEBKITTEN_TITLE: &'static str = "webkitten";

type GtkWidgetCallback = unsafe extern "C" fn(*mut gtk_sys::GtkWidget,
                                              *mut gtk_sys::GtkWidget);

enum WebKitWebView {}

#[link(name="webkit2gtk-4.0")]
extern "C" {
    fn webkit_web_view_new() -> *mut WebKitWebView;
    fn webkit_web_view_load_uri(webview: *mut WebKitWebView, uri: *const c_char);
}

pub struct Application {
    windows: Vec<Window>,
}

pub struct Window {
    gtk_window: *mut GtkWidget,
    webviews: Vec<*mut WebKitWebView>,
}

impl Application {

    pub fn new() -> Self {
        Application { windows: vec![] }
    }

    pub fn run(&mut self) {
        with_gtk_app(|| if self.windows.is_empty() { self.add_window(); })
    }

    pub fn add_window(&mut self) {
        let mut window = Window::new(WEBKITTEN_TITLE);
        window.add_webview("http://delisa.me");
        window.show();
        self.windows.push(window);
    }
}

impl Window {

    pub fn new(title: &'static str) -> Self {
        let widget = unsafe {
            let widget = gtk_sys::gtk_window_new(gtk_sys::GTK_WINDOW_TOPLEVEL);
            add_widget_callback(widget, "destroy", destroy_window_callback);
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

fn with_gtk_app<F>(callback: F) where F: FnOnce() -> () {
    unsafe { gtk_sys::gtk_init(0 as *mut c_int, 0 as *mut *mut *mut c_char); }
    callback();
    unsafe { gtk_sys::gtk_main(); }
}

// Destroy the application when a window is closed - temporary behavior while
// there is only one window per application.
unsafe extern "C" fn destroy_window_callback(_: *mut GtkWidget, _: *mut GtkWidget) {
    gtk_sys::gtk_main_quit();
}

// Add a callback to an event on a GTK+ widget
unsafe fn add_widget_callback(widget: *mut GtkWidget,
                              signal: &'static str,
                            callback: GtkWidgetCallback) {
    let gobject_widget: *mut gobject_sys::GObject = mem::transmute(widget);
    let signal_ptr = CString::new(signal).unwrap().as_ptr();
    let mut data = 0;
    let gcallback: gobject_sys::GCallback = mem::transmute(Some(callback));
    gobject_sys::g_signal_connect_data(gobject_widget, signal_ptr, gcallback,
                                      &mut data as *mut _ as glib_sys::gpointer,
                                      None, gobject_sys::GConnectFlags::empty());
}
