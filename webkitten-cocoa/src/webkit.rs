use cocoa::base::{class,id,nil};
use core_graphics::geometry::CGRect;
use cocoa::foundation::NSString;
use block::{ConcreteBlock,IntoConcreteBlock};

#[link(name = "WebKit", kind = "framework")]
extern {}

pub unsafe fn WKWebViewConfiguration() -> id {
    msg_send![class("WKWebViewConfiguration"), new]
}

pub trait WKWebViewConfiguration {

    unsafe fn user_content_controller(self) -> id;
}

impl WKWebViewConfiguration for id {
    unsafe fn user_content_controller(self) -> id {
        msg_send![self, userContentController]
    }
}

pub trait WKWebView {

    unsafe fn alloc(_: Self) -> id {
        msg_send![class("WKWebView"), alloc]
    }

    unsafe fn load_request(self, request: id /* NSURLRequest */);
    unsafe fn configuration(self) -> id;
}

pub unsafe fn WKWebView(frame: CGRect, config: id) -> id {
    let webview = WKWebView::alloc(nil);
    msg_send![webview, initWithFrame:frame configuration:config]
}
impl WKWebView for id {

    unsafe fn load_request(self, request: id) {
        msg_send![self, loadRequest:request];
    }

    unsafe fn configuration(self) -> id {
        msg_send![self, configuration]
    }
}

pub trait WKUserContentController {

    unsafe fn add_user_content_filter(self, filter: id /* _WKUserContentFilter */);
}

impl WKUserContentController for id {

    unsafe fn add_user_content_filter(self, filter: id) {
        msg_send![self, _addUserContentFilter:filter];
    }
}

pub trait _WKUserContentExtensionStore {

    unsafe fn default_store(_:Self) -> id {
        msg_send![class("_WKUserContentExtensionStore"), defaultStore]
    }

    unsafe fn compile_content_extension<F>(self,
                                        identifier: &str,
                                        extension: &str,
                                        block: ConcreteBlock<(id /* _WKUserContentFilter */, id /* NSError */), (), F>)
        where F: IntoConcreteBlock<(id, id), Ret=()> + 'static;
}

impl _WKUserContentExtensionStore for id {

    unsafe fn compile_content_extension<F>(self,
                                        identifier: &str,
                                        extension: &str,
                                        block: ConcreteBlock<(id, id), (), F>)
        where F: IntoConcreteBlock<(id, id), Ret=()> + 'static {
        let id_str = NSString::alloc(nil).init_str(identifier);
        let ex_str = NSString::alloc(nil).init_str(extension);
        let block = block.copy();
        msg_send![self, compileContentExtensionForIdentifier:id_str
                                     encodedContentExtension:ex_str
                                           completionHandler:block];

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cocoa::base::{id,nil};
    use core_graphics::geometry::{CGRect,CGPoint,CGSize};

    #[test]
    pub fn test_content_filter() {
        unsafe {
            assert!(_WKUserContentExtensionStore::default_store(nil) != nil);
        }
    }

    #[test]
    pub fn test_config() {
        unsafe {
            let config: id = WKWebViewConfiguration::new(nil);
            assert!(config.user_content_controller() != nil);
        }
    }

    #[test]
    pub fn test_webview() {
        unsafe {
            let config: id = WKWebViewConfiguration::new(nil);
            let frame: CGRect = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize { width: 200.0, height: 400.0 }
            };
            let webview = WKWebView::alloc(nil).init_frame_configuration(frame, config);
            assert!(webview != nil);
        }
    }
}
