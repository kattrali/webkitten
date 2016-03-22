extern crate gtk_sys;
extern crate gobject_sys;
extern crate libc;

use gtk_sys::{GtkWidget,GtkEditable,GtkContainer};
use gobject_sys::GObject;

mod text_field;
mod box_container;
pub use self::text_field::TextField;
pub use self::box_container::BoxContainer;

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

pub struct WebView {
    gtk_widget: *mut GtkWidget,
}

fn gboolean(value: bool) -> libc::c_int {
    return match value {
        true  => 1,
        false => 0
    }
}
