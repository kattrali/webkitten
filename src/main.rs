extern crate libc;
extern crate gtk_sys;
extern crate gio_sys;
extern crate gobject_sys;
extern crate glib_sys;

use libc::{c_char,c_void};
use std::ffi::CString;
use std::mem;

enum WebKitWebView {}

type GtkApplicationCallback = unsafe extern "C" fn(*mut gtk_sys::GtkApplication, *mut c_void);

const WEBKITTEN: &'static str = "webkitten";
const WEBKITTEN_APP_ID: &'static str = "me.delisa.webkitten";

#[link(name="webkit2gtk-4.0")]
extern "C" {
    fn webkit_web_view_new() -> *mut WebKitWebView;
    fn webkit_web_view_load_uri(webview: *mut WebKitWebView, uri: *const c_char);
}

unsafe extern "C" fn activate(app: *mut gtk_sys::GtkApplication, _:*mut c_void) {
    let widget = gtk_sys::gtk_application_window_new(app);
    let title = CString::new(WEBKITTEN).unwrap().as_ptr();
    let window: *mut gtk_sys::GtkWindow = mem::transmute(widget);
    gtk_sys::gtk_window_set_title(window, title);
    gtk_sys::gtk_window_set_default_size(window, 800, 600);

    let webview = webkit_web_view_new();
    let webview_widget: *mut gtk_sys::GtkWidget = mem::transmute(webview);
    let container: *mut gtk_sys::GtkContainer = mem::transmute(widget);
    gtk_sys::gtk_container_add(container, webview_widget);

    let uri = CString::new("http://www.webkitgtk.org").unwrap().as_ptr();
    webkit_web_view_load_uri(webview, uri);

    gtk_sys::gtk_widget_grab_focus(webview_widget);
    gtk_sys::gtk_widget_show_all(widget);
}

unsafe fn add_app_callback(app: *mut gtk_sys::GtkApplication,
                    signal: &'static str,
                    callback: Option<GtkApplicationCallback>) {
    let gobject_app: *mut gobject_sys::GObject = mem::transmute(app);
    let signal_ptr = CString::new(signal).unwrap().as_ptr();
    let mut data = 0;
    let gcallback: gobject_sys::GCallback = mem::transmute(callback);
    gobject_sys::g_signal_connect_data(gobject_app,
                                      signal_ptr,
                                      gcallback,
                                      &mut data as *mut _ as glib_sys::gpointer,
                                      None,
                                      gobject_sys::GConnectFlags::empty());
}

fn main() {
    unsafe {
        let identifier = CString::new(WEBKITTEN_APP_ID).unwrap();
        let flags = gio_sys::GApplicationFlags::empty();
        let app = gtk_sys::gtk_application_new(identifier.as_ptr(), flags);
        add_app_callback(app, "activate", Some(activate));

        let gapp: *mut gio_sys::GApplication = mem::transmute(app);
        let argv = 0;
        gio_sys::g_application_run(gapp, 0, argv as *mut *mut i8);

        let gobject_app: *mut gobject_sys::GObject = mem::transmute(app);
        gobject_sys::g_object_unref(gobject_app);
    }
    println!("Execution complete :)");
}
