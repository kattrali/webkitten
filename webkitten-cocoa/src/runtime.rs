use std::{slice,str};
use libc::c_char;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use cocoa::base::{id,nil,class,BOOL,YES};
use webkitten::ui::EventHandler;
use cocoa_ext::foundation::*;
use cocoa_ext::appkit::NSControl;
use ui::CocoaUI;

const ABDELEGATE_CLASS: &'static str = "AddressBarDelegate";
const CBDELEGATE_CLASS: &'static str = "CommandBarDelegate";

pub struct AddressBarDelegate {}
pub struct CommandBarDelegate {}

impl AddressBarDelegate {
    pub unsafe fn new() -> id { msg_send![class(ABDELEGATE_CLASS), new] }
}

impl CommandBarDelegate {
    pub unsafe fn new() -> id { msg_send![class(CBDELEGATE_CLASS), new] }
}

pub fn nsstring_as_str<'a>(nsstring: id) -> Option<&'a str> {
    let bytes = unsafe {
        let bytes: *const c_char = nsstring.utf8();
        let byte_str = bytes as *const u8;
        let len = nsstring.len();
        slice::from_raw_parts(byte_str, len)
    };
    str::from_utf8(bytes).ok()
}

pub fn log_error_description(err: id) {
    if err != nil {
        unsafe {
            let desc = msg_send![err, description];
            if let Some(desc) = nsstring_as_str(desc) {
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
        if let Some(mut decl) = ClassDecl::new(ABDELEGATE_CLASS, superclass) {
            unsafe {
                decl.add_method(sel!(controlTextDidEndEditing:),
                    address_bar_did_end_editing as extern fn(&Object, Sel, id));
                decl.add_method(sel!(control:textView:completions:forPartialWordRange:indexOfSelectedItem:),
                    address_bar_get_completion as extern fn(&Object, Sel, id, id, id, NSRange, id) -> id);
            }

            decl.register();
        }
    }
}

extern fn command_bar_did_end_editing(_: &Object, _cmd: Sel, notification: id) {
    if let Some(text) = notification_object_text(notification) {
        super::UI.engine.execute_command::<CocoaUI>(&super::UI, 0, 0, text);
    }
}

extern fn address_bar_get_completion(_: &Object, _cmd: Sel, control: id, _: id, words: id, range: NSRange, index: id) -> id {
    info!("requesting address bar completions",);
    unsafe {
        if let Some(prefix) = nsstring_as_str(control.string_value()) {
            let completions = super::UI.engine.address_completions::<CocoaUI>(&super::UI, prefix);
            <id as NSArray>::from_vec(completions, |item| <id as NSString>::from_str(&item))
        } else {
            words
        }
    }
}

extern fn command_bar_get_completion(_: &Object, _cmd: Sel, control: id, _: id, words: id, range: NSRange, index: id) -> id {
    info!("requesting command bar completions");
    unsafe {
        if let Some(prefix) = nsstring_as_str(control.string_value()) {
            let completions = super::UI.engine.command_completions::<CocoaUI>(&super::UI, prefix);
            <id as NSArray>::from_vec(completions, |item| <id as NSString>::from_str(&item))
        } else {
            words
        }
    }
}

extern fn address_bar_did_end_editing(_: &Object, _cmd: Sel, notification: id) {
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

fn is_return_key_event(notification: id) -> bool {
    let keycode = unsafe {
        notification.user_info().object_for_key("NSTextMovement").integer_value()
    };
    keycode == 0x10
}
