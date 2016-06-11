#![allow(non_snake_case)]

extern crate cocoa;
#[macro_use]
extern crate objc;
extern crate core_graphics;
extern crate block;

mod webkit;
mod foundation;

use webkit::*;
use foundation::*;
use cocoa::base::{selector, id, nil, NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
                        NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApp,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps, NSView};
use core_graphics::geometry::{CGRect,CGPoint,CGSize};
use block::ConcreteBlock;

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit");
        let quit_title = quit_prefix.stringByAppendingString_(
            NSProcessInfo::processInfo(nil).processName()
        );
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            quit_title,
            quit_action,
            quit_key
        ).autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // create Window
        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            NSRect::new(NSPoint::new(0., 0.), NSSize::new(700., 700.)),
            NSTitledWindowMask as NSUInteger,
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
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }
}
