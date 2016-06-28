use cocoa::base::{selector,id,nil};
use cocoa::foundation::{NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicyRegular,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps};

pub fn start_run_loop() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        nsapp().setActivationPolicy_(NSApplicationActivationPolicyRegular);
        create_menu();
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        nsapp().run();
    }
}

unsafe fn create_menu() {
    let menubar = NSMenu::new(nil).autorelease();
    let app_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(app_menu_item);
    nsapp().setMainMenu_(menubar);
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
}

pub unsafe fn nsapp() -> id {
    NSApplication::sharedApplication(nil)
}
