use std::ops::Deref;

use objc::runtime::{Class,YES,NO,BOOL};
use foundation::{NSString,NSURLRequest,NSURL,NSUInteger};
use core_graphics::CGRect;
use block::Block;

use super::{Id,ObjCClass,nil};


#[link(name = "WebKit", kind = "framework")]
extern {}

pub type ContentExtensionCompletionHandler = Deref<Target=Block<(Id, Id), ()>>;

pub enum WKFindOptions {
    CaseInsensitive = 1 << 0,
    AtWordStarts = 1 << 1,
    TreatMedialCapitalAsWordStart = 1 << 2,
    Backwards = 1 << 3,
    WrapAround = 1 << 4,
    ShowOverlay = 1 << 5,
    ShowFindIndicator = 1 << 6,
    ShowHighlight = 1 << 7
}

pub enum WKNavigationActionPolicy {
    Cancel = 0,
    Allow  = 1,
}

#[derive(PartialEq)]
pub enum WKNavigationType {
    LinkActivated   = 0,
    FormSubmitted   = 1,
    BackForward     = 2,
    Reload          = 3,
    FormResubmitted = 4,
    Other           = -1,
}

impl_objc_class!(WKFrameInfo);
impl_objc_class!(WKNavigation);
impl_objc_class!(WKNavigationAction);
impl_objc_class!(WKPreferences);
impl_objc_class!(WKUserContentController);
impl_objc_class!(WKWebView);
impl_objc_class!(WKWebViewConfiguration);
impl_objc_class!(WKWebsiteDataStore);
impl_objc_class!(_WKUserContentExtensionStore);
impl_objc_class!(_WKUserContentFilter);
impl_objc_class!(_WKUserStyleSheet);

impl WKFrameInfo {

    pub fn is_main_frame(&self) -> bool {
        let is_main: BOOL = unsafe { msg_send![self.ptr, isMainFrame] };
        is_main == YES
    }
}

impl WKNavigation {

    pub fn request(&self) -> Option<NSURLRequest> {
        NSURLRequest::from_ptr(unsafe { msg_send![self.ptr, _request] })
    }

    pub fn url_string(&self) -> Option<NSString> {
        self.request()
            .and_then(|req| Some(req.url().absolute_string()))
    }
}

impl WKNavigationAction {

    pub fn request(&self) -> Option<NSURLRequest> {
        NSURLRequest::from_ptr(unsafe { msg_send![self.ptr, request] })
    }

    pub fn modifier_flags(&self) -> NSUInteger {
        unsafe { msg_send![self.ptr, modifierFlags] }
    }

    pub fn navigation_type(&self) -> WKNavigationType {
        unsafe { msg_send![self.ptr, navigationType] }
    }

    pub fn target_frame(&self) -> Option<WKFrameInfo> {
        WKFrameInfo::from_ptr(unsafe { msg_send![self.ptr, targetFrame] })
    }

    pub fn source_frame(&self) -> Option<WKFrameInfo> {
        WKFrameInfo::from_ptr(unsafe { msg_send![self.ptr, sourceFrame] })
    }
}

impl WKPreferences {

    pub fn set_javascript_enabled(&self, enabled: bool) {
        let value = if enabled { YES } else { NO };
        unsafe { msg_send![self.ptr, setJavaScriptEnabled:value]; }
    }

    pub fn set_plugins_enabled(&self, enabled: bool) {
        let value = if enabled { YES } else { NO };
        unsafe { msg_send![self.ptr, setPlugInsEnabled:value]; }
    }
}

impl WKUserContentController {

    pub fn add_user_content_filter(&self, filter: _WKUserContentFilter) {
        unsafe { msg_send![self.ptr, _addUserContentFilter:filter.ptr()]; }
    }

    pub fn add_user_style_sheet(&self, stylesheet: _WKUserStyleSheet) {
        unsafe { msg_send![self.ptr, _addUserStyleSheet:stylesheet.ptr()]; }
    }

    pub fn can_add_user_style_sheet(&self) -> bool {
        let responds: BOOL = unsafe {
            msg_send![self.ptr, respondsToSelector:sel!(_addUserStyleSheet:)]
        };
        responds == YES
    }
}

impl WKWebView {

    pub fn new(frame: CGRect, config: WKWebViewConfiguration) -> Self {
        let ptr = unsafe {
            let webview: Id = msg_send![class!("WKWebView"), alloc];
            let webview: Id = msg_send![webview, initWithFrame:frame
                                                 configuration:config.ptr()];
            webview
        };
        WKWebView { ptr: ptr }
    }

    pub fn load_request(&self, request: NSURLRequest) {
        unsafe { msg_send![self.ptr, loadRequest:request.ptr()]; }
    }

    pub fn set_history_delegate<T: ObjCClass>(&self, delegate: T) {
        unsafe { msg_send![self.ptr, _setHistoryDelegate:delegate.ptr()]; }
    }

    pub fn set_navigation_delegate<T: ObjCClass>(&self, delegate: T) {
        unsafe { msg_send![self.ptr, setNavigationDelegate:delegate.ptr()]; }
    }

    pub fn configuration(&self) -> WKWebViewConfiguration {
        WKWebViewConfiguration {
            ptr: unsafe { msg_send![self.ptr, configuration] }
        }
    }

    pub fn can_go_back(&self) -> bool {
        let can: BOOL = unsafe { msg_send![self.ptr, canGoBack] };
        can == YES
    }

    pub fn can_go_forward(&self) -> bool {
        let can: BOOL = unsafe { msg_send![self.ptr, canGoForward] };
        can == YES
    }

    pub fn go_back(&self) {
        unsafe { msg_send![self.ptr, goBack]; }
    }

