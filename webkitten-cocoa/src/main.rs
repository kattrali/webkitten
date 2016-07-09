#![allow(non_snake_case)]

extern crate cocoa;
extern crate core_graphics;
extern crate block;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate objc;
extern crate url;
extern crate webkitten;

mod webkit;
mod cocoa_ext;
mod ui;
mod runtime;

use webkitten::ui::ApplicationUI;
use webkitten::Engine;


fn main() {
    env_logger::init().unwrap();
    runtime::declare_bar_delegates();
    ui::UI.run();
}
