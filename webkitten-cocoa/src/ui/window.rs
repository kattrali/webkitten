use cocoa::base::{id,nil,NO,YES,BOOL};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSFastEnumeration,
                        NSAutoreleasePool};
use cocoa::appkit::{NSWindow, NSTitledWindowMask, NSResizableWindowMask,
                    NSMiniaturizableWindowMask, NSClosableWindowMask,
                    NSFullSizeContentViewWindowMask, NSBackingStoreBuffered};
use cocoa_ext::foundation::{NSArray,NSURLRequest,NSString,NSUInteger,NSInteger};
use cocoa_ext::appkit::{NSLayoutConstraint,NSLayoutAttribute,
                        NSConstraintBasedLayoutInstallingConstraints,
                        NSTextField,NSView,NSControl};
use cocoa_ext::core_graphics::CGRectZero;
use core_graphics::base::CGFloat;
use objc::runtime::Object;
use block::ConcreteBlock;

use webkitten::WEBKITTEN_TITLE;
use webkitten::ui::{BrowserConfiguration, WindowArea};
use webkit::*;
use runtime::{CommandBarDelegate,WebViewHistoryDelegate,WebViewContainerView,log_error_description};
use super::webview;

const BAR_HEIGHT: usize = 24;

pub fn toggle(window_index: u32, visible: bool) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            match visible {
                true => window.makeKeyAndOrderFront_(nil),
                false => window.orderOut_(nil)
            }
        }
    }
}

pub fn open<T: Into<String>>(uri: Option<T>) -> id {
    unsafe {
        let window = create_nswindow();
        let index = index_for_window(window) as u32;
        if let Some(uri) = uri {
            add_and_focus_webview(index, uri.into());
        }
        window
    }
}

pub fn focus(window_index: u32) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            window.makeKeyAndOrderFront_(nil);
        }
    }
}

pub fn focus_area(window_index: u32, area: WindowArea) {
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

pub fn focused_index() -> u32 {
    unsafe {
        let windows: id = msg_send![super::application::nsapp(), windows];
        for (index, window) in windows.iter().enumerate() {
            let key: BOOL = msg_send![window, isKeyWindow];
            if key == YES {
                return index_for_window(window);
            }
        }
        0
    }
}

pub fn close(window_index: u32) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            window.close();
        }
    }
}

pub fn title(window_index: u32) -> String {
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

pub fn set_title(window_index: u32, title: &str) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let title_str = <id as NSString>::from_str(title);
            window.setTitle_(title_str);
        }
    }
}

pub fn window_count() -> u32 {
    unsafe {
        let windows: id = msg_send![super::application::nsapp(), windows];
        windows.count() as u32
    }
}

pub fn open_webview<T: Into<String>>(window_index: u32, uri: T) {
    unsafe {
        add_and_focus_webview(window_index, uri.into());
    }
}

pub fn close_webview(window_index: u32, index: u32) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let webviews = window_webviews(window);
            let is_focused = focused_webview_index(window_index) == index;
            info!("Closing 1 webview of {}", webviews.count());
            let mut removed_view: Option<id> = None;
            for view in webviews.iter() {
                if index_for_webview(view) == index {
                    removed_view = Some(view);
                    break;
                }
            }
            if let Some(view) = removed_view {
                view.remove_from_superview();
                if is_focused {
                    if index as usize >= webviews.count() as usize {
                        focus_webview(window_index, 0);
                    } else {
                        focus_webview(window_index, index);
                    }
                }
            }
        }
    }
}

pub fn focus_webview(window_index: u32, webview_index: u32) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            info!("Focusing webview {} in window {}", webview_index, window_index);
            let container = subview(window, WindowArea::WebView);
            let subviews = container.subviews();
            if webview_index as usize >= subviews.count() as usize {
                return
            }
            for view in subviews.iter() {
                let index = index_for_webview(view);
                let hidden = webview_index != index;
                view.set_hidden(hidden);
                info!("Set webview #{} hidden: {}", index, hidden);
            }
        }
    }
}

