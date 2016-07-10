use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use cocoa::base::{id,nil,class};
use webkitten::ui::{ApplicationUI,EventHandler};
use cocoa_ext::foundation::*;
use cocoa_ext::appkit::NSControl;
use ui::{CocoaUI,UI};

const CBDELEGATE_CLASS: &'static str = "CommandBarDelegate";

pub struct CommandBarDelegate {}

impl CommandBarDelegate {
    pub unsafe fn new() -> id { msg_send![class(CBDELEGATE_CLASS), new] }
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

pub fn declare_bar_delegates() {
    if let Some(superclass) = Class::get("NSObject") {
        if let Some(mut decl) = ClassDecl::new(CBDELEGATE_CLASS, superclass) {
            unsafe {
                decl.add_method(sel!(controlTextDidEndEditing:),
                    command_bar_did_end_editing as extern fn(&Object, Sel, id));
                decl.add_method(sel!(control:textView:completions:forPartialWordRange:indexOfSelectedItem:),
                    command_bar_get_completion as extern fn(&Object, Sel, id, id, id, NSRange, id) -> id);
            }

            decl.register();
        }
    }
}

extern fn command_bar_did_end_editing(_: &Object, _cmd: Sel, notification: id) {
    if let Some(text) = notification_object_text(notification) {
        UI.engine.execute_command::<CocoaUI>(&UI, UI.focused_window_index(), text);
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
    if is_return_key_event(notification) {
        unsafe {
            let control = notification.object();
            return control.string_value().as_str();
        };
    }
    None
}

fn is_return_key_event(notification: id) -> bool {
    let keycode = unsafe {
        notification.user_info().object_for_key("NSTextMovement").integer_value()
    };
    keycode == 0x10
}
