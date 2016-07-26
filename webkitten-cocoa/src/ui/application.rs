use macos::ObjCClass;
use macos::foundation::{NSAutoreleasePool,NSUInteger};
use macos::appkit::{NSApplicationActivationPolicy,NSMenu,NSMenuItem,nsapp};
use webkitten::ui::BrowserConfiguration;

use runtime::{KeyInputDelegate,AppDelegate};


pub fn initialize_app_env() -> AppDelegate {
    let _pool = NSAutoreleasePool::new();
    nsapp().set_activation_policy(NSApplicationActivationPolicy::Regular);
    let delegate = AppDelegate::new();
    create_menu(&delegate);
    delegate
}

pub fn start_run_loop(delegate: &AppDelegate) {
    let app = nsapp();
    app.activate(true);
    app.set_delegate(delegate);
    app.run();
}

fn create_menu(delegate: &AppDelegate) {
    let menubar = NSMenu::new("").autorelease();
    let app_menu_item = NSMenuItem::blank().autorelease();
    app_menu_item.set_submenu(create_app_menu(delegate));
    let edit_menu_item = NSMenuItem::blank().autorelease();
    edit_menu_item.set_submenu(create_edit_menu());
    let cmd_menu_item = NSMenuItem::blank().autorelease();
    cmd_menu_item.set_submenu(create_command_menu());
    let window_menu_item = NSMenuItem::blank().autorelease();
    window_menu_item.set_submenu(create_window_menu());
    menubar.add_item(app_menu_item);
    menubar.add_item(edit_menu_item);
    menubar.add_item(cmd_menu_item);
    menubar.add_item(window_menu_item);
    nsapp().set_main_menu(menubar);
}

fn create_window_menu() -> NSMenu {
    let menu = NSMenu::new("Window").autorelease();
    nsapp().set_windows_menu(&menu);
    menu
}

fn create_app_menu(delegate: &AppDelegate) -> NSMenu {
    let app_menu = NSMenu::new("").autorelease();
    app_menu.set_autoenables_items(false);
    let default_item = NSMenuItem::new("Set as default Web Browser", sel!(setAsDefaultBrowser), "").autorelease();
    default_item.set_target(delegate);
    app_menu.add_item(default_item);
    let quit_item = NSMenuItem::new("Quit", sel!(terminate:), "q").autorelease();
    app_menu.add_item(quit_item);
    app_menu
}

fn create_edit_menu() -> NSMenu {
    let edit_menu = NSMenu::new("Edit").autorelease();
    edit_menu.add_item(NSMenuItem::new("Undo", sel!(undo:), "z").autorelease());
    edit_menu.add_item(NSMenuItem::new("Redo", sel!(redo:), "Z").autorelease());
    edit_menu.add_item(NSMenuItem::separator());
    edit_menu.add_item(NSMenuItem::new("Cut", sel!(cut:), "x").autorelease());
    edit_menu.add_item(NSMenuItem::new("Copy", sel!(copy:), "c").autorelease());
    edit_menu.add_item(NSMenuItem::new("Paste", sel!(paste:), "v").autorelease());
    edit_menu.add_item(NSMenuItem::separator());
    edit_menu.add_item(NSMenuItem::new("Select All", sel!(selectAll:), "a").autorelease());
    edit_menu
}

fn create_command_menu() -> NSMenu {
    let cmd_menu = NSMenu::new("Command").autorelease();
    for (command_name, (keychar, modifier)) in super::UI.engine.config.command_keybindings() {
        let mut key = String::new();
        key.push(keychar);
        let item = NSMenuItem::new(&command_name, sel!(runKeybindingCommand),&key).autorelease();
        item.set_key_equivalent_modifier_mask(modifier as NSUInteger);
        item.set_target(&KeyInputDelegate::new(&command_name));
        cmd_menu.add_item(item);
    }
    cmd_menu
}
