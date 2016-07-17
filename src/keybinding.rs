use std::error;
use std::fmt;

pub enum KeyMask {
    Shift     = 1 << 16,
    Control   = 1 << 18,
    Alternate = 1 << 19,
    Super     = 1 << 20,
    Function  = 1 << 23,
}

const SHIFT_CODES: [&'static str; 1] = ["shift"];
const CONTROL_CODES: [&'static str; 2] = ["control", "ctrl"];
const ALT_CODES: [&'static str; 3] = ["alt", "option", "opt"];
const SUPER_CODES: [&'static str; 3] = ["super", "cmd", "command"];
const FN_CODES: [&'static str; 2] = ["fn", "function"];
const SPACE_CODE: &'static str = "space";

#[derive(Debug,PartialEq)]
pub struct ParseError {
    message: String,
    reason: ParseErrorReason,
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum ParseErrorReason {
    ModifierNotFound,
    NoKeySpecified,
    NoModifierSpecified,
    TooManyKeysSpecified,
}

impl error::Error for ParseError {

    fn description(&self) -> &str {
        &self.message
    }
}

impl ParseError {

    fn new(reason: ParseErrorReason,
           keys: Vec<&str>,
           modifier: &str) -> ParseError {
        let message = match reason {
            ParseErrorReason::ModifierNotFound =>
                format!("Modifier not found for '{}'", modifier),
            ParseErrorReason::NoKeySpecified =>
                format!("No key specified in chord"),
            ParseErrorReason::NoModifierSpecified =>
                format!("No modifier specified in chord"),
            ParseErrorReason::TooManyKeysSpecified =>
                format!("Too many keys specified in chord: {}", keys.join(" ")),
        };
        ParseError { reason: reason, message: message }
    }
}

impl fmt::Display for ParseError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn parse(chord: &str) -> Result<(char, usize), ParseError> {
    let mut key: Option<&str> = None;
    let mut modifier: usize = 0;
    for component in chord.split(" ") {
        if component.len() == 1 || component == SPACE_CODE {
            if key.is_some() {
                return Err(ParseError::new(ParseErrorReason::TooManyKeysSpecified,
                                           vec![key.unwrap(), component],
                                           ""));
            } else if component == SPACE_CODE {
                key = Some(" ");
            } else {
                key = Some(component);
            }
        } else {
            let code = component.to_lowercase();
            if ALT_CODES.contains(&code.as_str()) {
                modifier |= KeyMask::Alternate as usize;
            } else if SUPER_CODES.contains(&code.as_str()) {
                modifier |= KeyMask::Super as usize;
            } else if SHIFT_CODES.contains(&code.as_str()) {
                modifier |= KeyMask::Shift as usize;
            } else if CONTROL_CODES.contains(&code.as_str()) {
                modifier |= KeyMask::Control as usize;
            } else if FN_CODES.contains(&code.as_str()) {
                modifier |= KeyMask::Function as usize;
            } else {
                return Err(ParseError::new(ParseErrorReason::ModifierNotFound,
                                           vec![],
                                           component));
            }
        }
    }

    if key.is_none() {
        Err(ParseError::new(ParseErrorReason::NoKeySpecified, vec![], ""))
    } else if modifier == 0 {
        Err(ParseError::new(ParseErrorReason::NoModifierSpecified, vec![], ""))
    } else {
        Ok((key.unwrap().chars().next().unwrap(), modifier))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_invalid_modifier_reason() {
        let chord = parse("Hyper 7");
        assert_eq!(ParseErrorReason::ModifierNotFound,
                   chord.err().unwrap().reason);
    }

    #[test]
    fn parse_invalid_modifier_modifier() {
        let chord = parse("Hyper 7");
        assert!(chord.err().unwrap().message.contains("Hyper"));
    }

    #[test]
    fn parse_too_many_characters_reason() {
        let chord = parse("Ctrl Alt d a");
        assert_eq!(ParseErrorReason::TooManyKeysSpecified,
                  chord.err().unwrap().reason);
    }

    #[test]
    fn parse_no_modifier_reason() {
        let chord = parse("a");
        assert_eq!(ParseErrorReason::NoModifierSpecified,
                   chord.err().unwrap().reason);
    }

    #[test]
    fn parse_no_key_reason() {
        let chord = parse("Ctrl Alt");
        assert_eq!(ParseErrorReason::NoKeySpecified,
                   chord.err().unwrap().reason);
    }

    #[test]
    fn parse_case_insensitive_modifiers() {
        let chord1 = parse("CTRL OPTION a");
        let chord2 = parse("Ctrl option a");
        assert_eq!(chord1, chord2);

        let value = chord1.ok().unwrap();
        assert_eq!('a', value.0);
        assert_eq!(KeyMask::Control as usize | KeyMask::Alternate as usize,
                   value.1);
    }

    #[test]
    fn parse_alt_option_modifiers() {
        let chord1 = parse("Option b");
        let chord2 = parse("Alt b");
        assert_eq!(chord1, chord2);

        let value = chord1.ok().unwrap();
        assert_eq!('b', value.0);
        assert_eq!(KeyMask::Alternate as usize, value.1);
    }

    #[test]
    fn parse_command_modifier() {
        assert_eq!(KeyMask::Super as usize,
                   parse("cmd 1").ok().unwrap().1);
        assert_eq!(KeyMask::Super as usize,
                   parse("command 1").ok().unwrap().1);
        assert_eq!(KeyMask::Super as usize,
                   parse("super 1").ok().unwrap().1);
    }

    #[test]
    fn parse_alt_modifier() {
        assert_eq!(KeyMask::Alternate as usize,
                   parse("alt 1").ok().unwrap().1);
        assert_eq!(KeyMask::Alternate as usize,
                   parse("opt 1").ok().unwrap().1);
        assert_eq!(KeyMask::Alternate as usize,
                   parse("option 1").ok().unwrap().1);
    }

    #[test]
    fn parse_function_modifier() {
        assert_eq!(KeyMask::Function as usize,
                   parse("fn 1").ok().unwrap().1);
        assert_eq!(KeyMask::Function as usize,
                   parse("function 1").ok().unwrap().1);
    }

    #[test]
    fn parse_control_modifier() {
        assert_eq!(KeyMask::Control as usize,
                   parse("ctrl 1").ok().unwrap().1);
        assert_eq!(KeyMask::Control as usize,
                   parse("control 1").ok().unwrap().1);
    }

    #[test]
    fn parse_space_key() {
        assert_eq!(' ', parse("space control").ok().unwrap().0);
    }

    #[test]
    fn parse_shift_modifier() {
        assert_eq!(KeyMask::Shift as usize,
                   parse("shift 1").ok().unwrap().1);
    }
}
