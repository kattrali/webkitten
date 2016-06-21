use std::cell::RefCell;

use cocoa::base::{id};
use cocoa_ext::foundation::{NSURLRequest};

use webkitten::ui::WebView;
use webkit::*;

pub struct CocoaWebView {
    wkwebview: RefCell<id>
}

impl WebView for CocoaWebView {

    fn load_uri(&self, uri: &str) {
        let webview = self.wkwebview.borrow();
        unsafe { webview.load_request(NSURLRequest(uri)); }
    }

    fn go_back(&self) {
    }

    fn go_forward(&self) {
    }

    fn focus(&self) {
    }

    fn raw_html(&self) -> String {
        String::new()
    }

    fn uri(&self) -> String {
        String::new()
    }

    fn title(&self) -> String {
        String::new()
    }

    fn apply_javascript(&self, script: &str) {
    }

    fn apply_styles(&self, styles: &str) {
    }

    fn apply_content_filters(&self, identifier: &str, rules: &str) {
        unsafe {
            //let store = _WKUserContentExtensionStore::default_store(nil);
            //store.compile_content_extension(identifier,
                                            //rules,
                                            //ConcreteBlock::new(move |filter: id, err: id| {
                //let mut webview = self.wkwebview.borrow_mut();
                //if err == nil {
                    //webview.configuration().user_content_controller().add_user_content_filter(filter);
                //} else {
                    //println!("failed to load extension");
                //}
            //}));
        }
    }
}
