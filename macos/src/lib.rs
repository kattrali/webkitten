#[macro_use]
extern crate objc;
extern crate block;
extern crate libc;

use objc::runtime::Object;


pub type Id = *mut Object;
#[allow(non_upper_case_globals)]
pub const nil: Id = 0 as Id;

pub trait ObjCClass: Sized {

    fn ptr(&self) -> Id;

    fn from_ptr(ptr: Id) -> Option<Self>;

    fn class_name() -> &'static str;

    fn nil() -> Self;

    fn ptr_is_class(ptr: Id) -> bool;

    // FIXME: replace with TryInto trait when stabilized
    fn coerce<T: ObjCClass>(&self) -> Option<T> {
        T::from_ptr(self.ptr())
    }

    fn autorelease(&self) -> Self;

    fn release(&mut self);
}

/// Shorthand for getting a class by name
#[macro_export]
macro_rules! class {
    ($name: expr) => (Class::get($name).unwrap());
}

/// Implements the basics of `NSObject`
#[macro_export]
macro_rules! impl_objc_class {
    ($name: ident) => (
        #[derive(Debug)]
        pub struct $name { ptr: Id }

        impl PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                let eq: BOOL = unsafe {
                    msg_send![self.ptr, isEqual:other.ptr]
                };
                eq == YES
            }
        }

        impl ObjCClass for $name {

            fn ptr(&self) -> Id { self.ptr }

            fn from_ptr(ptr: Id) -> Option<Self> {
                match $name::ptr_is_class(ptr) {
                    true => Some($name { ptr: ptr }),
                    false => None
                }
            }

            fn ptr_is_class(ptr: Id) -> bool {
                let eq: BOOL = unsafe {
                    msg_send![ptr, isKindOfClass:class!($name::class_name())]
                };
                if eq != YES && ptr != 0 as Id {
                    let actual_class = NSString::from_ptr(unsafe {
                        msg_send![ptr, className]
                    }).and_then(|name| name.as_str()).unwrap_or("<unknown>");
                    println!("ERROR! Failed type coercion from {} to {}",
                             actual_class,
                             $name::class_name());
                }
                eq == YES
            }

            fn nil() -> Self { $name { ptr: 0 as Id } }

            fn class_name() -> &'static str { stringify!($name) }

            fn autorelease(&self) -> Self {
                $name { ptr: unsafe { msg_send![self.ptr, autorelease] } }
            }

            fn release(&mut self) {
                unsafe { msg_send![self.ptr(), release]; }
                self.ptr = 0 as Id;
            }
        }
    );
}

pub mod appkit;
pub mod core_foundation;
pub mod core_graphics;
pub mod core_services;
pub mod foundation;
pub mod webkit;

