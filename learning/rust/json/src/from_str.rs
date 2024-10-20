use crate::JSON;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::Chars;
use std::str::FromStr;

mod keychars {
    pub(super) const NULL_STARTS: char = 'n';
    pub(super) const NULL_REST: [char; 3] = ['u', 'l', 'l'];

    pub(super) const STRING_STARTS: char = '"';
    pub(super) const OBJECT_STARTS: char = '{';
}

use keychars::*;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidJSON(&'static str),
    InvalidControlCharacter(&'static str),
}

const NULL_PARSING_ERROR: ParseError = ParseError::InvalidJSON("Error while parsing `null`");
const NEWLINE_CHARACTER_ERROR: ParseError =
    ParseError::InvalidJSON("Newline character is not allowed");

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for JSON {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(&mut s.chars())
    }
}

fn parse(chars: &mut Chars) -> Result<JSON, ParseError> {
    match chars.find(|ch| !ch.is_whitespace()) {
        Some(NULL_STARTS) => {
            parse_null(chars.by_ref())?;
            Ok(JSON::Null)
        }

        Some(STRING_STARTS) => {
            let string = parse_string(chars.by_ref())?;
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
        Err(NULL_PARSING_ERROR)
    }
}

/// Take symbols until '"' appears
fn parse_string(chars: &mut Chars) -> Result<String, ParseError> {
    const STOP_TAKING: bool = false;
    const CONTINUE_TAKE: bool = true;

    let mut escape_char = false;
    let mut error: Option<ParseError> = None;

    let string = chars
        .take_while(|ch| {
            match ch {
                '\\' => {
                    escape_char = !escape_char;
                    CONTINUE_TAKE
                }
                'n' => {
                    if escape_char {
                        error = Some(NEWLINE_CHARACTER_ERROR);
                        STOP_TAKING
                    } else {
                        CONTINUE_TAKE
                    }
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
        .collect();

    if let Some(err) = error {
        Err(err)
    } else {
        Ok(string)
    }
}

fn parse_object(chars: &mut Chars) -> Result<HashMap<String, JSON>, ParseError> {
    unimplemented!();
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
        let parsed = parse(&mut r#"{"key": "value"}"#.chars()).unwrap();

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
        let err = parse_null(&mut chars).err().unwrap();
        let expect_err = NULL_PARSING_ERROR;

        assert_eq!(expect_err, err);
    }

    #[test]
    fn parse_string_happy() {
        let mut chars = r#"something""#.chars();
        let parsed = parse_string(chars.by_ref()).unwrap();

        assert_eq!(parsed, "something".to_string())
    }

    #[test]
    fn parse_string_quotes() {
        let mut chars = r#"something\" ""#.chars();
        let parsed = parse_string(chars.by_ref()).unwrap();

        assert_eq!(parsed, r#"something\" "#.to_string())
    }

    #[test]
    fn parse_string_escapes() {
        let mut chars = r#"\\something\\ ""#.chars();
        let parsed = parse_string(chars.by_ref()).unwrap();

        assert_eq!(parsed, r#"\\something\\ "#.to_string())
    }
}
