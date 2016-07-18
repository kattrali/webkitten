use cocoa::base::{selector,id,nil,YES};
use cocoa::foundation::{NSAutoreleasePool, NSProcessInfo};
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicyRegular,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps,
                    NSEventModifierFlags};
use webkitten::ui::BrowserConfiguration;
use cocoa_ext::foundation::{NSString,NSUInteger};
use runtime::KeyInputDelegate;

pub fn initialize_app_env() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        nsapp().setActivationPolicy_(NSApplicationActivationPolicyRegular);
        create_menu();
    }
}

pub fn start_run_loop() {
    unsafe {
        msg_send![nsapp(), activateIgnoringOtherApps:YES];
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
    app_menu_item.setSubmenu_(create_app_menu());
    let edit_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(edit_menu_item);
    edit_menu_item.setSubmenu_(create_edit_menu());
    let cmd_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(cmd_menu_item);
    cmd_menu_item.setSubmenu_(create_command_menu());
    let window_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(window_menu_item);
    window_menu_item.setSubmenu_(create_window_menu());
}

unsafe fn create_window_menu() -> id {
    let menu = NSMenu::alloc(nil).initWithTitle_(<id as NSString>::from_str("Window")).autorelease();
    msg_send![nsapp(), setWindowsMenu:menu];
    menu
}

unsafe fn create_app_menu() -> id {
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
    app_menu
}

unsafe fn create_edit_menu() -> id {
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
    edit_menu
}

unsafe fn create_command_menu() -> id {
    let cmd_menu = NSMenu::alloc(nil).initWithTitle_(<id as NSString>::from_str("Command")).autorelease();
    for (command_name, (keychar, modifier)) in super::UI.engine.config.command_keybindings() {
        let mut key = String::new();
        key.push(keychar);
        let mask = NSEventModifierFlags::from_bits(modifier as NSUInteger).unwrap();
        let item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            <id as NSString>::from_str(&command_name),
            sel!(runKeybindingCommand),
            <id as NSString>::from_str(&key)).autorelease();
        item.setKeyEquivalentModifierMask_(mask);
        msg_send![item, setTarget:KeyInputDelegate::new(&command_name)];
        cmd_menu.addItem_(item);
    }
    cmd_menu
}

pub unsafe fn nsapp() -> id {
    NSApplication::sharedApplication(nil)
}
