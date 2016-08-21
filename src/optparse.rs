use std::{env,fs};
use std::io::Write;
use std::path::Path;
use getopts::Options;
use super::config;

/// The runtime configuration of an instance of a webkitten application
pub struct RunConfiguration {
    /// The configuration file path
    pub path: String,
    /// Pages to open on initial load
    pub start_pages: Vec<String>,
    /// The exit status, set if the application has completed execution
    pub exit_status: Option<(i32, String)>
}

pub fn parse_opts(default_config_path: &str) -> RunConfiguration {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let mut exit_status: Option<(i32, String)> = None;
    let program = args[0].clone();
    opts.optopt("c", "config", "Set the configuration path", "PATH");
    opts.optflag("h", "help", "Print this help text");
    match opts.parse(&args[1..]) {
        Ok(matches) => {
            let path = matches.opt_str("c")
                .unwrap_or(String::from(default_config_path));
            validate_config_path(&path);
            if matches.opt_present("h") {
                exit_status = Some((0, usage(program, opts)));
            }
            RunConfiguration {
                path: path,
                start_pages: matches.free,
                exit_status: exit_status
            }
        },
        Err(err) => {
            let message = format!("{}\n{}", err, usage(program, opts));
            RunConfiguration {
                path: String::from(default_config_path),
                start_pages: vec![],
                exit_status: Some((1, message))
            }
        }
    }
}

fn usage(program: String, opts: Options) -> String {
    let brief = format!("Usage: {} [options] [URI ...]", program);
    return opts.usage(&brief);
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

