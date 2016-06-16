use std::cell::RefCell;
use webkitten::ui::{ApplicationUI,EventHandler,BrowserWindow,WebView};
use webkitten::{WEBKITTEN_TITLE,Engine};
use cocoa::base::{selector,id,nil,NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
                        NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApp,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSResizableWindowMask,
                    NSMiniaturizableWindowMask, NSClosableWindowMask,
                    NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps, NSView};
use block::ConcreteBlock;
use webkit::*;
use foundation::*;

pub struct CocoaUI {
    handler: Engine,
    nsapp: id,
    windows: RefCell<Vec<CocoaWindow>>
}

pub struct CocoaWindow {
    nswindow: id
}

pub struct CocoaWebView {
    wkwebview: RefCell<id>
}

impl ApplicationUI for CocoaUI {

    fn new(handler: Engine) -> Option<Self> {
        Some(CocoaUI {
            handler: handler,
            nsapp: unsafe { NSApp() },
            windows: RefCell::new(vec![])
        })
    }

    fn event_handler(&self) -> &Engine {
        &self.handler
    }

    fn run(&mut self) {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            self.nsapp.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            // create Menu Bar
            let menubar = NSMenu::new(nil).autorelease();
            let app_menu_item = NSMenuItem::new(nil).autorelease();
            menubar.addItem_(app_menu_item);
            self.nsapp.setMainMenu_(menubar);

            // create Application menu
            let app_menu = NSMenu::new(nil).autorelease();
            let quit_prefix = NSString::alloc(nil).init_str("Quit");
            let quit_title = quit_prefix.stringByAppendingString_(
                NSProcessInfo::processInfo(nil).processName()
            );
            let quit_action = selector("terminate:");
            let quit_key = NSString::alloc(nil).init_str("q");
            let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
                quit_title,
                quit_action,
                quit_key
            ).autorelease();
            app_menu.addItem_(quit_item);
            app_menu_item.setSubmenu_(app_menu);
            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
            self.nsapp.run();
        }
    }

    fn open_window(&self, uri: &str) {
        let window = CocoaWindow::new();
        let mut windows = self.windows.borrow_mut();
        windows.push(window);
    }

    fn window<B: BrowserWindow>(&self, index: u8) -> Option<&B> {
        None
    }

    fn focused_window_index(&self) -> u8 {
        0
    }

    fn focus_window(&self, index: u8) {
    }

    fn window_count(&self) -> u8 {
        0
    }
}

impl BrowserWindow for CocoaWindow {

    fn new() -> Self {
        let window = unsafe {
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
            window.makeKeyAndOrderFront_(nil);
            window
        };
        CocoaWindow { nswindow: window }
    }

    fn show(&self) {
        unsafe { self.nswindow.makeKeyAndOrderFront_(nil); }
    }

    fn hide(&self) {
        unsafe { self.nswindow.orderOut_(nil); }
    }

    fn open_webview(&self, uri: String) {
    }

    fn close_webview(&self, index: u8) {
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

impl WebView for CocoaWebView {

    fn load_uri(&self, uri: &str) {
        let webview = self.wkwebview.borrow();
        webview.load_request(NSURLRequest::with_url(nil, NSURL(uri)));
    }

    fn go_back(&self) {
    }

    fn go_forward(&self) {
    }

    fn focus(&self) {
    }

    fn raw_html(&self) -> String {
        String::new()
    }

    fn uri(&self) -> String {
        String::new()
    }

    fn title(&self) -> String {
        String::new()
    }

    fn apply_javascript(&self, script: &str) {
    }

    fn apply_styles(&self, styles: &str) {
    }

    fn apply_content_filters(&self, identifier: &str, rules: &str) {
        unsafe {
            let store = _WKUserContentExtensionStore::default_store(nil);
            store.compile_content_extension(identifier,
                                            rules,
                                            ConcreteBlock::new(move |filter: id, err: id| {
                let mut webview = self.wkwebview.borrow_mut();
                if err == nil {
                    webview.configuration().user_content_controller().add_user_content_filter(filter);
                } else {
                    println!("failed to load extension");
                }
            }));
        }
    }
}
