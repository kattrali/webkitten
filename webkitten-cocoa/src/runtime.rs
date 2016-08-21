use objc::declare::ClassDecl;
use objc::runtime::{Class,Object,Sel,BOOL,YES,NO};
use macos::{Id,ObjCClass};
use macos::foundation::*;
use macos::appkit::{NSControl,NSEvent,NSView,NSEventModifierFlags,
                    NSLayoutConstraint,NSWorkspace};
use macos::core_services::register_default_scheme_handler;
use macos::core_graphics::CGFloat;
use macos::webkit::*;
use webkitten::ui::{ApplicationUI,EventHandler,BrowserConfiguration,BufferEvent};
use webkitten::{WEBKITTEN_APP_ID,WEBKITTEN_TITLE};
use webkitten::config::Config;
use block::Block;

use ui::{CocoaUI,UI};

const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

impl_objc_class!(CommandBarDelegate);
impl_objc_class!(WebViewHistoryDelegate);
impl_objc_class!(WebViewContainerView);
impl_objc_class!(KeyInputDelegate);
impl_objc_class!(AppDelegate);
impl_objc_class!(CommandBarView);

impl CommandBarDelegate {
    pub fn new() -> Self {
        CommandBarDelegate {
            ptr: unsafe { msg_send![class!("CommandBarDelegate"), new] }
        }
    }
}

impl WebViewHistoryDelegate {
    pub fn new() -> Self {
        WebViewHistoryDelegate {
            ptr: unsafe { msg_send![class!("WebViewHistoryDelegate"), new] }
        }
    }
}

impl AppDelegate {
    pub fn new() -> Self {
        AppDelegate {
            ptr: unsafe { msg_send![class!("AppDelegate"), new] }
        }
    }
}

impl WebViewContainerView {
    pub fn new() -> Self {
        WebViewContainerView {
            ptr: unsafe { msg_send![class!("WebViewContainerView"), new] }
        }
    }
}

impl KeyInputDelegate {
    pub fn new(command: &str) -> Self {
        let ptr = unsafe {
            let delegate: *mut Object = msg_send![class!("KeyInputDelegate"), new];
            let obj = &mut *(delegate as *mut _ as *mut Object);
            obj.set_ivar("_command", NSString::from(command).ptr());
            delegate
        };
        KeyInputDelegate { ptr: ptr }
    }

    pub fn command(&self) -> Option<NSString> {
        let obj = unsafe { &mut *(self.ptr as *mut _ as *mut Object) };
        NSString::from_ptr(unsafe { *obj.get_ivar("_command") })
    }
}

impl CommandBarView {

    pub fn new() -> Self {
        let ptr = unsafe {
            let view: Id = msg_send![class!(CommandBarView::class_name()), new];
            msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];
            view
        };
        CommandBarView { ptr: ptr }
    }

    pub fn set_delegate<T: ObjCClass>(&self, delegate: &T) {
        unsafe { msg_send![self.ptr, setDelegate:delegate.ptr()] }
    }

    pub fn height(&self) -> CGFloat {
        if let Some(constraint) = self.height_constraint() {
            constraint.constant()
        } else {
            0 as CGFloat
        }
    }

    pub fn set_height(&self, height: CGFloat) {
        if let Some(constraint) = self.height_constraint() {
            if constraint.constant() != height {
                constraint.set_constant(height);
            }
        } else {
            let view = self.coerce::<NSView>().unwrap();
            let constraint = NSLayoutConstraint::height_constraint(&view, height);
            self.set_height_constraint(&constraint);
            view.add_constraint(constraint);
        }
    }

    fn height_constraint(&self) -> Option<NSLayoutConstraint> {
        let obj = unsafe { &mut *(self.ptr as *mut _ as *mut Object) };
        NSLayoutConstraint::from_ptr(unsafe { *obj.get_ivar("_heightConstraint") })
    }

    fn set_height_constraint(&self, constraint: &NSLayoutConstraint) {
       unsafe {
            let obj = &mut *(self.ptr as *mut _ as *mut Object);
            obj.set_ivar("_heightConstraint", constraint.ptr());
       }
    }
}

pub fn log_error_description(err: Id) {
    let desc = NSError::from_ptr(err)
        .and_then(|err| err.description())
        .and_then(|desc| desc.as_str());
    if let Some(desc) = desc {
        error!("{}", desc);
    }
}

pub fn declare_classes() {
    declare_view_classes();
    declare_app_delegates();
    declare_bar_delegate();
    declare_webview_delegates();
}

