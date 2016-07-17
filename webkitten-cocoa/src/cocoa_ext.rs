pub mod foundation {
    use objc::{Encode,Encoding};
    use libc;
    use cocoa::base::{class,id};
    use std::{str,slice};

    const UTF8_ENCODING: usize = 4;

    #[cfg(target_pointer_width = "32")]
    pub type NSInteger = libc::c_int;
    #[cfg(target_pointer_width = "32")]
    pub type NSUInteger = libc::c_uint;

    #[cfg(target_pointer_width = "64")]
    pub type NSInteger = libc::c_long;
    #[cfg(target_pointer_width = "64")]
    pub type NSUInteger = libc::c_ulong;

    pub struct NSRange {
        location: NSUInteger,
        length: NSUInteger,
    }

    unsafe impl Encode for NSRange {
        fn encode() -> Encoding {
            let encoding = format!("{{NSRange={}{}}}",
                                   NSUInteger::encode().as_str(),
                                   NSUInteger::encode().as_str());
            unsafe { Encoding::from_str(&encoding) }
        }
    }

    pub unsafe fn NSURL(url: &str) -> id {
        let url_str: id = <id as NSString>::from_str(url);
        msg_send![class("NSURL"), URLWithString:url_str]
    }

    pub trait NSURL {

        unsafe fn absolute_string(self) -> id;
    }

    impl NSURL for id {

        unsafe fn absolute_string(self) -> id {
            msg_send![self, absoluteString]
        }
    }

    pub unsafe fn NSURLRequest(url: &str) -> id {
        msg_send![class("NSURLRequest"), requestWithURL:NSURL(url)]
    }

    pub trait NSURLRequest {

        unsafe fn url(self) -> id;
    }

    impl NSURLRequest for id {

        unsafe fn url(self) -> id {
            msg_send![self, URL]
        }
    }

    pub trait NSNumber {

        unsafe fn integer_value(self) -> NSInteger;
    }

    impl NSNumber for id {

        unsafe fn integer_value(self) -> NSInteger {
            msg_send![self, integerValue]
        }
    }

    pub trait NSString {

        unsafe fn from_str(content: &str) -> id {
            let string: id = msg_send![class("NSString"), alloc];
            msg_send![string, initWithBytes:content.as_ptr()
                                     length:content.len()
                                   encoding:UTF8_ENCODING as id]
        }

        unsafe fn append(self, other: id) -> id;
        unsafe fn utf8(self) -> *const libc::c_char;
        unsafe fn len(self) -> usize;
        unsafe fn as_str<'a>(self) -> Option<&'a str>;
    }

    impl NSString for id {

        unsafe fn append(self, other: id) -> id {
            msg_send![self, stringByAppendingString:other]
        }

        unsafe fn utf8(self) -> *const libc::c_char {
            msg_send![self, UTF8String]
        }

        unsafe fn len(self) -> usize {
            msg_send![self, lengthOfBytesUsingEncoding:UTF8_ENCODING]
        }

        unsafe fn as_str<'a>(self) -> Option<&'a str> {
            let bytes = {
                let bytes = self.utf8();
                let byte_str = bytes as *const u8;
                slice::from_raw_parts(byte_str, self.len())
            };
            str::from_utf8(bytes).ok()
        }
    }

    pub trait NSDictionary {

        unsafe fn object_for_key(self, key: &str) -> id;
    }

    impl NSDictionary for id {

        unsafe fn object_for_key(self, key: &str) -> id {
            let key_str = <id as NSString>::from_str(key);
            msg_send![self, objectForKey:key_str]
        }
    }

    pub trait NSArray {

        unsafe fn from_vec<T, F>(items: Vec<T>, transform: F) -> id
            where F: Fn(&T) -> id {
            let array: id = msg_send![class("NSMutableArray"), new];
            for item in items {
                array.add_object(transform(&item));
            }
            array
        }

        unsafe fn object_at_index(self, index: NSUInteger) -> id;
        unsafe fn index_of_object(self, object: id) -> NSUInteger;
        unsafe fn get(self, index: NSUInteger) -> Option<id>;
        unsafe fn count(self) -> NSUInteger;
    }

    impl NSArray for id {

        unsafe fn object_at_index(self, index: NSUInteger) -> id {
            msg_send![self, objectAtIndex:index]
        }

        unsafe fn index_of_object(self, object: id) -> NSUInteger {
            msg_send![self, indexOfObject:object]
        }

        unsafe fn get(self, index: NSUInteger) -> Option<id> {
            if self.count() > index {
                Some(self.object_at_index(index))
            } else {
                None
            }
        }

        unsafe fn count(self) -> NSUInteger {
            msg_send![self, count]
        }
    }

    pub trait NSMutableArray {

        unsafe fn add_object(self, object: id);
    }

    impl NSMutableArray for id {
        unsafe fn add_object(self, object: id) {
            msg_send![self, addObject:object];
        }
    }

    pub trait NSNotification {

        unsafe fn object(self) -> id;
        unsafe fn user_info(self) -> id;
    }

    impl NSNotification for id {

        unsafe fn object(self) -> id {
            msg_send![self, object]
        }

        unsafe fn user_info(self) -> id {
            msg_send![self, userInfo]
        }
    }
}

pub mod core_graphics {
    use core_graphics::geometry::{CGRect,CGPoint,CGSize};

    pub fn CGRectZero() -> CGRect {
        CGRect {
            origin: CGPoint { x: 0., y: 0. },
            size: CGSize { width: 0., height: 0. }
        }
    }
}

