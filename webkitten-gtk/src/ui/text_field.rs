use std::mem::transmute;
use std::ffi::CString;
use super::gtk_sys::{GtkEntry,GtkEditable,GtkWidget};
use super::gtk_sys::{
    gtk_entry_get_text,
    gtk_entry_set_text,
    gtk_entry_new,
    gtk_entry_set_has_frame};

pub struct TextField {
    gtk_widget: *mut GtkWidget,
}

impl TextField {

    pub fn new() -> Self {
        TextField { gtk_widget: create_gtk_widget() }
    }

    /// The displayed text
    pub fn text(&self) -> String {
        return unsafe {
            let mut raw_text = *gtk_entry_get_text(self.gtk_entry());
            let c_text = CString::from_raw(&mut raw_text);
            return match c_text.into_string() {
                Ok(text) => text,
                _ => panic!("Unable to parse text")
            }
        }
    }

    /// Changes the displayed text
    pub fn set_text(&mut self, text: &'static str) {
        match CString::new(text) {
            Ok(entry_text) => { unsafe {
                gtk_entry_set_text(self.gtk_entry(), entry_text.into_raw())
            }},
            _ => {}
        }
    }

    fn gtk_entry(&self) -> *mut GtkEntry {
        return unsafe { transmute(self.gtk_widget) };
    }
}

impl super::GtkWidgetConvertible for TextField {

    fn gtk_widget(&self) -> *mut GtkWidget {
        self.gtk_widget
    }
}

impl super::GtkEditableConvertible for TextField {

    fn gtk_editable(&self) -> *mut GtkEditable {
        return unsafe { transmute(self.gtk_widget) }
    }
}

fn create_gtk_widget() -> *mut GtkWidget {
    return unsafe {
        let widget = gtk_entry_new();
        let entry: *mut GtkEntry = transmute(widget);
        gtk_entry_set_has_frame(entry, super::gboolean(false));
        widget
    }
}
