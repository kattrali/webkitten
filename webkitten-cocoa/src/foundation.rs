use cocoa::base::{class,id,nil};
use cocoa::foundation::NSString;

pub unsafe fn NSURL(url: &str) -> id {
    let url_str = NSString::alloc(nil).init_str(url);
    msg_send![class("NSURL"), URLWithString:url_str]
}

pub trait NSURLRequest {

    unsafe fn with_url(_:Self, url: id /* NSURL */) -> id {
        msg_send![class("NSURLRequest"), requestWithURL:url]
    }
}

impl NSURLRequest for id {
}
