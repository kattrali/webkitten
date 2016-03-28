use std::path::Path;
use std::fs::metadata;

pub struct Command {
    pub path: String,
    pub arguments: Vec<String>,
    pub name: String,
}

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
    fn test_resolve_by_filename() {
        let (path, result) = create_command("hello",
                                            b"print(\"hello world\")",
                                            "hello world");
        assert!(result.is_some());

        let command = result.unwrap();
        assert_eq!(command.name, "hello");
        assert_eq!(command.arguments, vec![String::from("world")]);
        assert_eq!(command.path, path);
    }

    fn create_command(name: &str, content: &[u8], invocation: &str) -> (String, Option<Command>) {
        let dir = temp_dir();
        let search_path = String::from(dir.clone().to_str().unwrap());
        let file_path = Path::new(dir.to_str().unwrap()).join(name);
        let mut file = File::create(file_path.as_path()).ok().unwrap();
        file.write(content);
        file.flush();
        let result = parse_command(vec![&search_path], invocation);
        remove_file(file_path.as_path());
        return (String::from(file_path.to_str().unwrap()), result);
    }
}
