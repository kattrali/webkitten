extern crate webkitten;
extern crate getopts;

use std::{env,fs};
use std::io::Write;
use getopts::Options;
use webkitten::Application;

/// Print command usage, given invocation path and options
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

/// Check if the given config path exists, creating if not required to already
/// exist
fn validate_config_path(config_path: String, require_exists: bool) {
    if !fs::metadata(config_path.as_str()).is_ok() {
        if require_exists {
            panic!("No config found at path: {}", config_path);
        }
        write_default_config(config_path);
    }
}

fn write_default_config(config_path: String) {
    match fs::File::create(config_path.as_str()) {
        Ok(ref mut file) => {
            if file.write(webkitten::config::DEFAULT_CONFIG.as_bytes()).is_ok() {
                let result = file.flush();
                if result.is_err() {
                    panic!("Unable to create default config ({}): {}",
                           config_path,
                           result.err().unwrap());
                }
            }
        },
        Err(e) => panic!("Unable to create default config ({}): {}", config_path, e)
    }
}

/// Load a new instance of `webkitten::Application` with a given config path
fn load_app(config_path: String, require_exists: bool) {
    validate_config_path(config_path.clone(), require_exists);
    match Application::new(config_path.as_str()) {
        Some(ref mut app) => app.run(),
        None => panic!("Unable to parse config from path: {}", config_path)
    }
}

/// Computes default configuration path
fn default_config_path() -> String {
    match env::var("HOME") {
        Ok(home) => format!("{}/.config/webkitten/config.toml", home),
        Err(_) => panic!("Unable to load default config from HOME")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let program = args[0].clone();
    opts.optopt("c", "config", "Set the configuration path", "PATH");
    opts.optflag("h", "help", "Print this help text");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    match matches.opt_str("c") {
		Some(config_path) => load_app(config_path, true),
        None => load_app(default_config_path(), false)
    }
}