pub mod appkit {
    use cocoa::base::{class,id,nil,NO,YES,BOOL};
    use core_graphics::base::CGFloat;
    use super::foundation::{NSString,NSMutableArray};

    const NSPASTEBOARD_TYPE_STRING: &'static str = "public.utf8-plain-text";

    pub trait NSPasteboard {

        unsafe fn general_pasteboard() -> id {
            msg_send![class("NSPasteboard"), generalPasteboard]
        }

        unsafe fn write_objects(self, objects: id);
        unsafe fn copy(self, text: &str);
    }

    impl NSPasteboard for id {

        unsafe fn write_objects(self, objects: id) {
            msg_send![self, writeObjects:objects];
        }

        unsafe fn copy(self, text: &str) {
            let raw = <id as NSString>::from_str(text);
            let data_type = <id as NSString>::from_str(NSPASTEBOARD_TYPE_STRING);
            let types: id = msg_send![class("NSMutableArray"), new];
            types.add_object(data_type);
            msg_send![self, declareTypes:types owner:nil];
            msg_send![self, setString:raw forType:data_type];
        }
    }

    pub enum NSLayoutAttribute {
        Left          = 1,
        Right         = 2,
        Top           = 3,
        Bottom        = 4,
        Leading       = 5,
        Trailing      = 6,
        Width         = 7,
        Height        = 8,
        CenterX       = 9,
        CenterY       = 10,
        Baseline      = 11,
        // LastBaseline  = 11,
        FirstBaseline = 12,
        NotAnAttribute = 0
    }

    pub enum NSLayoutRelation {
        LessThanOrEqual   = -1,
        Equal              = 0,
        GreaterThanOrEqual = 1,
    }

    pub trait NSLayoutConstraint {

        unsafe fn new(view1: id, attr1: NSLayoutAttribute, relation: NSLayoutRelation,
                      view2: id, attr2: NSLayoutAttribute, multiplier: CGFloat,
                      constant: CGFloat) -> id {
            msg_send![class("NSLayoutConstraint"), constraintWithItem:view1
                                                            attribute:attr1
                                                            relatedBy:relation
                                                               toItem:view2
                                                            attribute:attr2
                                                           multiplier:multiplier
                                                             constant:constant]
        }

        unsafe fn bind(view1: id, attr1: NSLayoutAttribute, view2: id, attr2: NSLayoutAttribute) -> id {
            <id as NSLayoutConstraint>::new(view1, attr1, NSLayoutRelation::Equal, view2, attr2, 1 as CGFloat, 0 as CGFloat)
        }
    }

    impl NSLayoutConstraint for id {}

    pub trait NSControl {

        unsafe fn string_value(self) -> id;
        unsafe fn set_string_value(self, value: &str);
        unsafe fn set_font(self, family: &str, size: i64);
    }

    impl NSControl for id {

        unsafe fn string_value(self) -> id {
            msg_send![self, stringValue]
        }

        unsafe fn set_string_value(self, value: &str) {
            let value_str: id = <id as NSString>::from_str(value);
            msg_send![self, setStringValue:value_str];
        }

        unsafe fn set_font(self, family: &str, size: i64) {
            let name = <id as NSString>::from_str(family);
            let font: id = msg_send![class("NSFont"), fontWithName:name
                                                              size:size as CGFloat];
            if font != nil {
                msg_send![self, setFont:font];
            } else {
                warn!("Unable to find font on system: {}", family);
            }
        }
    }

    pub trait NSConstraintBasedLayoutInstallingConstraints {

        unsafe fn add_constraint(self, constraint: id);
        unsafe fn set_height(self, height: CGFloat);
    }

    impl NSConstraintBasedLayoutInstallingConstraints for id {

        unsafe fn add_constraint(self, constraint: id) {
            msg_send![self, addConstraint:constraint];
        }

        unsafe fn set_height(self, height: CGFloat) {
            self.add_constraint(<id as NSLayoutConstraint>::new(self,
                NSLayoutAttribute::Height, NSLayoutRelation::Equal, nil,
                NSLayoutAttribute::NotAnAttribute, 1 as CGFloat, height))
        }
    }

    pub trait NSView {

        unsafe fn new() -> id {
            msg_send![class("NSView"), new]
        }

        unsafe fn subviews(self) -> id;
        unsafe fn add_subview(self, view: id);
        unsafe fn set_hidden(self, hidden: bool);
        unsafe fn hidden(self) -> BOOL;
        unsafe fn remove_from_superview(self);
    }

    impl NSView for id {

        unsafe fn subviews(self) -> id {
            msg_send![self, subviews]
        }

        unsafe fn add_subview(self, view: id) {
            msg_send![self, addSubview:view];
        }

        unsafe fn set_hidden(self, hidden: bool) {
            let value = if hidden { YES } else { NO };
            msg_send![self, setHidden:value];
        }

        unsafe fn hidden(self) -> BOOL {
            msg_send![self, isHidden]
        }

        unsafe fn remove_from_superview(self) {
            msg_send![self, removeFromSuperview];
        }
    }

    pub trait NSTextField {

        unsafe fn new() -> id;
        unsafe fn disable_translates_autoresizing_mask_into_constraints(self);
        unsafe fn set_delegate(self, delegate: id);
    }

    impl NSTextField for id {

        unsafe fn new() -> id {
            msg_send![class("NSTextField"), new]
        }

        unsafe fn disable_translates_autoresizing_mask_into_constraints(self) {
            msg_send![self, setTranslatesAutoresizingMaskIntoConstraints:NO];
        }

        unsafe fn set_delegate(self, delegate: id) {
            msg_send![self, setDelegate:delegate];
        }
    }
}

