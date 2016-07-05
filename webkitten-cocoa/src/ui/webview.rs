use cocoa::base::{id,nil,YES,NO};
use cocoa_ext::foundation::{NSURLRequest};

use webkit::*;
use runtime;


pub fn load_uri(webview: id, uri: &str) {
    unsafe { webview.load_request(NSURLRequest(uri)); }
}

pub fn load_html_string(webview: id, contents: &str) {
    unsafe { webview.load_html_string(contents, ""); }
}

pub fn is_loading(webview: id) -> bool {
    unsafe { webview.is_loading() == YES }
}

fn set_custom_user_agent(webview: id, user_agent: &str) {
    unsafe { webview.set_custom_user_agent(user_agent) }
}

fn custom_user_agent(webview: id) -> String {
    unsafe {
        let user_agent = webview.custom_user_agent();
        if user_agent != nil {
            if let Some(user_agent) = runtime::nsstring_as_str(user_agent) {
                return String::from(user_agent);
            }
        }
        return String::new();
    }
}

pub fn go_back(webview: id) -> bool {
    unsafe {
        if webview.can_go_back() == NO {
            return false
        }
        webview.go_back();
    }
    true
}

pub fn go_forward(webview: id) -> bool {
    unsafe {
        if webview.can_go_forward() == NO {
            return false
        }
        webview.go_forward();
    }
    true
}

pub fn reload(webview: id) {
    unsafe { webview.reload(); }
}

pub fn stop_loading(webview: id) {
    unsafe { webview.stop_loading(); }
}

pub fn uri(webview: id) -> String {
    unsafe {
        let url = webview.url();
        if url != nil {
            let url: id = msg_send![url, absoluteString];
            if let Some(url) = runtime::nsstring_as_str(url) {
                return String::from(url);
            }
        }
        return String::new();
    }
}

pub fn title(webview: id) -> String {
    unsafe {
        let title = webview.title();
        if title != nil {
            if let Some(title) = runtime::nsstring_as_str(title) {
                return String::from(title);
            }
        }
        return String::new();
    }
}

pub fn run_javascript(webview: id, script: &str) {
    unsafe { webview.evaluate_javascript(script); }
}

pub fn apply_styles(webview: id, styles: &str) {
    unsafe {
        let sheet = <id as _WKUserStyleSheet>::init_source(styles);
        webview.configuration().user_content_controller().add_user_style_sheet(sheet);
    }
}
