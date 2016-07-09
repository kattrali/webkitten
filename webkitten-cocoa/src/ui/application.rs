use cocoa::base::{selector,id,nil};
use cocoa::foundation::{NSAutoreleasePool, NSProcessInfo};
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicyRegular,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps};
use cocoa_ext::foundation::NSString;

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
    let quit_prefix = <id as NSString>::from_str("Quit ");
    let quit_title = quit_prefix.append(NSProcessInfo::processInfo(nil).processName());
    let quit_action = selector("terminate:");
    let quit_key = <id as NSString>::from_str("q");
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
