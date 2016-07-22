#![allow(non_snake_case)]

extern crate cocoa;
extern crate core_foundation_sys;
extern crate core_foundation;
extern crate core_graphics;
extern crate block;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate objc;
extern crate webkitten;

mod webkit;
mod cocoa_ext;
mod ui;
mod runtime;

use log::{LogRecord, LogLevel, LogMetadata,LogLevelFilter};
use webkitten::ui::ApplicationUI;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{}:{}: {}", record.level(),
                     record.location().module_path(), record.args());
        }
    }
}

fn main() {
	log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger)
    });
    runtime::declare_delegate_classes();
    ui::UI.run();
}
