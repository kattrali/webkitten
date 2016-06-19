#![allow(non_snake_case)]

extern crate cocoa;
extern crate core_graphics;
extern crate block;
#[macro_use]
extern crate objc;
extern crate webkitten;

mod webkit;
mod cocoa_ext;
mod ui;

use webkitten::ui::ApplicationUI;
use std::env;

fn main() {
    if let Some(home_dir) = env::home_dir() {
        let default_config_path = &format!("{}/.config/webkitten/config.toml", home_dir.display());
        if let Some(run_config) = webkitten::optparse::parse_opts(default_config_path) {
            if let Some(mut ui) = webkitten::Engine::run::<ui::CocoaUI>(&run_config.path) {
                ui.run();
            }
        }
    } else {
        panic!("Unable to locate home directory");
    }
}