pub fn webview(window_index: u32, webview_index: u32) -> Option<id> {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            window_webviews(window).get(webview_index as NSUInteger)
        } else {
            None
        }
    }
}

pub fn resize(window_index: u32, width: u32, height: u32) {
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

pub fn command_field_text(window_index: u32) -> String {
    field_text(window_index, WindowArea::CommandBar)
}

fn field_text(window_index: u32, view: WindowArea) -> String {
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

pub fn set_command_field_text(window_index: u32, text: &str) {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            let bar = subview(window, WindowArea::CommandBar);
            bar.set_string_value(text);
        }
    }
}

pub fn focused_webview_index(window_index: u32) -> u32 {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            for view in window_webviews(window).iter() {
                if view.hidden() == NO {
                    return index_for_webview(view)
                }
            }
        }
    }
    0
}

pub fn webview_count(window_index: u32) -> u32 {
    unsafe {
        if let Some(window) = window_for_index(window_index) {
            window_webviews(window).count() as u32
        } else {
            0
        }
    }
}

pub fn reference_indices(webview: id) -> Option<(u32, u32)> {
    unsafe {
        let window: id = msg_send![webview, window];
        if window != nil {
            let window_index  = index_for_window(window);
            let webview_index = index_for_webview(webview);
            return Some((window_index, webview_index));
        }
    }
    None
}

unsafe fn window_for_index(index: u32) -> Option<id> {
    info!("Looking up window for index: {}", index);
    let window: id = unsafe {
        msg_send![super::application::nsapp(), windowWithWindowNumber:index as NSInteger]
    };
    if window != nil {
        Some(window)
    } else {
        let indices = window_indices();
        if (index as usize) < indices.len() {
            let converted_index = indices[index as usize];
            if converted_index != index {
                debug!("Loading window with fallback index");
                return window_for_index(converted_index);
            }
        }
        warn!("No window found for index");
        None
    }
}

unsafe fn index_for_window(window: id) -> u32 {
    let index: u32 = msg_send![window, windowNumber];
    index
}

unsafe fn index_for_webview(webview: id) -> u32 {
    let window: id = msg_send![webview, window];
    let webviews = window_webviews(window);
    for (index, view) in webviews.iter().enumerate() {
        if view == webview {
            return index as u32;
        }
    }
    return 0u32;
}

unsafe fn window_indices() -> Vec<u32> {
    let windows: id = msg_send![super::application::nsapp(), windows];
    let mut indices: Vec<u32> = vec![];
    for window in windows.iter() {
        indices.push(index_for_window(window));
    }
    indices
}

unsafe fn subview(window: id, area: WindowArea) -> id {
    let index = match area {
        WindowArea::WebView => 0,
        WindowArea::CommandBar => 1
    };
    let subviews = window.contentView().subviews();
    msg_send![subviews, objectAtIndex:index]
}

unsafe fn add_and_focus_webview(window_index: u32, uri: String) {
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
            let webview = <id as WKWebView>::new(CGRectZero(), config, webview_count(window_index)).autorelease();
            webview.set_navigation_delegate(WebViewHistoryDelegate::new());
            webview.disable_translates_autoresizing_mask_into_constraints();
            webview.set_custom_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/601.6.17 (KHTML, like Gecko) Version/9.1.1 Safari/601.6.17");
            container.add_subview(webview);
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Top, container, NSLayoutAttribute::Top));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Bottom, container, NSLayoutAttribute::Bottom));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Left, container, NSLayoutAttribute::Left));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Right, container, NSLayoutAttribute::Right));
            webview::load_uri(webview, &uri);
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
    let container = WebViewContainerView::new();
    let command_bar = <id as NSTextField>::new();
    let ref config = super::UI.engine.config;
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
    if let Some((family, size)) = config.bar_font() {
        command_bar.set_font(&family, size);
    }
}
