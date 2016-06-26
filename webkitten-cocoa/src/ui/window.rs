use std::str;

use cocoa::base::{id,nil,NO,class};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize, NSFastEnumeration,
                        NSAutoreleasePool, NSString};
use cocoa::appkit::{NSWindow, NSTitledWindowMask, NSResizableWindowMask,
                    NSMiniaturizableWindowMask, NSClosableWindowMask,
                    NSBackingStoreBuffered};
use cocoa_ext::foundation::{NSURLRequest,NSArray};
use cocoa_ext::appkit::{NSLayoutConstraint,NSLayoutAttribute,
                        NSConstraintBasedLayoutInstallingConstraints,
                        NSTextField,NSView,NSControl};
use cocoa_ext::core_graphics::CGRectZero;
use core_graphics::base::CGFloat;

use webkitten::WEBKITTEN_TITLE;
use webkit::*;
use runtime::{AddressBarDelegate,CommandBarDelegate};
use super::webview;

const BAR_HEIGHT: usize = 24;

pub enum CocoaWindowSubview {
    AddressBar       = 0,
    WebViewContainer = 1,
    CommandBar       = 2,
}

pub fn toggle(window: id, visible: bool) {
    unsafe {
        if visible {
            window.makeKeyAndOrderFront_(nil);
        } else {
            window.orderOut_(nil);
        }
    }
}

pub fn open(uri: Option<&str>) {
    unsafe {
        let window = create_nswindow();
        if let Some(uri) = uri {
            webview::load_uri(add_and_focus_webview(window), uri);
        }
    }
}

pub fn close(window_index: u8) {
}

pub fn title(window_index: u8) -> String {
    String::new()
}

pub fn open_webview(window_index: u8, uri: &str) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            webview::load_uri(add_and_focus_webview(window), uri);
        }
    }
}

pub fn close_webview(window_index: u8, index: u8) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let container = subview(window, CocoaWindowSubview::WebViewContainer);
            if container.subviews().count() > (index as NSUInteger) {
                container.subviews().object_at_index(index as NSUInteger).remove_from_superview();
            }
        }
    }
}

pub fn focus_webview(window_index: u8, webview_index: u8) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            println!("Focusing webview {} in window {}", webview_index, window_index);
            let expected_index = webview_index as usize;
            for (index, view) in window_webviews(window).iter().enumerate() {
                view.set_hidden(index == expected_index);
            }
        }
    }
}

pub fn webview(window_index: u8, webview_index: u8) -> Option<id> {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            window_webviews(window).get(webview_index as NSUInteger)
        } else {
            None
        }
    }
}

pub fn resize(window_index: u8, width: u32, height: u32) {
}

pub fn address_field_text(window_index: u8) -> String {
    String::new()
}

pub fn set_address_field_text(window_index: u8, text: &str) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let bar = subview(window, CocoaWindowSubview::AddressBar);
            bar.set_string_value(text);
        }
    }
}

pub fn command_field_text(window_index: u8) -> String {
    String::new()
}

pub fn set_command_field_text(window_index: u8, text: &str) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let bar = subview(window, CocoaWindowSubview::CommandBar);
            bar.set_string_value(text);
        }
    }
}

pub fn focused_webview_index(window_index: u8) -> u8 {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            for (index, view) in window_webviews(window).iter().enumerate() {
                if view.hidden() == NO {
                    return index as u8;
                }
            }

        }
    }
    0
}

pub fn webview_count(window_index: u8) -> u8 {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            window_webviews(window).count() as u8
        } else {
            0
        }
    }
}

unsafe fn window_for_index(index: u8) -> Option<id> {
    let windows: id = msg_send![super::application::nsapp(), windows];
    windows.get(index as NSUInteger)
}

unsafe fn subview(window: id, index: CocoaWindowSubview) -> id {
    let subviews = window.contentView().subviews();
    msg_send![subviews, objectAtIndex:index]
}

unsafe fn add_and_focus_webview(window: id) -> id {
    let container = subview(window, CocoaWindowSubview::WebViewContainer);
    for view in container.subviews().iter() {
        view.set_hidden(true);
    }
    add_webview(container)
}

unsafe fn window_webviews(window: id) -> id {
    subview(window, CocoaWindowSubview::WebViewContainer).subviews()
}

unsafe fn create_nswindow() -> id {
    let mask = (NSTitledWindowMask as NSUInteger |
                NSMiniaturizableWindowMask as NSUInteger |
                NSResizableWindowMask as NSUInteger |
                NSClosableWindowMask as NSUInteger) as NSUInteger;
    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
        NSRect::new(NSPoint::new(0., 0.), NSSize::new(700., 700.)),
        mask,
        NSBackingStoreBuffered,
        NO
    ).autorelease();
    window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
    window.center();
    let title = NSString::alloc(nil).init_str(WEBKITTEN_TITLE);
    window.setTitle_(title);
    layout_window_subviews(window);
    window
}

unsafe fn add_webview(container: id) -> id {
    let webview = WKWebView(CGRectZero(), WKWebViewConfiguration().autorelease()).autorelease();
    webview.disable_translates_autoresizing_mask_into_constraints();
    container.add_subview(webview);
    container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Top, container, NSLayoutAttribute::Top));
    container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Bottom, container, NSLayoutAttribute::Bottom));
    container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Left, container, NSLayoutAttribute::Left));
    container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Right, container, NSLayoutAttribute::Right));
    webview
}

unsafe fn layout_window_subviews(window: id) -> (id, id) {
    let container = <id as NSView>::new();
    let address_bar = <id as NSTextField>::new();
    let command_bar = <id as NSTextField>::new();
    window.contentView().add_subview(address_bar);
    window.contentView().add_subview(container);
    window.contentView().add_subview(command_bar);
    address_bar.disable_translates_autoresizing_mask_into_constraints();
    address_bar.set_height(BAR_HEIGHT as CGFloat);
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(address_bar, NSLayoutAttribute::Top, window.contentView(), NSLayoutAttribute::Top));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(address_bar, NSLayoutAttribute::Left, window.contentView(), NSLayoutAttribute::Left));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(address_bar, NSLayoutAttribute::Right, window.contentView(), NSLayoutAttribute::Right));
    command_bar.disable_translates_autoresizing_mask_into_constraints();
    command_bar.set_height(BAR_HEIGHT as CGFloat);
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(command_bar, NSLayoutAttribute::Bottom, window.contentView(), NSLayoutAttribute::Bottom));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(command_bar, NSLayoutAttribute::Left, window.contentView(), NSLayoutAttribute::Left));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(command_bar, NSLayoutAttribute::Right, window.contentView(), NSLayoutAttribute::Right));
    container.disable_translates_autoresizing_mask_into_constraints();
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(container, NSLayoutAttribute::Top, address_bar, NSLayoutAttribute::Bottom));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(container, NSLayoutAttribute::Bottom, command_bar, NSLayoutAttribute::Top));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(container, NSLayoutAttribute::Left, window.contentView(), NSLayoutAttribute::Left));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(container, NSLayoutAttribute::Right, window.contentView(), NSLayoutAttribute::Right));
    window.makeKeyAndOrderFront_(nil);
    let address_bar_delegate: id = AddressBarDelegate::new();
    let command_bar_delegate: id = CommandBarDelegate::new();
    address_bar.set_delegate(address_bar_delegate);
    command_bar.set_delegate(command_bar_delegate);
    (address_bar_delegate, command_bar_delegate)
}
