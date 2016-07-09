use std::path::Path;
use std::fs::{File,metadata};

use config::Config;
use ui::BrowserConfiguration;

/// A representation of a script which executes and returns a boolean value
/// indicating success
#[derive(Debug,Clone)]
pub struct Command {
    pub path: String,
    pub arguments: Vec<String>,
}

impl Command {

    /// Parse a command name and arguments into an instance of Command
    pub fn parse(input: &str, config: &Config, suffix: &str) -> Option<Self> {
        let mut components = input.split_whitespace();
        components.next()
            .and_then(|name| config.resolved_command_name(name))
            .and_then(|name| resolve_command(config.command_search_paths(), &name, suffix))
            .and_then(|path| {
                Some(Command {
                    path: path,
                    arguments: components.map(|arg| String::from(arg)).collect(),
                })
            })
    }

    /// A File handle to the command path
    pub fn file(&self) -> Option<File> {
        File::open(&self.path).ok()
    }
}

/// Iterate over search paths returning the first file path in search paths
/// with the provided name
fn resolve_command(search_paths: Vec<String>, name: &str, suffix: &str) -> Option<String> {
    if name.is_empty() {
        return None
    }
    let mut ordered_paths: Vec<String> = search_paths.iter()
        .filter_map(|path| join_paths(&path, &format!("{}.{}", name, suffix)))
        .filter(|path| metadata(&path).is_ok())
        .collect();
    ordered_paths.reverse();
    return ordered_paths.pop();
}

/// Join a directory and file name into a string path if possible
fn join_paths(dir: &str, file_name: &str) -> Option<String> {
    let buf = Path::new(dir).join(file_name);
    buf.to_str().and_then(|path| Some(String::from(path)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::{File,remove_file};
    use std::io::Write;
    use std::path::Path;

    #[test]
    #[allow(unused_must_use)]
    fn test_resolve_by_filename() {
        let (path, result) = create_command("hello",
                                            b"print(\"hello world\");",
                                            "hello world");
        remove_file(Path::new(path.as_str().clone()));
        assert!(result.is_some());

        let command = result.unwrap();
        assert_eq!(command.arguments, vec![String::from("world")]);
        assert_eq!(command.path, path);
    }

    #[allow(unused_must_use)]
    fn create_command(name: &str, content: &[u8], invocation: &str) -> (String, Option<Command>) {
        let dir = temp_dir();
        let search_path = String::from(dir.clone().to_str().unwrap());
        let file_path = Path::new(dir.to_str().unwrap()).join(name);
        let mut file = File::create(file_path.as_path()).ok().unwrap();
        assert!(file.write(content).is_ok());
        file.flush();
        let result = Command::parse(invocation, vec![&search_path]);
        return (String::from(file_path.to_str().unwrap()), result);
    }
}
