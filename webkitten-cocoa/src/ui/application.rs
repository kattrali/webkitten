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

pub fn windows() -> id {
    unsafe { msg_send![nsapp(), windows] }
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
    let edit_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(edit_menu_item);
    let edit_menu = NSMenu::alloc(nil).initWithTitle_(<id as NSString>::from_str("Edit")).autorelease();
    edit_menu.addItem_(NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        <id as NSString>::from_str("Undo"),
        selector("undo:"),
        <id as NSString>::from_str("z")).autorelease());
    edit_menu.addItem_(NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        <id as NSString>::from_str("Redo"),
        selector("redo:"),
        <id as NSString>::from_str("Z")).autorelease());
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        <id as NSString>::from_str("Cut"),
        selector("cut:"),
        <id as NSString>::from_str("x")).autorelease());
    edit_menu.addItem_(NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        <id as NSString>::from_str("Copy"),
        selector("copy:"),
        <id as NSString>::from_str("c")).autorelease());
    edit_menu.addItem_(NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        <id as NSString>::from_str("Paste"),
        selector("paste:"),
        <id as NSString>::from_str("v")).autorelease());
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        <id as NSString>::from_str("Select All"),
        selector("selectAll:"),
        <id as NSString>::from_str("a")).autorelease());
    edit_menu_item.setSubmenu_(edit_menu);
}

pub unsafe fn nsapp() -> id {
    NSApplication::sharedApplication(nil)
}
