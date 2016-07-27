use macos::{Id,nil,ObjCClass};
use macos::foundation::{NSRect,NSPoint,NSSize,NSArray,NSUInteger,NSInteger};
use macos::appkit::*;
use macos::core_graphics::{CGFloat,CGRect};
use macos::webkit::*;
use block::ConcreteBlock;
use webkitten::WEBKITTEN_TITLE;
use webkitten::ui::{BrowserConfiguration,WindowArea};

use runtime::{CommandBarDelegate,WebViewHistoryDelegate,WebViewContainerView,
              log_error_description};
use super::CocoaUI;


pub fn toggle(window_index: u32, visible: bool) {
    if let Some(window) = window_for_index(window_index) {
        match visible {
            true => window.make_key_and_order_front(),
            false => window.order_out()
        }
    }
}

pub fn open<T: Into<String>>(uri: Option<T>) -> NSWindow {
    let window = create_nswindow();
    if let Some(uri) = uri {
        add_and_focus_webview(window.number(), uri.into());
    }
    window
}

pub fn focus(window_index: u32) {
    if let Some(window) = window_for_index(window_index) {
        window.make_key_and_order_front();
    }
}

pub fn focus_area(window_index: u32, area: WindowArea) {
    match area {
        WindowArea::WebView => {
            if let Some(webview) = webview(window_index, focused_webview_index(window_index)) {
                webview.coerce::<NSResponder>().unwrap().become_first_responder();
            }
        },
        WindowArea::CommandBar => {
            if let Some(window) = window_for_index(window_index) {
                subview(&window, area).coerce::<NSResponder>().unwrap().become_first_responder();
            }
        }
    }
}

pub fn focused_index() -> u32 {
    let windows = nsapp().windows();
    for index in 0 .. windows.count() {
        if let Some(window) = windows.get::<NSWindow>(index) {
            if window.is_key_window() {
                return window.number() as u32;
            }
        }
    }
    0
}

pub fn close(window_index: u32) {
    if let Some(mut window) = window_for_index(window_index) {
        let webviews = window_webviews(&window);
        for index in 0 .. webviews.count() {
            if let Some(view) = webviews.get::<WKWebView>(index) {
                view.remove_from_superview();
                view.release_delegates();
                view.close();
            }
        }
        window.release_delegate();
        window.close();
    }
}

pub fn title(window_index: u32) -> String {
    window_for_index(window_index)
        .and_then(|win| win.title())
        .and_then(|title| title.as_str())
        .and_then(|title| Some(String::from(title)))
        .unwrap_or(String::new())
}

pub fn set_title(window_index: u32, title: &str) {
    if let Some(window) = window_for_index(window_index) {
        window.set_title(title);
    }
}

pub fn open_webview<T: Into<String>>(window_index: u32, uri: T) {
    add_and_focus_webview(window_index, uri.into());
}

pub fn close_webview(window_index: u32, webview_index: u32) {
    if let Some(window) = window_for_index(window_index) {
        let webviews = window_webviews(&window);
        let is_focused = focused_webview_index(window_index) == webview_index;
        info!("Closing 1 webview of {}", webviews.count());
        for index in 0 .. webviews.count() {
            if let Some(view) = webviews.get::<WKWebView>(index) {
                if index == (webview_index as NSUInteger) {
                    view.remove_from_superview();
                    view.release_delegates();
                    view.close();
                    if is_focused {
                        if index as usize >= webviews.count() as usize {
                            focus_webview(window_index, 0);
                        } else {
                            focus_webview(window_index, index as u32);
                        }
                    }
                    break;
                }
            }
        }
    }
}

pub fn focus_webview(window_index: u32, webview_index: u32) {
    if let Some(window) = window_for_index(window_index) {
        info!("Focusing webview {} in window {}", webview_index, window_index);
        let container = subview(&window, WindowArea::WebView);
        let subviews = container.subviews().unwrap();
        if webview_index as usize >= subviews.count() as usize {
            return
        }
        for index in 0 .. subviews.count() {
            if let Some(view) = subviews.get::<NSView>(index) {
                let hidden = (webview_index as NSUInteger) != index;
                view.set_hidden(hidden);
                info!("Set webview #{} hidden: {}", index, hidden);
            }
        }
    }
}

