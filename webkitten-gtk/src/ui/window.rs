use std::mem::transmute;
use std::ffi::CString;

use super::gtk_sys;
use super::gtk_sys::{GtkWindow,GtkContainer,GtkWidget};
use super::{WebView,
            BoxContainer,
            TextField,
            FlowDirection,
            GtkContainerConvertible,
            GtkWidgetConvertible};

pub struct Window {
    address_field: TextField,
    command_field: TextField,
    gtk_widget: *mut GtkWidget,
    webview_container: BoxContainer,
    webviews: Vec<WebView>,
    widget_container: BoxContainer,
    did_layout: bool,
}

impl Window {

    pub fn new(title: &'static str) -> Self {
        Window {
            address_field: TextField::new(),
            command_field: TextField::new(),
            gtk_widget: create_gtk_widget(title),
            webview_container: BoxContainer::new(FlowDirection::Vertical),
            widget_container: BoxContainer::new(FlowDirection::Vertical),
            webviews: vec![],
            did_layout: false
        }
    }

    pub fn add_webview(&mut self, uri: &'static str) {
        if !self.did_layout {
            self.layout();
        }
        let webview = WebView::new();
        self.webview_container.add_child(webview.gtk_widget(), true);
        webview.load_uri(uri);
        webview.focus();

        self.webviews.push(webview);
    }

    pub fn add_child(&mut self, child: *mut GtkWidget) {
        unsafe { gtk_sys::gtk_container_add(self.gtk_container(), child) }
    }

    pub fn show(&mut self) {
        unsafe { gtk_sys::gtk_widget_show_all(self.gtk_widget); }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        unsafe { gtk_sys::gtk_window_set_default_size(self.gtk_window(),
                                                      width, height) }
    }

    fn gtk_window(&self) -> *mut GtkWindow {
        unsafe { transmute(self.gtk_widget) }
    }

    fn layout(&mut self) {
        self.widget_container.add_child(self.command_field.gtk_widget(), false);
        self.widget_container.add_child(self.webview_container.gtk_widget(), true);
        self.widget_container.add_child(self.address_field.gtk_widget(), false);
        let widget_container = self.widget_container.gtk_widget();
        self.add_child(widget_container);
        self.did_layout = true;
    }
}

impl GtkContainerConvertible for Window {

    fn gtk_container(&self) -> *mut GtkContainer {
        unsafe { transmute(self.gtk_widget) }
    }
}

// Destroy the application when a window is closed - temporary behavior while
// there is only one window per application.
unsafe extern "C" fn destroy_window_callback(_: *mut GtkWidget, _: *mut GtkWidget) {
    gtk_sys::gtk_main_quit();
}

fn create_gtk_widget(title: &'static str) -> *mut GtkWidget {
    return unsafe {
        let widget = gtk_sys::gtk_window_new(gtk_sys::GTK_WINDOW_TOPLEVEL);
        super::add_widget_callback(widget, "destroy", destroy_window_callback);
        let window: *mut GtkWindow = transmute(widget);
        gtk_sys::gtk_window_set_title(window,  CString::new(title).unwrap().as_ptr());
        gtk_sys::gtk_window_set_default_size(window, 800, 600);
        widget
    };
}
