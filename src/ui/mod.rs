extern crate gtk_sys;
extern crate gobject_sys;
extern crate glib_sys;
extern crate libc;

use gobject_sys::GObject;
use gtk_sys::{GtkWidget,GtkEditable,GtkContainer};
use std::ffi::CString;
use std::mem;

mod text_field;
mod box_container;
mod window;
mod web_view;

pub use self::text_field::TextField;
pub use self::box_container::BoxContainer;
pub use self::window::Window;
pub use self::web_view::WebView;

pub type GtkWidgetCallback = unsafe extern "C" fn(*mut gtk_sys::GtkWidget,
                                                  *mut gtk_sys::GtkWidget);

/// Structures with a GObject representation
trait GObjectConvertible {
    fn gobject(&self) -> *mut GObject;
}

/// Structures with a GTK+ widget representation
trait GtkContainerConvertible {
    fn gtk_container(&self) -> *mut GtkContainer;
}

/// Structures with a GTK+ widget representation
trait GtkWidgetConvertible {
    fn gtk_widget(&self) -> *mut GtkWidget;
}

/// Structures with a GTK+ editable representation
trait GtkEditableConvertible {
    fn gtk_editable(&self) -> *mut GtkEditable;
}

pub enum FlowDirection {
    Vertical,
    Horizontal
}

// Add a callback to an event on a GTK+ widget
pub fn add_widget_callback(widget: *mut GtkWidget,
                           signal: &'static str,
                         callback: GtkWidgetCallback) {
    unsafe {
        let gobject_widget: *mut gobject_sys::GObject = mem::transmute(widget);
        let signal_ptr = CString::new(signal).unwrap().as_ptr();
        let mut data = 0;
        let gcallback: gobject_sys::GCallback = mem::transmute(Some(callback));
        gobject_sys::g_signal_connect_data(gobject_widget, signal_ptr, gcallback,
                                          &mut data as *mut _ as glib_sys::gpointer,
                                          None, gobject_sys::GConnectFlags::empty());
    }
}

fn gboolean(value: bool) -> libc::c_int {
    return match value {
        true  => 1,
        false => 0
    }
}
