use cocoa::base::{id,nil,NO,YES,BOOL};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSFastEnumeration,
                        NSAutoreleasePool};
use cocoa::appkit::{NSWindow, NSTitledWindowMask, NSResizableWindowMask,
                    NSMiniaturizableWindowMask, NSClosableWindowMask,
                    NSFullSizeContentViewWindowMask, NSBackingStoreBuffered};
use cocoa_ext::foundation::{NSArray,NSURLRequest,NSString,NSUInteger};
use cocoa_ext::appkit::{NSLayoutConstraint,NSLayoutAttribute,
                        NSConstraintBasedLayoutInstallingConstraints,
                        NSTextField,NSView,NSControl};
use cocoa_ext::core_graphics::CGRectZero;
use core_graphics::base::CGFloat;
use block::ConcreteBlock;

use webkitten::WEBKITTEN_TITLE;
use webkitten::ui::{BrowserConfiguration, WindowArea};
use webkit::*;
use runtime::{CommandBarDelegate,WebViewHistoryDelegate,log_error_description};
use super::webview;

const BAR_HEIGHT: usize = 24;

pub fn toggle(window_index: u8, visible: bool) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            match visible {
                true => window.makeKeyAndOrderFront_(nil),
                false => window.orderOut_(nil)
            }
        }
    }
}

pub fn open(uri: Option<&str>) {
    unsafe {
        create_nswindow();
        if let Some(uri) = uri {
            add_and_focus_webview(window_count() - 1, uri.to_owned());
        }
    }
}

pub fn focus(window_index: u8) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            window.makeKeyAndOrderFront_(nil);
        }
    }
}

pub fn focus_area(window_index: u8, area: WindowArea) {
    match area {
        WindowArea::WebView => unsafe {
            if let Some(webview) = webview(window_index, focused_webview_index(window_index)) {
               msg_send![webview, becomeFirstResponder];
            }
        },
        WindowArea::CommandBar => unsafe {
            if let Some(window) = window_for_index(window_index) {
                let bar: id = subview(window, area);
                msg_send![bar, becomeFirstResponder];
            }
        }
    }
}

pub fn focused_index() -> u8 {
    unsafe {
        let windows: id = msg_send![super::application::nsapp(), windows];
        for (index, window) in windows.iter().enumerate() {
            let key: BOOL = msg_send![window, isKeyWindow];
            if key == YES {
                return index as u8;
            }
        }
        0
    }
}

pub fn close(window_index: u8) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            window.close();
        }
    }
}

pub fn title(window_index: u8) -> String {
    unsafe {
        window_for_index(window_index)
            .and_then(|win| {
                let title: id = msg_send![win, title];
                title.as_str()
            })
            .and_then(|title| Some(String::from(title)))
            .unwrap_or(String::new())
    }
}

pub fn set_title(window_index: u8, title: &str) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let title_str = <id as NSString>::from_str(title);
            window.setTitle_(title_str);
        }
    }
}

pub fn window_count() -> u8 {
    unsafe {
        let windows: id = msg_send![super::application::nsapp(), windows];
        windows.count() as u8
    }
}

pub fn open_webview(window_index: u8, uri: &str) {
    unsafe {
        add_and_focus_webview(window_index, uri.to_owned());
        if let Some(webview) = webview(window_index, focused_webview_index(window_index)) {
            webview::load_uri(webview, uri);
        }
    }
}

pub fn close_webview(window_index: u8, index: u8) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let container = subview(window, WindowArea::WebView);
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
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let frame = NSRect {
                origin: window.frame().origin,
                size: NSSize { width: width as CGFloat, height: height as CGFloat }
            };
            window.setFrame_display_(frame, YES);
        }
    }
}

pub fn command_field_text(window_index: u8) -> String {
    field_text(window_index, WindowArea::CommandBar)
}

fn field_text(window_index: u8, view: WindowArea) -> String {
    unsafe {
        window_for_index(window_index)
            .and_then(|window| {
                let field = subview(window, view);
                let text: id = field.string_value();
                text.as_str() })
            .and_then(|text| Some(String::from(text)))
            .unwrap_or(String::new())
    }
}

pub fn set_command_field_text(window_index: u8, text: &str) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let bar = subview(window, WindowArea::CommandBar);
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

pub fn reference_indices(webview: id) -> Option<(u8, u8)> {
    unsafe {
        let window: id = msg_send![webview, window];
        if window != nil {
            let windows = super::application::windows();
            let window_index = windows.index_of_object(window) as u8;
            let webviews = window_webviews(window);
            let webview_index = webviews.index_of_object(webview) as u8;
            return Some((window_index, webview_index));
        }
    }
    None
}

