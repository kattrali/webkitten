extern crate toml;

use self::toml::Value;

const DEFAULT_CONFIG: &'static str = r#"
[[window]]
start-page = "http://delisa.me"

[[keybindings]]
back = "<Control>Left"
forward = "<Control>Right"
"#;

