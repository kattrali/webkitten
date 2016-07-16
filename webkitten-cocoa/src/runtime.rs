use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use cocoa::base::{id,nil,class};
use webkitten::ui::{ApplicationUI,EventHandler,BrowserConfiguration};
use cocoa_ext::foundation::*;
use cocoa_ext::appkit::NSControl;
use ui::{CocoaUI,UI,window};
use webkit::WKNavigation;

const CBDELEGATE_CLASS: &'static str = "CommandBarDelegate";
const WVHDELEGATE_CLASS: &'static str = "WebViewHistoryDelegate";

pub struct CommandBarDelegate;
pub struct WebViewHistoryDelegate;

impl CommandBarDelegate {
    pub unsafe fn new() -> id { msg_send![class(CBDELEGATE_CLASS), new] }
}

impl WebViewHistoryDelegate {
    pub unsafe fn new() -> id { msg_send![class(WVHDELEGATE_CLASS), new] }
}

pub fn log_error_description(err: id) {
    if err != nil {
        unsafe {
            let desc: id = msg_send![err, description];
            if let Some(desc) = desc.as_str() {
                error!("{}", desc);
            }
        }
    }
}

pub fn declare_delegate_classes() {
    if let Some(superclass) = Class::get("NSObject") {
        declare_bar_delegate(&superclass);
        declare_webview_delegates(&superclass);
    }
}

fn declare_bar_delegate(superclass: &Class) {
    if let Some(mut decl) = ClassDecl::new(CBDELEGATE_CLASS, superclass) {
        unsafe {
            decl.add_method(sel!(controlTextDidChange:),
                command_bar_text_changed as extern fn(&Object, Sel, id));
            decl.add_method(sel!(controlTextDidEndEditing:),
                command_bar_did_end_editing as extern fn(&Object, Sel, id));
            decl.add_method(sel!(control:textView:completions:forPartialWordRange:indexOfSelectedItem:),
                command_bar_get_completion as extern fn(&Object, Sel, id, id, id, NSRange, id) -> id);
        }

        decl.register();
    }
}

fn declare_webview_delegates(superclass: &Class) {
    if let Some(mut decl) = ClassDecl::new(WVHDELEGATE_CLASS, superclass) {
        unsafe {
            decl.add_method(sel!(_webView:navigationDidFinishDocumentLoad:),
                webview_did_navigate as extern fn (&Object, Sel, id, id));
        }
        decl.register();
    }
}

extern fn webview_did_navigate(_: &Object, _cmd: Sel, webview: id, navigation: id) {
    if let Some((window_index, webview_index)) = window::reference_indices(webview) {
        let uri = unsafe { navigation.request().url().absolute_string().as_str() };
        if let Some(uri) = uri {
            UI.engine.on_load_uri::<CocoaUI>(&UI, window_index, webview_index, uri);
        }
    }
}

extern fn command_bar_did_end_editing(_: &Object, _cmd: Sel, notification: id) {
    if is_return_key_event(notification) {
        if let Some(text) = notification_object_text(notification) {
            UI.engine.execute_command::<CocoaUI>(&UI, UI.focused_window_index(), text);
        }
    }
}

extern fn command_bar_text_changed(_: &Object, _cmd: Sel, notification: id) {
    if let Some(text) = notification_object_text(notification) {
        if let Some(command) = UI.engine.config.command_matching_prefix(text) {
            UI.engine.execute_command::<CocoaUI>(&UI, UI.focused_window_index(), &command);
        }
    }
}

extern fn command_bar_get_completion(_: &Object, _cmd: Sel, control: id, _: id, words: id, _: NSRange, _: id) -> id {
    info!("requesting command bar completions");
    unsafe {
        if let Some(prefix) = control.string_value().as_str() {
            let completions = UI.engine.command_completions::<CocoaUI>(&UI, prefix);
            <id as NSArray>::from_vec(completions, |item| <id as NSString>::from_str(&item))
        } else {
            words
        }
    }
}

fn notification_object_text<'a>(notification: id) -> Option<&'a str> {
    unsafe {
        let control = notification.object();
        return control.string_value().as_str();
    };
}

fn is_return_key_event(notification: id) -> bool {
    let keycode = unsafe {
        notification.user_info().object_for_key("NSTextMovement").integer_value()
    };
    keycode == 0x10
}