    pub fn go_forward(&self) {
        unsafe { msg_send![self.ptr, goForward]; }
    }

    pub fn reload(&self) {
        unsafe { msg_send![self.ptr, reload:nil]; }
    }

    pub fn reload_without_content_blockers(&self) {
        if self.can_reload_without_content_blockers() {
            unsafe { msg_send![self.ptr, _reloadWithoutContentBlockers]; }
        }
    }

    pub fn can_reload_without_content_blockers(&self) -> bool {
        let responds: BOOL = unsafe {
            let selector = sel!(_reloadWithoutContentBlockers);
            msg_send![self.ptr, respondsToSelector:selector]
        };
        responds == YES
    }

    pub fn stop_loading(&self) {
        unsafe { msg_send![self.ptr, stopLoading]; }
    }

    pub fn has_only_secure_content(&self) -> bool {
        let has: BOOL = unsafe { msg_send![self.ptr, hasOnlySecureContent] };
        has == YES
    }

    pub fn load_html_string(&self, contents: &str, base_url: &str) {
        unsafe {
            msg_send![self.ptr, loadHTMLString:NSString::from(contents)
                                       baseURL:NSURL::from(NSString::from(base_url))];
        }
    }

    pub fn is_loading(&self) -> bool {
        let loading: BOOL = unsafe { msg_send![self.ptr, isLoading] };
        loading == YES
    }

    pub fn url(&self) -> Option<NSURL> {
        NSURL::from_ptr(unsafe { msg_send![self.ptr, URL] })
    }

    pub fn title(&self) -> Option<NSString> {
        NSString::from_ptr(unsafe { msg_send![self.ptr, title] })
    }

    pub fn set_custom_user_agent(&self, user_agent: &str) {
        unsafe {
            msg_send![self.ptr, setCustomUserAgent:NSString::from(user_agent)];
        }
    }

    pub fn custom_user_agent(&self) -> Option<NSString> {
        NSString::from_ptr(unsafe { msg_send![self.ptr, customUserAgent] })
    }

    pub fn evaluate_javascript(&self, script: &str) {
        unsafe {
            msg_send![self.ptr, evaluateJavaScript:NSString::from(script)
                                 completionHandler:nil];
        }
    }

    pub fn find_string(&self, query: &str) {
        let options: NSUInteger = WKFindOptions::CaseInsensitive as NSUInteger |
                                  WKFindOptions::WrapAround as NSUInteger |
                                  WKFindOptions::ShowFindIndicator as NSUInteger |
                                  WKFindOptions::TreatMedialCapitalAsWordStart as NSUInteger |
                                  WKFindOptions::ShowHighlight as NSUInteger;
        unsafe {
            msg_send![self.ptr, _findString:NSString::from(query)
                                    options:options
                                   maxCount:100 as NSUInteger];
        }
    }

    pub fn hide_find_results(&self) {
        unsafe { msg_send![self.ptr, _hideFindUI]; }
    }
}

impl WKWebViewConfiguration {

    pub fn new() -> Self {
        WKWebViewConfiguration {
            ptr: unsafe { msg_send![class!("WKWebViewConfiguration"), new] }
        }
    }

    pub fn preferences(&self) -> WKPreferences {
        WKPreferences { ptr: unsafe { msg_send![self.ptr, preferences] } }
    }

    pub fn user_content_controller(&self) -> WKUserContentController {
        WKUserContentController {
            ptr: unsafe { msg_send![self.ptr, userContentController] }
        }
    }

    pub fn website_data_store(&self) -> WKWebsiteDataStore {
        WKWebsiteDataStore {
            ptr: unsafe { msg_send![self.ptr, websiteDataStore] }
        }
    }

    pub fn set_website_data_store(&self, store: WKWebsiteDataStore) {
        unsafe { msg_send![self.ptr, setWebsiteDataStore:store.ptr()] }
    }
}

impl WKWebsiteDataStore {

    pub fn default_store() -> Self {
        WKWebsiteDataStore {
            ptr: unsafe {
                msg_send![class!("WKWebsiteDataStore"), defaultDataStore]
            }
        }
    }

    pub fn nonpersistent_store() -> Self {
        WKWebsiteDataStore {
            ptr: unsafe {
                msg_send![class!("WKWebsiteDataStore"), nonPersistentDataStore]
            }
        }
    }
}

impl _WKUserContentExtensionStore {

    pub fn default_store() -> Self {
        _WKUserContentExtensionStore {
            ptr: unsafe {
                msg_send![class!("_WKUserContentExtensionStore"), defaultStore]
            }
        }
    }

    pub fn compile_content_extension(&self,
                                     identifier: &str,
                                     extension: &str,
                                     block: &ContentExtensionCompletionHandler) {
        let id_str = NSString::from(identifier);
        let ex_str = NSString::from(extension);
        unsafe {
            msg_send![self.ptr, compileContentExtensionForIdentifier:id_str
                                             encodedContentExtension:ex_str
                                                   completionHandler:block.deref()];
        }
    }

    pub fn lookup_content_extension(&self,
                                    identifier: &str,
                                    block: &ContentExtensionCompletionHandler) {
        let id_str = NSString::from(identifier);
        unsafe {
            msg_send![self.ptr, lookupContentExtensionForIdentifier:id_str
                                                  completionHandler:block.deref()];
        }
    }
}

impl _WKUserStyleSheet {

    pub fn new(styles: &str) -> Self {
        let source = NSString::from(styles);
        let ptr = unsafe {
            let sheet: Id = msg_send![class!("_WKUserStyleSheet"), alloc];
            let sheet: Id = msg_send![sheet, initWithSource:source
                                           forMainFrameOnly:YES];
            sheet
        };
        _WKUserStyleSheet { ptr: ptr }
    }
}
