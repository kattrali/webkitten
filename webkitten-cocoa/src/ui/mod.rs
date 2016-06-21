mod webview;
mod window;

use std::{slice,str};
use libc::c_char;

use cocoa::base::{selector,id,nil};
use cocoa::foundation::{NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicyRegular,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps};
use cocoa_ext::foundation::{NSDictionary,NSNotification,NSNumber};
use cocoa_ext::appkit::{NSControl};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};

use webkitten::ui::{ApplicationUI,EventHandler,BrowserWindow,WebView};
use webkitten::{Engine};
use self::webview::CocoaWebView;
use self::window::{CocoaWindow,ABDELEGATE_CLASS,CBDELEGATE_CLASS};

pub struct CocoaUI {
    engine: Engine
}

impl ApplicationUI for CocoaUI {

    fn new(engine: Engine) -> Option<Self> {
        Some(CocoaUI {engine: engine})
    }

    fn event_handler(&self) -> &Engine {
       &self.engine
    }

    fn run(&self) {
        declare_bar_delegates();
        if let Some(start_page) = self.event_handler().config.lookup("window.start-page") {
            self.open_window(start_page.as_str());
        }
        start_run_loop();
    }

    fn open_window(&self, uri: Option<&str>) {
        let window = CocoaWindow::new();
        if let Some(uri) = uri {
            window.open_webview(uri);
        }
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

fn start_run_loop() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        nsapp().setActivationPolicy_(NSApplicationActivationPolicyRegular);
        create_menu();
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        nsapp().run();
    }
}

unsafe fn create_menu() {
    let menubar = NSMenu::new(nil).autorelease();
    let app_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(app_menu_item);
    nsapp().setMainMenu_(menubar);
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
}

unsafe fn nsapp() -> id {
    NSApplication::sharedApplication(nil)
}


fn declare_bar_delegates() {
    if let Some(superclass) = Class::get("NSObject") {
        if let Some(mut decl) = ClassDecl::new(CBDELEGATE_CLASS, superclass) {
            unsafe {
                decl.add_method(sel!(controlTextDidEndEditing:),
                    command_bar_did_end_editing as extern fn(&Object, Sel, id));
            }

            decl.register();
        }
        if let Some(mut decl) = ClassDecl::new(ABDELEGATE_CLASS, superclass) {
            unsafe {
                decl.add_method(sel!(controlTextDidEndEditing:),
                    address_bar_did_end_editing as extern fn(&Object, Sel, id));
            }

            decl.register();
        }
    }
}

extern fn command_bar_did_end_editing(this: &Object, _cmd: Sel, notification: id) {
    if let Some(text) = notification_object_text(notification) {
        super::UI.engine.execute_command::<CocoaUI>(&super::UI, 0, 0, text);
    }
}

extern fn address_bar_did_end_editing(this: &Object, _cmd: Sel, notification: id) {
    if let Some(text) = notification_object_text(notification) {
        super::UI.engine.update_address::<CocoaUI>(&super::UI, 0, 0, text);
    }
}

fn notification_object_text<'a>(notification: id) -> Option<&'a str> {
    if is_return_key_event(notification) {
        let raw_text = unsafe {
            let control = notification.object();
            control.string_value()
        };
        return nsstring_as_str(raw_text);
    }
    None
}

fn nsstring_as_str<'a>(nsstring: id) -> Option<&'a str> {
    let bytes = unsafe {
        let bytes: *const c_char = nsstring.UTF8String();
        let byte_str = bytes as *const u8;
        let len = nsstring.len();
        slice::from_raw_parts(byte_str, len)
    };
    str::from_utf8(bytes).ok()
}

fn is_return_key_event(notification: id) -> bool {
    let keycode = unsafe {
        notification.user_info().object_for_key("NSTextMovement").integer_value()
    };
    keycode == 0x10
}
