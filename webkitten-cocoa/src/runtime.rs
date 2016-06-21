use std::{slice,str};
use libc::c_char;

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use cocoa::base::{selector,id,nil,class};
use webkitten::ui::EventHandler;
use cocoa::foundation::NSString;

use cocoa_ext::foundation::{NSDictionary,NSNotification,NSNumber};
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
        let bytes: *const c_char = nsstring.UTF8String();
        let byte_str = bytes as *const u8;
        let len = nsstring.len();
        slice::from_raw_parts(byte_str, len)
    };
    str::from_utf8(bytes).ok()
}

pub fn declare_bar_delegates() {
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

fn is_return_key_event(notification: id) -> bool {
    let keycode = unsafe {
        notification.user_info().object_for_key("NSTextMovement").integer_value()
    };
    keycode == 0x10
}
