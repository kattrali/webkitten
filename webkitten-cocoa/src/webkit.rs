//! Bindings to WebKit.framework on macOS

use std::ops::Deref;
use cocoa::base::{class,id,nil,BOOL,NO};
use core_graphics::geometry::CGRect;
use cocoa_ext::foundation::NSString;
use block::Block;

#[link(name = "WebKit", kind = "framework")]
extern {}

pub trait WKWebViewConfiguration {

    unsafe fn new() -> id;
    unsafe fn user_content_controller(self) -> id;
    unsafe fn website_data_store(self) -> id;
    unsafe fn set_website_data_store(self, store: id);
}

impl WKWebViewConfiguration for id {

    unsafe fn new() -> id {
        msg_send![class("WKWebViewConfiguration"), new]
    }

    unsafe fn user_content_controller(self) -> id {
        msg_send![self, userContentController]
    }

    unsafe fn website_data_store(self) -> id {
        msg_send![self, websiteDataStore]
    }

    unsafe fn set_website_data_store(self, store: id) {
        msg_send![self, setWebsiteDataStore: store];
    }
}

pub trait WKWebView {

    unsafe fn alloc(_: Self) -> id {
        msg_send![class("WKWebView"), alloc]
    }

    unsafe fn load_request(self, request: id /* NSURLRequest */);
    unsafe fn configuration(self) -> id;
    unsafe fn go_back(self);
    unsafe fn can_go_back(self) -> BOOL;
    unsafe fn go_forward(self);
    unsafe fn can_go_forward(self) -> BOOL;
    unsafe fn reload(self);
    unsafe fn stop_loading(self);
    unsafe fn has_only_secure_content(self) -> BOOL;
    unsafe fn load_html_string(self, contents: &str, base_url: &str);
    unsafe fn is_loading(self) -> BOOL;
    unsafe fn url(self) -> id;
    unsafe fn title(self) -> id;
    unsafe fn set_custom_user_agent(self, user_agent: &str);
    unsafe fn custom_user_agent(self) -> id;
    unsafe fn evaluate_javascript(self, script: &str);
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

    unsafe fn can_go_back(self) -> BOOL {
        msg_send![self, canGoBack]
    }

    unsafe fn can_go_forward(self) -> BOOL {
        msg_send![self, canGoForward]
    }

    unsafe fn go_back(self) {
        msg_send![self, goBack];
    }

    unsafe fn go_forward(self) {
        msg_send![self, goForward];
    }

    unsafe fn reload(self) {
        msg_send![self, reload:nil];
    }

    unsafe fn stop_loading(self) {
        msg_send![self, stopLoading];
    }

    unsafe fn has_only_secure_content(self) -> BOOL {
        msg_send![self, hasOnlySecureContent]
    }

    unsafe fn load_html_string(self, contents: &str, base_url: &str) {
        let contents_str = <id as NSString>::from_str(contents);
        let url_str = <id as NSString>::from_str(base_url);
        msg_send![self, loadHTMLString:contents_str baseURL:url_str];
    }

    unsafe fn is_loading(self) -> BOOL {
        msg_send![self, isLoading]
    }

    unsafe fn url(self) -> id {
        msg_send![self, URL]
    }

    unsafe fn title(self) -> id {
        msg_send![self, title]
    }

    unsafe fn set_custom_user_agent(self, user_agent: &str) {
        let ua_str = <id as NSString>::from_str(user_agent);
        msg_send![self, setCustomUserAgent:ua_str];
    }

    unsafe fn custom_user_agent(self) -> id {
        msg_send![self, customUserAgent]
    }

    unsafe fn evaluate_javascript(self, script: &str) {
        let script_str = <id as NSString>::from_str(script);
        msg_send![self, evaluateJavaScript:script_str completionHandler:nil];
    }
}

pub trait WKUserContentController {

    unsafe fn add_user_content_filter(self, filter: id /* _WKUserContentFilter */);
    unsafe fn add_user_style_sheet(self, stylsheet: id /* _WKUserStyleSheet */);
}

impl WKUserContentController for id {

    unsafe fn add_user_content_filter(self, filter: id) {
        msg_send![self, _addUserContentFilter:filter];
    }

    unsafe fn add_user_style_sheet(self, stylesheet: id) {
        msg_send![self, _addUserStyleSheet:stylesheet];
    }
}

pub trait _WKUserStyleSheet {

    unsafe fn init_source(styles: &str) -> id {
        let source = <id as NSString>::from_str(styles);
        let sheet: id = msg_send![class("_WKUserStyleSheet"), alloc];
        let sheet: id = msg_send![sheet, initWithSource:source mainFrameOnly:NO];
        sheet
    }
}

impl _WKUserStyleSheet for id {}

pub type ContentExtensionCompletionHandler = Deref<Target=Block<(id, id), ()>>;

pub trait _WKUserContentExtensionStore {

    unsafe fn default_store(_:Self) -> id {
        msg_send![class("_WKUserContentExtensionStore"), defaultStore]
    }

    unsafe fn compile_content_extension(self,
                                        identifier: &str,
                                        extension: &str,
                                        block: &ContentExtensionCompletionHandler);

    unsafe fn lookup_content_extension(self,
                                       identifier: &str,
                                       block: &ContentExtensionCompletionHandler);
}

impl _WKUserContentExtensionStore for id {

    unsafe fn compile_content_extension(self,
                                        identifier: &str,
                                        extension: &str,
                                        block: &ContentExtensionCompletionHandler) {
        let id_str = <id as NSString>::from_str(identifier);
        let ex_str = <id as NSString>::from_str(extension);
        msg_send![self, compileContentExtensionForIdentifier:id_str
                                     encodedContentExtension:ex_str
                                           completionHandler:block.deref()];
    }

    unsafe fn lookup_content_extension(self,
                                       identifier: &str,
                                       block: &ContentExtensionCompletionHandler) {
        let id_str = <id as NSString>::from_str(identifier);
        msg_send![self, lookupContentExtensionForIdentifier:id_str
                                          completionHandler:block.deref()];
    }
}

pub trait WKWebsiteDataStore {

    unsafe fn default_store() -> id;
    unsafe fn nonpersistent_store() -> id;
}

impl WKWebsiteDataStore for id {

    unsafe fn default_store() -> id {
        msg_send![class("WKWebsiteDataStore"), defaultDataStore]
    }

    unsafe fn nonpersistent_store() -> id {
        msg_send![class("WKWebsiteDataStore"), nonPersistentDataStore]
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
