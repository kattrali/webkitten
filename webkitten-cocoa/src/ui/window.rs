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
                        NSTextField,NSView};
use cocoa_ext::core_graphics::CGRectZero;
use core_graphics::base::CGFloat;

use webkitten::ui::{BrowserWindow,WebView};
use webkitten::WEBKITTEN_TITLE;
use webkit::*;

const BAR_HEIGHT: usize = 26;
pub const ABDELEGATE_CLASS: &'static str = "AddressBarDelegate";
pub const CBDELEGATE_CLASS: &'static str = "CommandBarDelegate";

pub enum CocoaWindowSubview {
    AddressBar       = 0,
    WebViewContainer = 1,
    CommandBar       = 2,
}

pub struct CocoaWindow {
    nswindow: id
}

impl BrowserWindow for CocoaWindow {

    fn new() -> Self {
        let window = CocoaWindow {
            nswindow: unsafe {
                let window = create_nswindow();
                layout_window_subviews(window);
                window
            }
        };
        window
    }

    fn show(&self) {
        unsafe { self.nswindow.makeKeyAndOrderFront_(nil); }
    }

    fn hide(&self) {
        unsafe { self.nswindow.orderOut_(nil); }
    }

    fn open_webview(&self, uri: &str) {
        let webview = self.add_and_focus_webview();
        unsafe { webview.load_request(NSURLRequest(uri)); }
    }

    fn close_webview(&self, index: u8) {
        unsafe {
            let container = self.subview(CocoaWindowSubview::WebViewContainer);
            if container.subviews().count() > (index as NSUInteger) {
                container.subviews().object_at_index(index as NSUInteger).remove_from_superview();
            }
        }
    }

    fn focus_webview(&self, index: u8) {
    }

    fn webview<W: WebView>(&self, index: u8) -> Option<&W> {
        None
    }

    fn resize(&self, width: u32, height: u32) {
    }

    fn address_field_text(&self) -> String {
        String::new()
    }

    fn set_address_field_text(&self, text: String) {
    }

    fn command_field_text(&self) -> String {
        String::new()
    }

    fn set_command_field_text(&self, text: String) {
    }

    fn focused_webview_index(&self) -> u8 {
        0
    }
}

impl CocoaWindow {

    unsafe fn subview(&self, index: CocoaWindowSubview) -> id {
        let subviews = self.nswindow.contentView().subviews();
        msg_send![subviews, objectAtIndex:index]
    }

    fn add_and_focus_webview(&self) -> id {
        unsafe {
            let container = self.subview(CocoaWindowSubview::WebViewContainer);
            for view in container.subviews().iter() {
                view.set_hidden(true);
            }
            add_webview(container)
        }
    }
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
    let address_bar_delegate: id = msg_send![class(ABDELEGATE_CLASS), new];
    let command_bar_delegate: id = msg_send![class(CBDELEGATE_CLASS), new];
    address_bar.set_delegate(address_bar_delegate);
    command_bar.set_delegate(command_bar_delegate);
    (address_bar_delegate, command_bar_delegate)
}
