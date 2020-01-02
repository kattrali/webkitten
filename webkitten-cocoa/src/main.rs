#![allow(non_snake_case)]

extern crate block;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate macos;
#[macro_use]
extern crate objc;
extern crate webkitten;
extern crate dirs;

mod ui;
mod runtime;

use webkitten::ui::ApplicationUI;

static SIMPLE_LOGGER: SimpleLogger = SimpleLogger;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{}:{}: {}", record.level(),
                     record.module_path().unwrap(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() {
    log::set_logger(&SIMPLE_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
    runtime::declare_classes();
    ui::UI.run();
}