pub fn webview(window_index: u32, webview_index: u32) -> Option<WKWebView> {
    window_for_index(window_index)
        .and_then(|window| {
            window_webviews(&window).get::<WKWebView>(webview_index as NSUInteger)
        })
}

pub fn resize(window_index: u32, width: u32, height: u32) {
    if let Some(window) = window_for_index(window_index) {
        let frame = NSRect {
            origin: window.frame().origin,
            size: NSSize { width: width as CGFloat, height: height as CGFloat }
        };
        window.set_frame(frame);
    }
}

pub fn command_field_text(window_index: u32) -> String {
    field_text(window_index, WindowArea::CommandBar)
}

fn field_text(window_index: u32, view: WindowArea) -> String {
    window_for_index(window_index)
        .and_then(|window| Some(subview(&window, view)))
        .and_then(|view| view.coerce::<NSControl>())
        .and_then(|control| control.text())
        .and_then(|text| text.as_str())
        .and_then(|text| Some(String::from(text)))
        .unwrap_or(String::new())
}

pub fn set_command_field_text(window_index: u32, text: &str) {
    if let Some(window) = window_for_index(window_index) {
        if let Some(bar) = subview(&window, WindowArea::CommandBar).coerce::<NSControl>() {
            bar.set_text(text);
        }
    }
}

pub fn focused_webview_index(window_index: u32) -> u32 {
    if let Some(window) = window_for_index(window_index) {
        let webviews = window_webviews(&window);
        for index in 0 .. webviews.count() {
            if let Some(view) = webviews.get::<NSView>(index) {
                if !view.hidden() {
                    return index as u32;
                }
            }
        }
    }
    0
}

pub fn webview_count(window_index: u32) -> u32 {
    if let Some(window) = window_for_index(window_index) {
        window_webviews(&window).count() as u32
    } else {
        0
    }
}

fn window_for_index(index: u32) -> Option<NSWindow> {
    info!("Looking up window for index: {}", index);
    if let Some(window) = nsapp().window_by_number(index as NSInteger) {
        return Some(window)
    }
    info!("Using fallback window lookup");
    nsapp().window_by_index(index as NSUInteger)
}

fn subview(window: &NSWindow, area: WindowArea) -> NSView {
    let index = match area {
        WindowArea::WebView => 0,
        WindowArea::CommandBar => 1
    };
    let subviews = window.content_view().unwrap().subviews().unwrap();
    subviews.get::<NSView>(index).unwrap()
}

fn add_and_focus_webview(window_index: u32, uri: String) {
    let store = _WKUserContentExtensionStore::default_store();
    let ref config = super::UI.engine.config;
    let private_browsing = config.use_private_browsing(&uri);
    let use_plugins = config.use_plugins(&uri);
    let skip_content_filter = config.skip_content_filter(&uri);
    let block = ConcreteBlock::new(move |filter: Id, err: Id| {
        if let Some(window) = window_for_index(window_index) {
            let container = subview(&window, WindowArea::WebView);
            let subviews = container.subviews().unwrap();
            for index in 0 .. subviews.count() {
                if let Some(view) = subviews.get::<NSView>(index) {
                    view.set_hidden(true);
                }
            }
            let config = WKWebViewConfiguration::new().autorelease();
            if private_browsing {
                info!("blocking data storage in buffer");
                config.set_website_data_store(WKWebsiteDataStore::nonpersistent_store());
            }
            info!("setting plugins option to {}", use_plugins);
            config.preferences().set_plugins_enabled(use_plugins);
            if let Some(filter) = _WKUserContentFilter::from_ptr(filter) {
                config.user_content_controller().add_user_content_filter(filter);
            } else if err != nil {
                log_error_description(err);
            }
            let webview = WKWebView::new(CGRect::zero(), config).autorelease();
            webview.set_navigation_delegate(WebViewHistoryDelegate::new());
            webview.set_custom_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/601.6.17 (KHTML, like Gecko) Version/9.1.1 Safari/601.6.17");
            let webview_view = webview.coerce::<NSView>().unwrap();
            webview_view.disable_translates_autoresizing_mask_into_constraints();
            container.add_subview(&webview_view);
            container.add_constraint(NSLayoutConstraint::bind(&webview_view, NSLayoutAttribute::Top, &container, NSLayoutAttribute::Top));
            container.add_constraint(NSLayoutConstraint::bind(&webview_view, NSLayoutAttribute::Bottom, &container, NSLayoutAttribute::Bottom));
            container.add_constraint(NSLayoutConstraint::bind(&webview_view, NSLayoutAttribute::Left, &container, NSLayoutAttribute::Left));
            container.add_constraint(NSLayoutConstraint::bind(&webview_view, NSLayoutAttribute::Right, &container, NSLayoutAttribute::Right));
            webview.load_request(CocoaUI::create_request(&uri));
        }
    });
    if skip_content_filter {
        let block = block.copy();
        unsafe { block.call((nil, nil)); }
    } else {
        store.lookup_content_extension("filter", &block.copy());
    }
}

