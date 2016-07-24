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
    let menubar = NSMenu::new("");
    let app_menu_item = NSMenuItem::blank();
    app_menu_item.set_submenu(create_app_menu(delegate));
    let edit_menu_item = NSMenuItem::blank();
    edit_menu_item.set_submenu(create_edit_menu());
    let cmd_menu_item = NSMenuItem::blank();
    cmd_menu_item.set_submenu(create_command_menu());
    let window_menu_item = NSMenuItem::blank();
    window_menu_item.set_submenu(create_window_menu());
    menubar.add_item(app_menu_item);
    menubar.add_item(edit_menu_item);
    menubar.add_item(cmd_menu_item);
    menubar.add_item(window_menu_item);
    nsapp().set_main_menu(menubar);
}

fn create_window_menu() -> NSMenu {
    let menu = NSMenu::new("Window");
    nsapp().set_windows_menu(&menu);
    menu
}

fn create_app_menu(delegate: &AppDelegate) -> NSMenu {
    let app_menu = NSMenu::new("");
    app_menu.set_autoenables_items(false);
    let default_item = NSMenuItem::new("Set as default Web Browser", sel!(setAsDefaultBrowser), "");
    default_item.set_target(delegate);
    app_menu.add_item(default_item);
    let quit_item = NSMenuItem::new("Quit", sel!(terminate:), "q");
    app_menu.add_item(quit_item);
    app_menu
}

fn create_edit_menu() -> NSMenu {
    let edit_menu = NSMenu::new("Edit");
    edit_menu.add_item(NSMenuItem::new("Undo", sel!(undo:), "z"));
    edit_menu.add_item(NSMenuItem::new("Redo", sel!(redo:), "Z"));
    edit_menu.add_item(NSMenuItem::separator());
    edit_menu.add_item(NSMenuItem::new("Cut", sel!(cut:), "x"));
    edit_menu.add_item(NSMenuItem::new("Copy", sel!(copy:), "c"));
    edit_menu.add_item(NSMenuItem::new("Paste", sel!(paste:), "v"));
    edit_menu.add_item(NSMenuItem::separator());
    edit_menu.add_item(NSMenuItem::new("Select All", sel!(selectAll:), "a"));
    edit_menu
}

fn create_command_menu() -> NSMenu {
    let cmd_menu = NSMenu::new("Command");
    for (command_name, (keychar, modifier)) in super::UI.engine.config.command_keybindings() {
        let mut key = String::new();
        key.push(keychar);
        let item = NSMenuItem::new(&command_name, sel!(runKeybindingCommand),&key);
        item.set_key_equivalent_modifier_mask(modifier as NSUInteger);
        item.set_target(&KeyInputDelegate::new(&command_name));
        cmd_menu.add_item(item);
    }
    cmd_menu
}
