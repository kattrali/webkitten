extern crate libc;
extern crate gtk_sys;
extern crate gio_sys;
extern crate gobject_sys;
extern crate glib_sys;
extern crate webkitten;

use webkitten::Application;
use libc::c_void;
use std::ffi::CString;
use std::mem;
use gtk_sys::GtkApplication;

type GtkApplicationCallback = unsafe extern "C" fn(*mut gtk_sys::GtkApplication,
                                                   *mut c_void);

// Initialize a GTK+ application
fn main() {
    unsafe {
        let identifier = CString::new(webkitten::WEBKITTEN_APP_ID).unwrap();
        let flags = gio_sys::GApplicationFlags::empty();
        let app = gtk_sys::gtk_application_new(identifier.as_ptr(), flags);
        add_app_callback(app, "activate", Some(activate));

        let gapp: *mut gio_sys::GApplication = mem::transmute(app);
        gio_sys::g_application_run(gapp, 0, 0 as *mut *mut i8);

        let gobject_app: *mut gobject_sys::GObject = mem::transmute(app);
        gobject_sys::g_object_unref(gobject_app);
    }
}

// Activation callback to the completion of GTK+ application initialization
// Creates a new webkitten application instance and shows the first window
unsafe extern "C" fn activate(app: *mut GtkApplication, _:*mut c_void) {
    let mut webkitten = Application::new(app);
    webkitten.show();
}

// Add a callback to an event on a GTK+ application
unsafe fn add_app_callback(app: *mut GtkApplication, signal: &'static str,
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

