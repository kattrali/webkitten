extern crate toml;

use std::env;
use std::fs::File;
use std::io::Read;
use self::toml::Value;

pub const DEFAULT_CONFIG: &'static str = r#"
[window]
start-page = "https://delisa.fuller.li"

[keybindings]
back = "<Control>Left"
forward = "<Control>Right"

[alias]
r = "reload"
"#;

/// Configuration key for the page which should be loaded when opening a new
/// window
pub const WINDOW_START_PAGE: &'static str = "window.start-page";

/// Placeholder used in webkitten configuration to represent the configuration
/// property `general.config-dir`.
const CONFIG_DIR: &'static str = "CONFIG_DIR";

const HOME: &'static str = "HOME";

pub struct Config {
    value: Value
}

impl Config {

    pub fn default() -> Option<Self> {
        Config::parse(DEFAULT_CONFIG)
    }

    /// Reload cached configuration from disk
    /// returns true if parsing is successful
    pub fn load(&mut self, path: &str) -> bool {
        if let Some(update) = Config::parse_file(path) {
            self.value = update.value;
            true
        } else {
            false
        }
    }

    pub fn parse_file(path: &str) -> Option<Self> {
        File::open(path)
            .ok()
            .and_then(|mut file| {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).ok().and_then(|_| {
                    Config::parse(buffer.as_str())
                })
            })
    }

    pub fn parse(raw_input: &str) -> Option<Self> {
        let result = raw_input.parse();
        match result {
            Ok(value) => Some(Config { value: value }),
            Err(errors) => {
                for err in errors {
                    error!("Failed to parse toml: {}", err);
                }
                None
            },
        }
    }

    pub fn lookup<'a>(&'a self, key: &'a str) -> Option<&Value> {
        self.value.lookup(key)
    }

    pub fn lookup_path_slice(&self, key: &str) -> Option<Vec<String>> {
        self.value.lookup(key)
            .and_then(|value| value.as_slice())
            .and_then(|values| {
                let mut resolved_paths = vec![];
                for path in values {
                    if let Some(path) = path.as_str() {
                        resolved_paths.push(self.parse_path(path))
                    }
                }
                Some(resolved_paths)
            })
    }

    pub fn lookup_bool<'a>(&'a self, key: &'a str) -> Option<bool> {
        self.value.lookup(key)
            .and_then(|value| value.as_bool())
    }

    pub fn lookup_str<'a>(&'a self, key: &'a str) -> Option<&'a str> {
        self.value.lookup(key)
            .and_then(|value| value.as_str())
            .and_then(|value| Some(value))
    }

    pub fn lookup_path<'a>(&self, key: &'a str) -> Option<String> {
        self.lookup_str(key)
            .and_then(|value| Some(self.parse_path(&value)))
    }

    pub fn parse_path<'a>(&self, value: &'a str) -> String {
        self.replace_config_dir(&self.replace_home(value))
    }

    fn replace_config_dir<'a>(&self, value: &'a str) -> String {
        self.lookup_str("general.config-dir")
            .and_then(|config_dir| Some(value.replace(CONFIG_DIR, &config_dir)))
            .unwrap_or(String::from(value))
    }

    fn replace_home<'a>(&self, value: &'a str) -> String {
        env::home_dir()
            .and_then(|home| {
                if let Some(home) = home.to_str() {
                    Some(value.replace(HOME, &home))
                } else {
                    None
                }
            })
            .unwrap_or(String::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_parse() {
        let config = Config::parse(super::DEFAULT_CONFIG).unwrap();
        let start_page = config.lookup("window.start-page").unwrap();
        assert_eq!(start_page.as_str().unwrap(), "https://delisa.fuller.li");
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_parse_file() {
        let mut path = temp_dir();
        path.push("test-config.toml");
        let mut file = File::create(path.as_path()).ok().unwrap();
        assert!(file.write(super::DEFAULT_CONFIG.as_bytes()).is_ok());
        file.flush();
        let config = Config::parse_file(path.to_str().unwrap()).unwrap();
        let start_page = config.lookup("window.start-page").unwrap();
        assert_eq!(start_page.as_str().unwrap(), "https://delisa.fuller.li");
    }

    #[test]
    fn test_defalt_config() {
        let config = Config::default().unwrap();
        let back = config.lookup("keybindings.back").unwrap();
        assert_eq!(back.as_str().unwrap(), "<Control>Left");
    }
}
