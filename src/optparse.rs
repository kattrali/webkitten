use std::{env,fs};
use std::io::Write;
use std::path::Path;
use getopts::Options;
use super::config;

pub struct RunConfiguration {
    pub path: String,
    pub start_pages: Vec<String>,
}

pub fn parse_opts(default_config_path: &str) -> Option<RunConfiguration> {
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
        return None;
    }
    let path = matches.opt_str("c").unwrap_or(String::from(default_config_path));
    validate_config_path(&path);
    Some(RunConfiguration { path: path, start_pages: matches.free })
}

/// Print command usage, given invocation path and options
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

/// Check if the given config path exists, creating if not required to already
/// exist
fn validate_config_path(config_path: &str) {
    if let Some(parent) = Path::new(config_path).parent() {
        let metadata = fs::metadata(parent);
        if metadata.is_err() || !metadata.unwrap().is_dir() {
            write_config_dir(parent);
        }
    }
    if !fs::metadata(config_path).is_ok() {
        write_default_config(config_path);
    }
}

fn write_config_dir(path: &Path) {
    match fs::create_dir_all(path) {
        Ok(()) => (),
        Err(e) => panic!("Unable to create config dir ({}): {}", path.display(), e)
    }
}

fn write_default_config(config_path: &str) {
    match fs::File::create(config_path) {
        Ok(ref mut file) => {
            if file.write(config::DEFAULT_CONFIG.as_bytes()).is_ok() {
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

