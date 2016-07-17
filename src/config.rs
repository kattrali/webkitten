use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use toml::Value;
use url::Url;

use ui::BrowserConfiguration;

pub const DEFAULT_CONFIG: &'static str = r#"
[general]
private-browsing = false

[window]
start-page = "https://delisa.fuller.li"
"#;

/// Placeholder used in webkitten configuration to represent the configuration
/// property `general.config-dir`.
const CONFIG_DIR: &'static str = "CONFIG_DIR";

const HOME: &'static str = "HOME";

pub struct Config {
    value: Value
}

impl BrowserConfiguration for Config {

    fn parse(raw_input: &str) -> Option<Self> {
        match raw_input.parse() {
            Ok(value) => Some(Config { value: value }),
            Err(errors) => {
                for err in errors { error!("Failed to parse toml: {}", err); }
                None
            },
        }
    }

    fn lookup_bool<'a>(&'a self, key: &'a str) -> Option<bool> {
        self.lookup(key)
            .and_then(|value| value.as_bool())
    }

    fn lookup_raw_str<'a>(&'a self, key: &'a str) -> Option<String> {
        self.lookup(key)
            .and_then(|value| value.as_str())
            .and_then(|value| Some(String::from(value)))
    }

    fn lookup_str<'a>(&'a self, key: &'a str) -> Option<String> {
        self.lookup_raw_str(key)
            .and_then(|value| Some(self.parse_path(&value)))
    }

    fn lookup_integer<'a>(&'a self, key: &'a str) -> Option<i64> {
        self.lookup(key)
            .and_then(|value| value.as_integer())
    }

    fn lookup_str_table(&self, key: &str) -> Option<HashMap<String, String>> {
        if let Some(table) = self.lookup(key).and_then(|value| value.as_table()) {
            let mut map: HashMap<String, String> = HashMap::new();
            for (key, raw_value) in table {
                if let Some(value) = raw_value.as_str() {
                    map.insert(key.to_owned(), value.to_owned());
                }
            }
            return Some(map);
        }
        None
    }

    fn lookup_str_vec(&self, key: &str) -> Option<Vec<String>> {
        self.lookup(key)
            .and_then(|value| value.as_slice())
            .and_then(|values| {
                let mut str_values: Vec<String> = vec![];
                for value in values {
                    if let Some(value) = value.as_str() {
                        str_values.push(self.parse_path(value))
                    }
                }
                Some(str_values)
            })
    }

    fn lookup_site_bool<'a>(&'a self, uri: &str, key: &'a str) -> Option<bool> {
        site_config_key(uri, key).and_then(|key| self.lookup_bool(&key))
    }

    fn lookup_site_str<'a>(&'a self, uri: &str, key: &'a str) -> Option<String> {
        site_config_key(uri, key).and_then(|key| self.lookup_str(&key))
    }

    fn lookup_site_str_vec<'a>(&'a self, uri: &str, key: &'a str) -> Option<Vec<String>> {
        site_config_key(uri, key).and_then(|key| self.lookup_str_vec(&key))
    }
}

impl Config {

    /// Create a `Configuration` with the default options
    pub fn default() -> Option<Self> {
        Config::parse(DEFAULT_CONFIG)
    }

    /// Reload cached configuration from disk returns true if parsing is
    /// successful
    pub fn load(&mut self, path: &str) -> bool {
        if let Some(update) = Config::parse_file(path) {
            self.value = update.value;
            true
        } else {
            false
        }
    }

    /// Parse a file at a path and create a `Configuration` if possible
    pub fn parse_file(path: &str) -> Option<Self> {
        let mut buffer = String::new();
        File::open(path).ok()
            .and_then(|mut file| file.read_to_string(&mut buffer).ok())
            .and_then(|_| Config::parse(buffer.as_str()))
    }

    /// Look up the raw TOML value for a key
    fn lookup<'a>(&'a self, key: &'a str) -> Option<&Value> {
        self.value.lookup(&key.clone())
    }

    fn parse_path(&self, value: &str) -> String {
        self.replace_config_dir(&self.replace_home(value))
    }

    fn replace_config_dir<'a>(&self, value: &'a str) -> String {
        self.config_dir()
            .and_then(|dir| Some(value.replace(CONFIG_DIR, &dir)))
            .unwrap_or(String::from(value))
    }

    fn replace_home(&self, value: &str) -> String {
        if let Some(home) = env::home_dir() {
            if let Some(home) = home.to_str() {
                return value.replace(HOME, &home)
            }
        }
        String::from(value)
    }
}

fn site_config_key(uri: &str, key: &str) -> Option<String> {
    if let Ok(url) = Url::parse(&uri) {
        if let Some(host) = url.host_str() {
            return Some(format!("sites.\"{}\".{}", host, key))
        }
    }
    None
}