fn declare_view_classes() {
    let mut container = ClassDecl::new(WebViewContainerView::class_name(), class!("NSView")).unwrap();
    unsafe {
        container.add_method(sel!(acceptsFirstResponder),
            container_accepts_first_responder as extern fn (&Object, Sel) -> BOOL);
        container.add_method(sel!(keyDown:),
            container_key_down as extern fn (&mut Object, Sel, Id));
    }
    container.register();
    let mut bar = ClassDecl::new(CommandBarView::class_name(), class!("NSTextField")).unwrap();
    bar.add_ivar::<Id>("_heightConstraint");
    bar.register();
}

fn declare_app_delegates() {
    let mut key_input = ClassDecl::new(KeyInputDelegate::class_name(), class!("NSObject")).unwrap();
    key_input.add_ivar::<Id>("_command");
    unsafe {
        key_input.add_method(sel!(runKeybindingCommand),
            run_keybinding_command as extern fn(&mut Object, Sel));
    }
    key_input.register();
    let mut app_delegate = ClassDecl::new(AppDelegate::class_name(), class!("NSObject")).unwrap();
    unsafe {
        app_delegate.add_method(sel!(applicationWillFinishLaunching:),
            app_will_finish_launching as extern fn (&mut Object, Sel, Id));
        app_delegate.add_method(sel!(applicationDidFinishLaunching:),
            app_finished_launching as extern fn (&Object, Sel, Id));
        app_delegate.add_method(sel!(application:openFile:),
            open_file as extern fn (&Object, Sel, Id, Id) -> BOOL);
        app_delegate.add_method(sel!(setAsDefaultBrowser),
            set_as_default_browser as extern fn (&Object, Sel));
        app_delegate.add_method(sel!(handleGetURLEvent:withReplyEvent:),
            handle_get_url as extern fn (&Object, Sel, Id, Id));
    }
    app_delegate.register();
}

fn declare_bar_delegate() {
    let mut decl = ClassDecl::new(CommandBarDelegate::class_name(), class!("NSObject")).unwrap();
    unsafe {
        decl.add_method(sel!(controlTextDidChange:),
            command_bar_text_changed as extern fn(&Object, Sel, Id));
        decl.add_method(sel!(controlTextDidEndEditing:),
            command_bar_did_end_editing as extern fn(&Object, Sel, Id));
        decl.add_method(sel!(control:textView:completions:forPartialWordRange:indexOfSelectedItem:),
            command_bar_get_completion as extern fn(&Object, Sel, Id, Id, Id, NSRange, Id) -> Id);
    }
    decl.register();
}

fn declare_webview_delegates() {
    let mut decl = ClassDecl::new(WebViewHistoryDelegate::class_name(), class!("NSObject")).unwrap();
    unsafe {
        decl.add_method(sel!(webView:didStartProvisionalNavigation:),
            webview_will_load as extern fn (&Object, Sel, Id, Id));
        decl.add_method(sel!(_webView:navigation:didSameDocumentNavigation:),
            webview_did_same_nav as extern fn (&Object, Sel, Id, Id, Id));
        decl.add_method(sel!(_webView:navigationDidFinishDocumentLoad:),
            webview_did_load as extern fn (&Object, Sel, Id, Id));
        decl.add_method(sel!(webView:didFailProvisionalNavigation:withError:),
            webview_load_failed as extern fn (&Object, Sel, Id, Id, Id));
        decl.add_method(sel!(webView:didFailNavigation:withError:),
            webview_load_failed as extern fn (&Object, Sel, Id, Id, Id));
        decl.add_method(sel!(webView:decidePolicyForNavigationAction:decisionHandler:),
            webview_will_navigate as extern fn (&Object, Sel, Id, Id, Id));
    }
    decl.register();
}

pub fn default_user_agent() -> String {
    let os_version = NSProcessInfo::process_info().os_version();
    let minor_version = &format!("{}", os_version.minorVersion);
    let os_version_string = format!("Macintosh; Intel Mac OS X {}_{}_{}",
                                    os_version.majorVersion,
                                    os_version.minorVersion,
                                    os_version.patchVersion);
    let webkit_version = NSBundle::from_class(class!("WKView"))
        .and_then(|bundle| bundle.get_info_dict_object::<NSString>("CFBundleVersion"))
        .and_then(|version| version.as_str())
        .and_then(|version| Some(version.trim_left_matches(minor_version)));
    let webkitten_version_string = format!("{}/{}", WEBKITTEN_TITLE, APP_VERSION);
    if let Some(webkit_version) = webkit_version {
        format!("Mozilla/5.0 ({}) AppleWebKit/{wkversion} {app_version} Version/9.1.1 Safari/{wkversion}",
                os_version_string,
                wkversion = webkit_version,
                app_version = webkitten_version_string)
    } else {
        format!("Mozilla/5.0 ({}) AppleWebKit (KHTML, like Gecko) {app_version} Safari",
                os_version_string,
                app_version = webkitten_version_string)
    }
}

