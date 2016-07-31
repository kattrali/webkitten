use std::{str,slice};
use std::cmp::PartialEq;

use objc::{Encode,Encoding};
use objc::runtime::{Object,Class,YES,BOOL};
use libc;

use super::{Id,ObjCClass};
use core_graphics::{CGRect,CGSize,CGPoint};


#[link(name = "Foundation", kind = "framework")]
extern {}

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

pub type NSRect  = CGRect;
pub type NSSize  = CGSize;
pub type NSPoint = CGPoint;

const UTF8_ENCODING: NSUInteger = 4;

impl_objc_class!(NSAppleEventDescriptor);
impl_objc_class!(NSAppleEventManager);
impl_objc_class!(NSArray);
impl_objc_class!(NSAutoreleasePool);
impl_objc_class!(NSDictionary);
impl_objc_class!(NSError);
impl_objc_class!(NSMutableArray);
impl_objc_class!(NSNotification);
impl_objc_class!(NSNumber);
impl_objc_class!(NSString);
impl_objc_class!(NSURL);
impl_objc_class!(NSURLRequest);

impl NSAppleEventDescriptor {

    pub fn url_param_value(&self) -> Option<NSString> {
        const URL_KEYWORD: u32 = 757935405;
        NSString::from_ptr(unsafe {
            let descriptor: Id = msg_send![self.ptr, paramDescriptorForKeyword:URL_KEYWORD];
            msg_send![descriptor, stringValue]
        })
    }
}

impl NSAppleEventManager {

    pub fn shared_manager() -> Self {
        NSAppleEventManager {
            ptr: unsafe {
                msg_send![class!("NSAppleEventManager"), sharedAppleEventManager]
            }
        }
    }

    pub fn set_get_url_event_handler<T: ObjCClass>(&self, handler: &T) {
        const INTERNET_EVENT_CLASS: u32 = 1196773964;
        const GET_URL_EVENT_ID: u32 = 1196773964;
        unsafe {
            msg_send![self.ptr, setEventHandler:handler.ptr()
                                    andSelector:sel!(handleGetURLEvent:withReplyEvent:)
                                  forEventClass:INTERNET_EVENT_CLASS
                                     andEventID:GET_URL_EVENT_ID];
        }
    }
}

impl NSArray {

    /// Create an array from a vector
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::{NSArray,NSString};
    ///
    /// let items = vec!["abc","def","ghi"];
    /// let array = NSArray::from_vec(items, |item| NSString::from(item));
    /// assert_eq!(3, array.count());
    /// ```
    pub fn from_vec<T, O, F>(items: Vec<T>, transform: F) -> Self
        where F: Fn(&T) -> O,
              O: ObjCClass {
        let array = NSMutableArray::new();
        for item in items {
            array.push(transform(&item));
        }
        array.copy()
    }

    /// Create an array from a vector
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::{NSArray,NSString};
    ///
    /// let items = vec!["abc","def","ghi"];
    /// let array = NSArray::from_vec(items, |item| NSString::from(item));
    /// assert_eq!("def", array.get::<NSString>(1).unwrap().as_str().unwrap());
    /// ```
    pub fn get<T: ObjCClass>(&self, index: NSUInteger) ->Option<T> {
        if self.count() > index {
            T::from_ptr(unsafe { msg_send![self.ptr, objectAtIndex:index] })
        } else {
            None
        }
    }

    pub fn index_of<T: ObjCClass>(&self, object: &T) -> NSUInteger {
        unsafe { msg_send![self.ptr, indexOfObject:object.ptr()] }
    }

    pub fn count(&self) -> NSUInteger {
        unsafe { msg_send![self.ptr, count] }
    }

    pub fn as_mut(&self) -> NSMutableArray {
        NSMutableArray { ptr: unsafe { msg_send![self.ptr, mutableCopy] }}
    }
}

impl NSAutoreleasePool {

    pub fn new() -> Self {
        NSAutoreleasePool {
            ptr: unsafe { msg_send![class!("NSAutoreleasePool"), new] }
        }
    }

    pub fn drain(&self) {
        unsafe { msg_send![self.ptr, drain] }
    }
}

impl NSMutableArray {

    /// Create empty `NSMutableArray`
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::NSMutableArray;
    ///
    /// let array = NSMutableArray::new();
    /// assert_eq!(0, array.count());
    /// ```
    pub fn new() -> Self {
        NSMutableArray { ptr: unsafe { msg_send![class!("NSMutableArray"), new] }}
    }

    pub fn count(&self) -> NSUInteger {
        unsafe { msg_send![self.ptr, count] }
    }

    /// Add an object to the array
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::{NSMutableArray,NSString};
    ///
    /// let array = NSMutableArray::new();
    /// array.push(NSString::from("hello"));
    /// assert_eq!(1, array.count());
    /// ```
    pub fn push<T: ObjCClass>(&self, object: T) {
        unsafe { msg_send![self.ptr, addObject:object.ptr()] }
    }

    pub fn get<T: ObjCClass>(&self, index: NSUInteger) ->Option<T> {
        if self.count() > index {
            T::from_ptr(unsafe { msg_send![self.ptr, objectAtIndex:index] })
        } else {
            None
        }
    }

    pub fn insert<T: ObjCClass>(&self, index: NSUInteger, object: T) {
        unsafe { msg_send![self.ptr, insertObject:object.ptr() atIndex:index] }
    }

