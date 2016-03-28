extern crate hlua;

use self::hlua::{Lua,LuaError};
use std::path::Path;
use std::fs::{File,metadata};

/// A representation of a script which executes and returns a boolean value
/// indicating success
#[derive(Debug,Clone)]
pub struct Command {
    path: String,
    arguments: Vec<String>,
    name: String,
}

/// Parse a command name and arguments into an instance of Command
pub fn parse_command(search_paths: Vec<&str>, input: &str) -> Option<Command> {
    let mut components = input.split_whitespace();
    return match components.next() {
        Some(name) => match resolve_command(search_paths, name) {
            Some(path) => Some(Command {
                path: path,
                arguments: components.map(|arg| String::from(arg)).collect(),
                name: String::from(name) }),
            None => None
        },
        None => None
    }
}

/// Run a command instance in a new Lua scripting runtime
pub fn execute(command: Command) -> Result<bool, LuaError> {
    let mut lua = Lua::new();
    lua.set("selected_window_index", 0);
    lua.set("selected_buffer_index", 0);
    lua.set("arguments", command.arguments);
    return match File::open(command.path) {
        Ok(file) => lua.execute_from_reader::<bool, _>(file),
        Err(e) => Err(LuaError::ReadError(e))
    };
}

/// Iterate over search paths returning the first file path in search paths
/// with the provided name
fn resolve_command(search_paths: Vec<&str>, name: &str) -> Option<String> {
    if name.is_empty() {
        return None
    }
    let mut ordered_paths: Vec<String> = search_paths.iter()
        .filter_map(|path| join_paths(path, name.clone()))
        .filter(|path| metadata(&path).is_ok())
        .collect();
    ordered_paths.reverse();
    return ordered_paths.pop();
}

/// Join a directory and file name into a string path if possible
fn join_paths(dir: &str, file_name: &str) -> Option<String> {
    let buf = Path::new(dir).join(file_name);
    return match buf.to_str() {
        Some(path) => Some(String::from(path)),
        None => None
    }
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
        assert_eq!(command.name, "hello");
        assert_eq!(command.arguments, vec![String::from("world")]);
        assert_eq!(command.path, path);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_execute() {
        let (path, result) = create_command("call-and-return",
                                         b"return true;",
                                         "call-and-return");
        let command = result.unwrap();
        let output = execute(command.clone());
        remove_file(Path::new(path.as_str().clone()));
        assert!(output.is_ok());
        assert!(output.ok().unwrap());

    }

    #[allow(unused_must_use)]
    fn create_command(name: &str, content: &[u8], invocation: &str) -> (String, Option<Command>) {
        let dir = temp_dir();
        let search_path = String::from(dir.clone().to_str().unwrap());
        let file_path = Path::new(dir.to_str().unwrap()).join(name);
        let mut file = File::create(file_path.as_path()).ok().unwrap();
        assert!(file.write(content).is_ok());
        file.flush();
        let result = parse_command(vec![&search_path], invocation);
        return (String::from(file_path.to_str().unwrap()), result);
    }
}