extern fn set_as_default_browser(_: &Object, _cmd: Sel) {
    register_default_scheme_handler("http", WEBKITTEN_APP_ID);
    register_default_scheme_handler("https", WEBKITTEN_APP_ID);
}

extern fn open_file(_: &Object, _cmd: Sel, _app: Id, path: Id) -> BOOL {
    if let Some(path) = NSString::from_ptr(path).and_then(|s| s.as_str()) {
        let window_index = UI.focused_window_index()
            .unwrap_or(UI.open_window::<String, Config>(None, None));
        UI.focus_window(window_index);
        let mut protocol = String::from("file://");
        protocol.push_str(path);
        UI.open_webview::<_, Config>(window_index, Some(protocol), None);
        return YES;
    }
    NO
}

extern fn app_will_finish_launching(this: &mut Object, _cmd: Sel, _note: Id) {
    if let Some(delegate) = AppDelegate::from_ptr(this) {
        NSAppleEventManager::shared_manager().set_get_url_event_handler(&delegate);
    }
}

extern fn app_finished_launching(_: &Object, _cmd: Sel, _note: Id) {
}

extern fn handle_get_url(_: &Object, _cmd: Sel, event: Id, _reply_event: Id) {
    let url = NSAppleEventDescriptor::from_ptr(event)
        .and_then(|event| event.url_param_value())
        .and_then(|url| url.as_str());
    if let Some(window_index) = UI.focused_window_index() {
        UI.open_webview::<_, Config>(window_index, url, None);
    } else {
        UI.open_window::<_, Config>(url, None);
    }
}

extern fn container_accepts_first_responder(_: &Object, _cmd: Sel) -> BOOL {
    YES
}

extern fn container_key_down(this: &mut Object, _cmd: Sel, event: Id) {
    if let Some(event) = NSEvent::from_ptr(event) {
        let flags = event.modifier_flags();
        if flags == 0 || flags == 256 {
            return;
        }
        if let Some(view) = NSView::from_ptr(this).and_then(|v| v.superview()) {
            view.key_down(event);
        }
    }
}

extern fn run_keybinding_command(this: &mut Object, _cmd: Sel) {
    if let Some(key_delegate) = KeyInputDelegate::from_ptr(this) {
        if let Some(command) = key_delegate.command().and_then(|c| c.as_str()) {
            UI.engine.execute_command::<CocoaUI<_>, _>(&UI, UI.focused_window_index(), command);
        }
    }
}

