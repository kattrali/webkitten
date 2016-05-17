use std::mem::transmute;
use super::{FlowDirection,GtkWidgetConvertible,GtkContainerConvertible,gboolean};
use super::gtk_sys::{GtkBox,GtkWidget,GtkContainer,GtkOrientation};
use super::gtk_sys::{
    gtk_box_new,
    gtk_box_pack_end,
    gtk_container_remove};

pub struct BoxContainer {
    gtk_widget: *mut GtkWidget,
}

impl BoxContainer {

    pub fn new(direction: FlowDirection) -> Self {
        let orientation = gtk_orientation(direction);
        BoxContainer { gtk_widget: create_gtk_widget(orientation) }
    }

    /// Adds a child widget to the container, bottom to top
    ///
    /// ## Panics
    ///
    /// Panics if the widget is already a child of the container
    pub fn add_child(&mut self, child: *mut GtkWidget, fill: bool) {
        let gfill = gboolean(fill);
        unsafe { gtk_box_pack_end(self.gtk_box(), child, gfill, gfill, 0); }
    }

    /// Removes a child widget from the container
    pub fn remove_child(&mut self, child: *mut GtkWidget) {
        unsafe { gtk_container_remove(self.gtk_container(), child); }
    }

    fn gtk_box(&self) -> *mut GtkBox {
        return unsafe { transmute(self.gtk_widget) };
    }
}

impl GtkContainerConvertible for BoxContainer {

    fn gtk_container(&self) -> *mut GtkContainer {
        return unsafe { transmute(self.gtk_widget) }
    }
}

impl GtkWidgetConvertible for BoxContainer {

    fn gtk_widget(&self) -> *mut GtkWidget {
        self.gtk_widget
    }
}

fn create_gtk_widget(direction: GtkOrientation) -> *mut GtkWidget {
    return unsafe { gtk_box_new(direction, 0) };
}

fn gtk_orientation(direction: FlowDirection) -> GtkOrientation {
    return match direction {
        FlowDirection::Vertical   => GtkOrientation::Vertical,
        FlowDirection::Horizontal => GtkOrientation::Horizontal,
    }
}