unsafe fn window_for_index(index: u8) -> Option<id> {
    super::application::windows().get(index as NSUInteger)
}

unsafe fn subview(window: id, area: WindowArea) -> id {
    let index = match area {
        WindowArea::WebView => 0,
        WindowArea::CommandBar => 1
    };
    let subviews = window.contentView().subviews();
    msg_send![subviews, objectAtIndex:index]
}

unsafe fn add_and_focus_webview(window_index: u8, uri: String) {
    let store = _WKUserContentExtensionStore::default_store(nil);
    let ref config = super::UI.engine.config;
    let private_browsing = config.use_private_browsing(&uri);
    let use_plugins = config.use_plugins(&uri);
    let skip_content_filter = config.skip_content_filter(&uri);
    let block = ConcreteBlock::new(move |filter: id, err: id| {
        if let Some(window) = window_for_index(window_index) {
            let container = subview(window, WindowArea::WebView);
            for view in container.subviews().iter() {
                view.set_hidden(true);
            }
            let config = <id as WKWebViewConfiguration>::new().autorelease();
            if private_browsing {
                info!("blocking data storage in buffer");
                config.set_website_data_store(<id as WKWebsiteDataStore>::nonpersistent_store());
            }
            info!("setting plugins option to {}", use_plugins);
            config.preferences().set_plugins_enabled(use_plugins);
            if filter != nil && err == nil {
                config.user_content_controller().add_user_content_filter(filter);
            } else if err == nil {
                log_error_description(err);
            }
            let webview = <id as WKWebView>::new(CGRectZero(), config).autorelease();
            webview.set_navigation_delegate(WebViewHistoryDelegate::new());
            webview.disable_translates_autoresizing_mask_into_constraints();
            webview.set_custom_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/601.6.17 (KHTML, like Gecko) Version/9.1.1 Safari/601.6.17");
            container.add_subview(webview);
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Top, container, NSLayoutAttribute::Top));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Bottom, container, NSLayoutAttribute::Bottom));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Left, container, NSLayoutAttribute::Left));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Right, container, NSLayoutAttribute::Right));
            webview.load_request(NSURLRequest(&uri));
        }
    });
    if skip_content_filter {
        let block = block.copy();
        block.call((nil, nil));
    } else {
        store.lookup_content_extension("filter", &block.copy());
    }
}

unsafe fn window_webviews(window: id) -> id {
    subview(window, WindowArea::WebView).subviews()
}

unsafe fn create_nswindow() -> id {
    let mask = (NSTitledWindowMask as NSUInteger |
                NSMiniaturizableWindowMask as NSUInteger |
                NSResizableWindowMask as NSUInteger |
                NSFullSizeContentViewWindowMask as NSUInteger |
                NSClosableWindowMask as NSUInteger) as NSUInteger;
    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
        NSRect::new(NSPoint::new(0., 0.), NSSize::new(700., 700.)),
        mask,
        NSBackingStoreBuffered,
        NO
    ).autorelease();
    window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
    window.center();
    let title = <id as NSString>::from_str(WEBKITTEN_TITLE);
    window.setTitle_(title);
    layout_window_subviews(window);
    window
}

unsafe fn layout_window_subviews(window: id) {
    let container = <id as NSView>::new();
    let command_bar = <id as NSTextField>::new();
    window.contentView().add_subview(container);
    window.contentView().add_subview(command_bar);
    command_bar.disable_translates_autoresizing_mask_into_constraints();
    command_bar.set_height(BAR_HEIGHT as CGFloat);
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(command_bar, NSLayoutAttribute::Bottom, window.contentView(), NSLayoutAttribute::Bottom));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(command_bar, NSLayoutAttribute::Left, window.contentView(), NSLayoutAttribute::Left));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(command_bar, NSLayoutAttribute::Right, window.contentView(), NSLayoutAttribute::Right));
    container.disable_translates_autoresizing_mask_into_constraints();
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(container, NSLayoutAttribute::Top, window.contentView(), NSLayoutAttribute::Top));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(container, NSLayoutAttribute::Bottom, command_bar, NSLayoutAttribute::Top));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(container, NSLayoutAttribute::Left, window.contentView(), NSLayoutAttribute::Left));
    window.contentView().add_constraint(<id as NSLayoutConstraint>::bind(container, NSLayoutAttribute::Right, window.contentView(), NSLayoutAttribute::Right));
    window.makeKeyAndOrderFront_(nil);
    command_bar.set_delegate(CommandBarDelegate::new());
}