extern fn webview_will_navigate(_: &Object, _cmd: Sel, webview_ptr: Id, action:Id,
                                handler: Id) {
    const PERMITTED_SCHEMES: [&'static str; 5] = ["file","http","https","ftp","about"];
    if let Some(action) = WKNavigationAction::from_ptr(action) {
        if let Some(request) = action.request() {
            let url = request.url();
            let openable_type = action.navigation_type() == WKNavigationType::LinkActivated;
            let cmd_pressed = action.modifier_flags() == NSEventModifierFlags::Command as NSUInteger;
            // Open in a new frame
            let new_frame = (openable_type && cmd_pressed) || action.target_frame().is_none();
            if new_frame {
                let window = NSView::from_ptr(webview_ptr)
                    .and_then(|view| view.window());
                if let (Some(url), Some(window)) = (url.absolute_string().as_str(), window) {
                    run_nav_action_block(handler, WKNavigationActionPolicy::Cancel);
                    UI.engine.on_new_frame_request::<CocoaUI<_>, _>(&UI, window.number() as u32, url);
                    return;
                }
            } else if let Some(scheme) = url.scheme().as_str() {
                // Open in the existing frame
                if PERMITTED_SCHEMES.contains(&scheme) {
                    run_nav_action_block(handler, WKNavigationActionPolicy::Allow);
                    return;
                }
                info!("Unable to open scheme: {}", scheme);
            }
            // Open in the default app
            run_nav_action_block(handler, WKNavigationActionPolicy::Cancel);
            NSWorkspace::shared_workspace().open_url(url);
            return;
        }
    }
    run_nav_action_block(handler, WKNavigationActionPolicy::Cancel);
}

fn run_nav_action_block(handler: Id, policy: WKNavigationActionPolicy) {
    unsafe {
        let ref block = *(handler as *mut _ as *mut Block<(WKNavigationActionPolicy,), ()>);
        block.call((policy,));
    }
}

extern fn webview_will_load(_: &Object, _cmd: Sel, webview_ptr: Id, nav_ptr: Id) {
    register_uri_event(webview_ptr, nav_ptr, BufferEvent::Request);
}

extern fn webview_load_failed(_: &Object, _cmd: Sel, webview_ptr: Id, nav_ptr: Id, error: Id) {
    if let Some(error) = NSError::from_ptr(error) {
        let mut message = String::new();
        if let Some(description) = error.localized_description().and_then(|d| d.as_str()) {
            message.push_str(&description);
        }
        if let Some(reason) = error.localized_failure_reason().and_then(|d| d.as_str()) {
            message.push_str(&format!(" ({})", reason));
        }
        register_uri_event(webview_ptr, nav_ptr, BufferEvent::Fail(message));
    }
}

extern fn webview_did_same_nav(_: &Object, _cmd: Sel, webview_ptr: Id, nav_ptr: Id, _nav_type: Id) {
    register_uri_event(webview_ptr, nav_ptr, BufferEvent::Load);
}

extern fn webview_did_load(_: &Object, _cmd: Sel, webview_ptr: Id, nav_ptr: Id) {
    register_uri_event(webview_ptr, nav_ptr, BufferEvent::Load);
}

extern fn command_bar_did_end_editing(_: &Object, _cmd: Sel, notification: Id) {
    if is_return_key_event(notification) {
        if let Some(text) = notification_object_text(notification) {
            UI.engine.execute_command::<CocoaUI<_>, _>(&UI, UI.focused_window_index(), text);
        }
    }
}

extern fn command_bar_text_changed(_: &Object, _cmd: Sel, notification: Id) {
    if let Some(text) = notification_object_text(notification) {
        if let Some(command) = UI.engine.config.command_matching_prefix(text) {
            UI.engine.execute_command::<CocoaUI<_>, _>(&UI, UI.focused_window_index(), &command);
        }
    }
}

extern fn command_bar_get_completion(_: &Object, _cmd: Sel, control: Id, _: Id, words: Id, _: NSRange, _: Id) -> Id {
    info!("requesting command bar completions");
    let prefix = NSControl::from_ptr(control)
        .and_then(|control| control.text())
        .and_then(|string| string.as_str());
    if let Some(prefix) = prefix {
        let completions = UI.engine.command_completions::<CocoaUI<_>, _>(&UI, prefix);
        NSArray::from_vec(completions, |item| NSString::from(&item)).ptr()
    } else {
        words
    }
}

fn register_uri_event(webview_ptr: Id, nav_ptr: Id, event: BufferEvent) {
    let uri = WKNavigation::from_ptr(nav_ptr)
        .and_then(|u| u.url_string())
        .and_then(|u| u.as_str())
        // FIXME: Workaround for refresh event not including a `request` object
        .or(WKWebView::from_ptr(webview_ptr)
            .and_then(|view| view.url())
            .and_then(|url| url.absolute_string().as_str()));
    if let Some((window_index, webview_index)) = reference_indices(webview_ptr) {
        UI.engine.on_buffer_event::<CocoaUI<_>, _>(&UI, window_index,
                                                   webview_index, uri, event);
    }
}

fn reference_indices(webview: Id) -> Option<(u32, u32)> {
    if let Some(webview) = WKWebView::from_ptr(webview).and_then(|v| v.coerce::<NSView>()) {
        if let Some(window) = webview.window() {
            return Some((window.number() as u32,
                         webview.subview_index().unwrap() as u32));
        }
    }
    None
}

fn notification_object_text<'a>(notification: Id) -> Option<&'a str> {
    return NSNotification::from_ptr(notification)
        .and_then(|note| note.object::<NSControl>())
        .and_then(|control| control.text())
        .and_then(|string| string.as_str());
}

fn is_return_key_event(notification: Id) -> bool {
    const RETURN_KEY_VALUE: NSInteger = 0x10;
    return NSNotification::from_ptr(notification)
        .and_then(|note| note.user_info())
        .and_then(|info| info.get::<NSNumber>("NSTextMovement"))
        .and_then(|value| Some(value.integer_value() == RETURN_KEY_VALUE))
        .unwrap_or(false);
}