    pub fn remove_last_object(&self) {
        unsafe { msg_send![self.ptr, removeLastObject] }
    }

    pub fn copy(&self) -> NSArray {
        let ptr = unsafe { msg_send![self.ptr, copy] };
        NSArray { ptr: ptr }
    }
}

impl NSDictionary {

    /// Creates a reference to an object in a dictionary if the specified type
    /// matches the actual type
    pub fn get<T: ObjCClass>(&self, key: &str)-> Option<T> {
        T::from_ptr(unsafe { self.raw_object_for_key(key) })
    }

    unsafe fn raw_object_for_key(&self, key: &str) -> Id {
        msg_send![self.ptr, objectForKey:NSString::from(key).ptr]
    }
}

impl NSError {

    pub fn description(&self) -> Option<NSString> {
        NSString::from_ptr(unsafe { msg_send![self.ptr, description] })
    }
}

impl NSNotification {

    pub fn object<T: ObjCClass>(&self) -> Option<T> {
        T::from_ptr(unsafe { msg_send![self.ptr, object] })
    }

    pub fn user_info(&self) -> Option<NSDictionary> {
        NSDictionary::from_ptr(unsafe { msg_send![self.ptr, userInfo] })
    }
}

impl NSNumber {

    pub fn integer_value(&self) -> NSInteger {
        unsafe { msg_send![self.ptr, integerValue] }
    }
}

impl NSString {

    /// Create a new empty `NSString`
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::NSString;
    ///
    /// let string = NSString::new();
    /// assert_eq!(0, string.len());
    /// ```
    pub fn new() -> Self {
        NSString { ptr: unsafe { msg_send![class!("NSString"), new] } }
    }

    /// Creates an `NSString` from a `str`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::NSString;
    ///
    /// let string = NSString::from("hello");
    /// assert_eq!("hello", string.as_str().unwrap());
    /// ```
    pub fn from(content: &str) -> Self {
        let ptr: *mut Object = unsafe {
            let string: *mut Object = msg_send![class!("NSString"), alloc];
            msg_send![string, initWithBytes:content.as_ptr()
                                     length:content.len()
                                   encoding:UTF8_ENCODING]
        };
        NSString { ptr: ptr }
    }

    /// Append one `NSString` to another
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::NSString;
    ///
    /// assert_eq!("hi", NSString::from("hi").as_str().unwrap());
    /// ```
    pub fn push_str(&mut self, other: NSString) {
        let ptr: *mut Object = unsafe {
            msg_send![self.ptr, stringByAppendingString:other.ptr]
        };
        self.ptr = ptr;
    }

    /// Coerce a `NSString` into a `str`
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::NSString;
    ///
    /// let mut content = NSString::from("h");
    /// content.push_str(NSString::from("i"));
    /// assert_eq!(NSString::from("hi"), content);
    /// ```
    pub fn as_str<'a>(&self) -> Option<&'a str> {
        let bytes = unsafe {
            let byte_str = self.utf8() as *const u8;
            slice::from_raw_parts(byte_str, self.len())
        };
        str::from_utf8(bytes).ok()
    }

    /// A null-terminated UTF-8 representation
    pub fn utf8(&self) -> *const libc::c_char {
        unsafe { msg_send![self.ptr, UTF8String] }
    }

    /// The length of the string as measured in UTF-8 code points
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::NSString;
    ///
    /// assert_eq!(0, NSString::new().len());
    /// assert_eq!(5, NSString::from("hello").len());
    /// ```
    pub fn len(&self) -> usize {
        unsafe { msg_send![self.ptr, lengthOfBytesUsingEncoding:UTF8_ENCODING] }
    }
}

#[repr(C)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

unsafe impl Encode for NSRange {
    fn encode() -> Encoding {
        let encoding = format!("{{NSRange={}{}}}",
                               NSUInteger::encode().as_str(),
                               NSUInteger::encode().as_str());
        unsafe { Encoding::from_str(&encoding) }
    }
}

impl NSURL {

    /// Create a new `NSURL` from an `NSString`
    ///
    /// ## Examples
    ///
    /// ```
    /// use macos::foundation::{NSString,NSURL};
    ///
    /// let url = NSURL::from(NSString::from("/some/path"));
    /// assert_eq!("/some/path", url.absolute_string().as_str().unwrap());
    /// ```
    pub fn from(string: NSString) -> Self {
        let ptr: *mut Object = unsafe {
            msg_send![class!("NSURL"), URLWithString:string.ptr]
        };
        NSURL { ptr: ptr }
    }

    pub fn absolute_string(&self) -> NSString {
        let ptr: *mut Object = unsafe { msg_send![self.ptr, absoluteString] };
        NSString { ptr: ptr }
    }
}

impl NSURLRequest {

    pub fn from(url: NSURL) -> Self {
        let ptr: *mut Object = unsafe {
            msg_send![class!("NSURLRequest"), requestWithURL:url.ptr]
        };
        NSURLRequest { ptr: ptr }
    }

    pub fn url(&self) -> NSURL {
        NSURL { ptr: unsafe { msg_send![self.ptr, URL] } }
    }
}

#[test]
fn nsstring_eq() {
    assert_eq!(NSString::from("hello"), NSString::from("hello"));
    assert_eq!(NSString::new(), NSString::new());
    assert!(NSString::from("bat") != NSString::from("bot"));
}
