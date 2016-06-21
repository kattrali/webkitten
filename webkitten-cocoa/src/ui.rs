use std::cell::RefCell;
use webkitten::ui::{ApplicationUI,EventHandler,BrowserWindow,WebView};
use webkitten::{WEBKITTEN_TITLE,Engine};
use cocoa::base::{selector,id,nil,NO,class};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize, NSFastEnumeration,
                        NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApp,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSResizableWindowMask,
                    NSMiniaturizableWindowMask, NSClosableWindowMask,
                    NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps};
use block::ConcreteBlock;
use webkit::*;
use cocoa_ext::foundation::{NSURLRequest,NSArray,NSDictionary,NSNotification,
                            NSNumber};
use cocoa_ext::appkit::{NSLayoutConstraint,NSLayoutAttribute,NSLayoutRelation,
                        NSConstraintBasedLayoutInstallingConstraints,
                        NSTextField,NSView,NSControl};
use cocoa_ext::core_graphics::CGRectZero;
use core_graphics::base::CGFloat;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use libc::c_char;
use std::{slice,str};

const BAR_HEIGHT: usize = 26;
const ABDELEGATE_CLASS: &'static str = "AddressBarDelegate";
const CBDELEGATE_CLASS: &'static str = "CommandBarDelegate";

enum CocoaWindowSubview {
    AddressBar       = 0,
    WebViewContainer = 1,
    CommandBar       = 2,
}

pub struct CocoaUI {
    engine: Engine
}

pub struct CocoaWindow {
    nswindow: id
}

pub struct CocoaWebView {
    wkwebview: RefCell<id>
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

impl BrowserWindow for CocoaWindow {

    fn new() -> Self {
        let window = CocoaWindow { nswindow: CocoaWindow::create_nswindow() };
        window.configure_subviews();
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

    fn create_nswindow() -> id {
        unsafe {
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
    }

    fn configure_subviews(&self) -> (id, id) {
        let window = self.nswindow;
        unsafe {
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
    }

    fn add_and_focus_webview(&self) -> id {
        unsafe {
            let container = self.subview(CocoaWindowSubview::WebViewContainer);
            for view in container.subviews().iter() {
                view.set_hidden(true);
            }
            let webview = WKWebView(CGRectZero(), WKWebViewConfiguration().autorelease()).autorelease();
            webview.disable_translates_autoresizing_mask_into_constraints();
            container.add_subview(webview);
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Top, container, NSLayoutAttribute::Top));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Bottom, container, NSLayoutAttribute::Bottom));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Left, container, NSLayoutAttribute::Left));
            container.add_constraint(<id as NSLayoutConstraint>::bind(webview, NSLayoutAttribute::Right, container, NSLayoutAttribute::Right));
            webview
        }
    }
}

impl WebView for CocoaWebView {

    fn load_uri(&self, uri: &str) {
        let webview = self.wkwebview.borrow();
        unsafe { webview.load_request(NSURLRequest(uri)); }
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
            //let store = _WKUserContentExtensionStore::default_store(nil);
            //store.compile_content_extension(identifier,
                                            //rules,
                                            //ConcreteBlock::new(move |filter: id, err: id| {
                //let mut webview = self.wkwebview.borrow_mut();
                //if err == nil {
                    //webview.configuration().user_content_controller().add_user_content_filter(filter);
                //} else {
                    //println!("failed to load extension");
                //}
            //}));
        }
    }
}

//let superclass = NSObject::class();
//if let Some(mut decl) = ClassDecl::new("AddressBarDelegate", superclass) {
    //extern fn check_input(this: &mut Object, _cmd: Sel, control: id, editor: id) {
        //println!("Whammo!");
    //}
    //decl.add_method(sel!(control:textShouldEndEditing:), check_input);
//}
