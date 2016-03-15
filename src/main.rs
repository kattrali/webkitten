extern crate libc;
extern crate gtk_sys;
extern crate gio_sys;
extern crate gobject_sys;
extern crate glib_sys;

use libc::{c_void};
use std::ffi::CString;
use std::mem;

unsafe extern "C" fn activate(app: *mut gtk_sys::GtkApplication, _:*mut c_void) {
    let widget = gtk_sys::gtk_application_window_new(app);
    let title = CString::new("webkitten").unwrap().as_ptr();
    let window: *mut gtk_sys::GtkWindow = mem::transmute(widget);
    gtk_sys::gtk_window_set_title(window, title);
    gtk_sys::gtk_window_set_default_size(window, 200, 400);
    gtk_sys::gtk_widget_show_all(widget);
}

fn main() {
    unsafe {
        let identifier = CString::new("me.delisa.webkitten").unwrap();
        let flags = gio_sys::GApplicationFlags::empty();
        let app = gtk_sys::gtk_application_new(identifier.as_ptr(), flags);
        let gobject_app: *mut gobject_sys::GObject = mem::transmute(app);

        let activate_signal = CString::new("activate").unwrap().as_ptr();
        let mut data = 0;
        let callback: gobject_sys::GCallback = mem::transmute(Some(activate));
        gobject_sys::g_signal_connect_data(gobject_app,
                              activate_signal,
                              callback,
                              &mut data as *mut _ as glib_sys::gpointer,
                              None,
                              gobject_sys::GConnectFlags::empty());
        let argv = 0;
        let gapp: *mut gio_sys::GApplication = mem::transmute(app);
        gio_sys::g_application_run(gapp, 0, argv as *mut *mut i8);
        gobject_sys::g_object_unref(gobject_app);
    }
    println!("Execution complete :)");
}
