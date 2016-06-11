use cocoa::base::{class,id};
use core_graphics::geometry::CGRect;

pub trait WKWebViewConfiguration {

    unsafe fn new(_:Self) -> id {
        msg_send![class("WKWebViewConfiguration"), new]
    }

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

    unsafe fn init_frame_configuration(self,
                                       frame: CGRect,
                                       config: id /* WKWebViewConfiguration */) -> Self;
    unsafe fn load_request(self, request: id /* NSURLRequest */);
}

impl WKWebView for id {

    unsafe fn init_frame_configuration(self,
                                       frame: CGRect,
                                       config: id) -> id {
        msg_send![self, initWithFrame:frame
                        configuration:config]
    }

    unsafe fn load_request(self, request: id) {
        msg_send![self, loadRequest:request];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cocoa::base::{id,nil};
    use core_graphics::geometry::{CGRect,CGPoint,CGSize};

    #[test]
    pub fn test_config() {
        unsafe {
            let _config: id = WKWebViewConfiguration::new(nil);
            _config.user_content_controller();
        }
    }

    #[test]
    pub fn test_webview() {
        unsafe {
            let _config: id = WKWebViewConfiguration::new(nil);
            let _frame: CGRect = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize { width: 200.0, height: 400.0 }
            };
            WKWebView::alloc(nil).init_frame_configuration(_frame, _config);
        }
    }
}
