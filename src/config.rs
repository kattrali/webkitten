extern crate toml;

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

pub fn default_config() -> Option<Value> {
    return DEFAULT_CONFIG.parse().ok();
}

pub fn parse_config(raw_input: &str) -> Option<Value> {
    return raw_input.parse().ok();
}

pub fn parse_config_file(path: &str) -> Option<Value> {
    return match File::open(path) {
        Ok(mut file) => {
            let mut buffer = String::new();
            match file.read_to_string(&mut buffer) {
                Ok(_) => parse_config(buffer.as_str()),
                _ => None
            }
        },
        _ => None
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
        let config = parse_config(super::DEFAULT_CONFIG).unwrap();
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
        let config = parse_config_file(path.to_str().unwrap()).unwrap();
        let start_page = config.lookup("window.start-page").unwrap();
        assert_eq!(start_page.as_str().unwrap(), "https://delisa.fuller.li");
    }

    #[test]
    fn test_defalt_config() {
        let config = default_config().unwrap();
        let back = config.lookup("keybindings.back").unwrap();
        assert_eq!(back.as_str().unwrap(), "<Control>Left");
    }
}
