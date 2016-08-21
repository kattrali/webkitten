use macos::{Id,nil,ObjCClass};
use macos::foundation::{NSRect,NSPoint,NSSize,NSArray,NSUInteger,NSInteger};
use macos::appkit::*;
use macos::core_graphics::{CGFloat,CGRect};
use macos::webkit::*;
use block::ConcreteBlock;
use webkitten::WEBKITTEN_TITLE;
use webkitten::ui::{BrowserConfiguration,WindowArea,BufferEvent,EventHandler};

use ui::{CocoaUI,UI};
use runtime::{CommandBarDelegate,WebViewHistoryDelegate,WebViewContainerView,
              log_error_description,CommandBarView};


const BAR_HEIGHT: usize = 24;

pub fn toggle(window_index: u32, visible: bool) {
    if let Some(window) = window_for_index(window_index) {
        match visible {
            true => window.make_key_and_order_front(),
            false => window.order_out()
        }
    }
}

pub fn open<T, B>(uri: Option<T>, config: Option<B>) -> u32
    where B: BrowserConfiguration,
          T: Into<String> {
    let window = create_nswindow();
    add_and_focus_webview(window.number(), uri, config);
    window.number()
}

pub fn focus(window_index: u32) {
    if let Some(window) = window_for_index(window_index) {
        window.make_key_and_order_front();
    }
}

pub fn focus_area(window_index: u32, area: WindowArea) {
    match area {
        WindowArea::WebView => focus_webview_area(window_index),
        WindowArea::CommandBar => focus_command_bar_area(window_index)
    }
}

fn focus_webview_area(window_index: u32) {
    let webview = focused_webview_index(window_index)
        .and_then(|webview_index| webview(window_index, webview_index))
        .and_then(|webview| webview.coerce::<NSResponder>());
    if let Some(webview) = webview {
        webview.become_first_responder();
    }
}

fn focus_command_bar_area(window_index: u32) {
    let bar = window_for_index(window_index)
        .and_then(|window| subview(&window, WindowArea::CommandBar).coerce::<NSResponder>());
    if let Some(bar) = bar {
        set_command_field_visible(window_index, true);
        bar.become_first_responder();
    }
}

pub fn focused_index() -> Option<u32> {
    let windows = nsapp().ordered_windows();
    for index in 0 .. windows.count() {
        if let Some(window) = windows.get::<NSWindow>(index) {
            if window.is_key_window() {
                return Some(window.number());
            }
        }
    }
    windows.get::<NSWindow>(0).map(|window| window.number())
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

pub fn open_webview<T, B>(window_index: u32, uri: Option<T>, config: Option<B>)
    where B: BrowserConfiguration,
          T: Into<String> {
    add_and_focus_webview(window_index, uri, config);
}

pub fn close_webview(window_index: u32, webview_index: u32) {
    if let Some(window) = window_for_index(window_index) {
        if let Some(focused_index) = focused_webview_index(window_index) {
            let is_focused = focused_index == webview_index;
            let webviews = window_webviews(&window);
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
                if !hidden {
                    view.coerce::<NSResponder>().unwrap().become_first_responder();
                    UI.engine.on_buffer_event::<CocoaUI<_>, _>(&UI, window_index,
                                                            webview_index,
                                                            None, BufferEvent::Focus);
                }
                info!("Set webview {} hidden: {}", index, hidden);
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

pub fn command_field_visible(window_index: u32) -> bool {
    let bar = window_for_index(window_index)
        .and_then(|window| subview(&window, WindowArea::CommandBar).coerce::<CommandBarView>());
    if let Some(bar) = bar {
        return bar.height() > 0 as CGFloat;
    }
    false
}

pub fn set_command_field_visible(window_index: u32, visible: bool) {
    let bar = window_for_index(window_index)
        .and_then(|window| subview(&window, WindowArea::CommandBar).coerce::<CommandBarView>());
    if let Some(bar) = bar {
        bar.set_height(if visible { BAR_HEIGHT } else { 0 } as CGFloat);
    }
}

pub fn focused_webview_index(window_index: u32) -> Option<u32> {
    if let Some(window) = window_for_index(window_index) {
        let webviews = window_webviews(&window);
        for index in 0 .. webviews.count() {
            if let Some(view) = webviews.get::<NSView>(index) {
                if !view.hidden() {
                    return Some(index as u32);
                }
            }
        }
    }
    None
}

pub fn webview_count(window_index: u32) -> u32 {
    window_for_index(window_index)
        .map(|w| window_webviews(&w).count() as u32)
        .unwrap_or(0)
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

fn add_and_focus_webview<T, B>(window_index: u32, uri: Option<T>, buffer_config: Option<B>)
    where B: BrowserConfiguration,
          T: Into<String> {
    let store = _WKUserContentExtensionStore::default_store();
    let ref config = super::UI.engine.config;
    let uri = uri.map(|u| u.into()).unwrap_or(String::new());
    let mut private_browsing = config.use_private_browsing(&uri);
    let mut use_plugins = config.use_plugins(&uri);
    let mut skip_content_filter = config.skip_content_filter(&uri);
    let mut use_js = config.use_javascript(&uri);
    if let Some(buffer_config) = buffer_config {
        private_browsing = buffer_config.use_private_browsing(&uri);
        use_plugins = buffer_config.use_plugins(&uri);
        skip_content_filter = buffer_config.skip_content_filter(&uri);
        use_js = buffer_config.use_javascript(&uri);
    }
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
            info!("setting js option to {}", use_js);
            config.preferences().set_javascript_enabled(use_js);
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
            if !uri.is_empty() {
                webview.load_request(super::create_request(&uri));
            }
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
    let container = WebViewContainerView::new().autorelease().coerce::<NSView>().unwrap();
    let command_bar = CommandBarView::new().autorelease();
    command_bar.set_delegate(&CommandBarDelegate::new());
    command_bar.set_height(BAR_HEIGHT as CGFloat);
    let ref config = super::UI.engine.config;
    let content_view = window.content_view().unwrap();
    let command_bar_view = command_bar.coerce::<NSView>().unwrap();
    content_view.add_subview(&container);
    content_view.add_subview(&command_bar_view);
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
