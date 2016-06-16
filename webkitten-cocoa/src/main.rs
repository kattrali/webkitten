#![allow(non_snake_case)]

extern crate cocoa;
#[macro_use]
extern crate objc;
extern crate core_graphics;
extern crate block;
extern crate webkitten;

mod webkit;
mod foundation;
mod ui;

use webkit::*;
use foundation::*;
use cocoa::base::{selector, id, nil, NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
                        NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApp,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSResizableWindowMask,
                    NSMiniaturizableWindowMask, NSClosableWindowMask,
                    NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps, NSView};
use core_graphics::geometry::{CGRect,CGPoint,CGSize};
use block::ConcreteBlock;
use webkitten::ui::ApplicationUI;

fn main() {
    if let Some(mut ui) = webkitten::Engine::run::<ui::CocoaUI>("/Users/delisa/.config/webkitten/config.toml") {
        ui.open_window("http://delisa.me");
        ui.run();
    }
}

fn old_main() {
    unsafe {

        // create Window
        let mask = (NSTitledWindowMask as NSUInteger |
                    NSMiniaturizableWindowMask as NSUInteger |
                    NSResizableWindowMask as NSUInteger |
                    NSClosableWindowMask as NSUInteger) as NSUInteger;
        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            NSRect::new(NSPoint::new(0., 0.), NSSize::new(700., 700.)),
            mask,
            NSBackingStoreBuffered,
            NO
        ).autorelease();
        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.center();
        let store = _WKUserContentExtensionStore::default_store(nil);
        let rules = "[{'trigger':{'url-filter':'^https?://+([^:/]+\\\\.)?google\\\\.com/images/','url-filter-is-case-sensitive':true},'action':{'type': 'block'}}]";
        store.compile_content_extension("sandblocks",
                                        &rules.replace("'", "\""),
                                        ConcreteBlock::new(move |filter: id, err: id| {
            let config = WKWebViewConfiguration::new(nil).autorelease();
            if err == nil {
                config.user_content_controller().add_user_content_filter(filter);
            } else {
                println!("failed to load extension");
            }
            let frame: CGRect = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize { width: 700., height: 700.}
            };
            let webview = WKWebView::alloc(nil).init_frame_configuration(frame, config).autorelease();
            window.contentView().addSubview_(webview);
            webview.load_request(NSURLRequest::with_url(nil, NSURL("https://google.com")));
        }));
        let title = NSString::alloc(nil).init_str("Hello World!");
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);
    }
}