fn window_webviews(window: &NSWindow) -> NSArray {
    subview(window, WindowArea::WebView).subviews().unwrap()
}

fn create_nswindow() -> NSWindow {
    let mask = NSWindowMask::Titled as NSUInteger |
               NSWindowMask::Miniaturizable as NSUInteger |
               NSWindowMask::Resizable as NSUInteger |
               NSWindowMask::FullSizeContentView as NSUInteger |
               NSWindowMask::Closable as NSUInteger;
    let frame = NSRect {
        origin: NSPoint { x: 0., y: 0. },
        size: NSSize { width: 700., height: 700. }
    };
    let window = NSWindow::new(frame, mask,
                               NSBackingStoreType::Buffered, false);
    window.cascade_top_left_from_point(NSPoint { x: 20., y: 20. });
    window.center();
    window.set_title(WEBKITTEN_TITLE);
    layout_window_subviews(&window);
    window
}

fn layout_window_subviews(window: &NSWindow) {
    const BAR_HEIGHT: usize = 24;

    let container = WebViewContainerView::new().autorelease().coerce::<NSView>().unwrap();
    let command_bar = NSTextField::new().autorelease();
    command_bar.set_delegate(&CommandBarDelegate::new());
    let ref config = super::UI.engine.config;
    let content_view = window.content_view().unwrap();
    let command_bar_view = command_bar.coerce::<NSView>().unwrap();
    content_view.add_subview(&container);
    content_view.add_subview(&command_bar_view);
    command_bar_view.set_height(BAR_HEIGHT as CGFloat);
    command_bar_view.disable_translates_autoresizing_mask_into_constraints();
    content_view.add_constraint(NSLayoutConstraint::bind(&command_bar_view, NSLayoutAttribute::Bottom, &content_view, NSLayoutAttribute::Bottom));
    content_view.add_constraint(NSLayoutConstraint::bind(&command_bar_view, NSLayoutAttribute::Left, &content_view, NSLayoutAttribute::Left));
    content_view.add_constraint(NSLayoutConstraint::bind(&command_bar_view, NSLayoutAttribute::Right, &content_view, NSLayoutAttribute::Right));
    container.disable_translates_autoresizing_mask_into_constraints();
    content_view.add_constraint(NSLayoutConstraint::bind(&container, NSLayoutAttribute::Top, &content_view, NSLayoutAttribute::Top));
    content_view.add_constraint(NSLayoutConstraint::bind(&container, NSLayoutAttribute::Bottom, &command_bar_view, NSLayoutAttribute::Top));
    content_view.add_constraint(NSLayoutConstraint::bind(&container, NSLayoutAttribute::Left, &content_view, NSLayoutAttribute::Left));
    content_view.add_constraint(NSLayoutConstraint::bind(&container, NSLayoutAttribute::Right, &content_view, NSLayoutAttribute::Right));
    window.make_key_and_order_front();
    if let Some((family, size)) = config.bar_font() {
        command_bar_view.coerce::<NSControl>().unwrap().set_font(&family, size as isize);
    }
}
