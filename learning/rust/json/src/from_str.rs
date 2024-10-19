use crate::JSON;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::Chars;
use std::str::FromStr;

mod keychars {
    pub(super) const NULL_STARTS: char = 'n';
    pub(super) const NULL_REST: [char; 3] = ['u', 'l', 'l'];

    pub(super) const STRING_STARTS: char = '"';
    pub(super) const STRING_ENDS: char = STRING_STARTS;

    pub(super) const OBJECT_STARTS: char = '{';
    pub(super) const OBJECT_ENDS: char = '}';
}

use keychars::*;

#[derive(Debug)]
pub enum ParseError {
    InvalidJSON(&'static str),
}

const NULL_PARSING_ERROR: &str = "Error while parsing `null`";

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for JSON {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

fn parse(string: &str) -> Result<JSON, ParseError> {
    let mut chars = string.chars();

    match chars.find(|ch| !ch.is_whitespace()) {
        Some(NULL_STARTS) => {
            parse_null(chars.by_ref())?;
            Ok(JSON::Null)
        }

        Some(STRING_STARTS) => {
            let string = parse_string(chars.by_ref());
            Ok(JSON::String(string))
        }

        Some(OBJECT_STARTS) => {
            let object = parse_object(chars.by_ref())?;
            Ok(JSON::Object(object))
        }

        _ => Err(ParseError::InvalidJSON("Incorrect symbol in JSON")),
    }
}

/// Take 3 symbols after 'n' and expect them to be "ull"
fn parse_null(chars: &mut Chars) -> Result<(), ParseError> {
    if chars.take(3).eq(NULL_REST) {
        Ok(())
    } else {
        Err(ParseError::InvalidJSON(NULL_PARSING_ERROR))
    }
}

/// Take symbols until '"' appears
fn parse_string(chars: &mut Chars) -> String {
    const STOP_TAKING: bool = false;
    const CONTINUE_TAKE: bool = true;

    let mut escape_char = false;

    chars
        .take_while(|ch| {
            match ch {
                '\\' => {
                    escape_char = !escape_char;
                    CONTINUE_TAKE
                }
                '"' => {
                    if escape_char {
                        escape_char = false;
                        CONTINUE_TAKE
                    } else {
                        STOP_TAKING // on `"` we exit
                    }
                }
                _ => {
                    escape_char = false;
                    CONTINUE_TAKE
                }
            }
        })
        .collect()
}

fn parse_object(chars: &mut Chars) -> Result<HashMap<String, JSON>, ParseError> {
    unimplemented!();
    let char = chars.next();
    let mut string = String::new();

    match char {
        None => return Err(ParseError::InvalidJSON("Error while parsing object")),
        _ => {
            unimplemented!()
        }
    }
}

fn parse_integer(chars: &mut Chars) -> i64 {
    unimplemented!()
}

fn parse_float(chars: &mut Chars) -> f64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn parse_happy() {
        let json = JSON::Object(HashMap::from([(
            "key".to_string(),
            JSON::String("value".to_string()),
        )]));
        let parsed = parse(r#"{"key": "value"}"#).unwrap();

        assert_eq!(json, parsed)
    }

    #[test]
    #[allow(clippy::unit_cmp)]
    fn parse_null_happy() {
        let mut chars = "ull".chars();

        #[allow(clippy::let_unit_value)]
        let parsed = parse_null(&mut chars).unwrap();

        assert_eq!((), parsed)
    }

    #[test]
    fn parse_null_unhappy() {
        let mut chars = "NOT A NULL".chars();

        let parsed = parse_null(&mut chars).expect_err(NULL_PARSING_ERROR);
    }

    #[test]
    fn parse_string_happy() {
        let mut chars = r#"something""#.chars();
        let parsed = parse_string(chars.by_ref());

        assert_eq!(parsed, "something".to_string())
    }

    #[test]
    fn parse_string_quotes() {
        let mut chars = r#"something\" ""#.chars();
        let parsed = parse_string(chars.by_ref());

        assert_eq!(parsed, r#"something\" "#.to_string())
    }

    #[test]
    fn parse_string_escapes() {
        let mut chars = r#"\\something\\ ""#.chars();
        let parsed = parse_string(chars.by_ref());

        assert_eq!(parsed, r#"\\something\\ "#.to_string())
    }
}
