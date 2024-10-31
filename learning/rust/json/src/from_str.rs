use crate::JSON;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::Chars;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidJSON,
    InvalidControlCharacter,
    InvalidEscapeCharacter,
    UnclosedStringLiteral,
}

use ParseError::*;

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
        Some('n') => {
            parse_null(chars)?;
            Ok(JSON::Null)
        }

        Some('"') => {
            let string = parse_string(chars)?;
            Ok(JSON::String(string))
        }

        Some('{') => {
            let object = parse_object(chars)?;
            Ok(JSON::Object(object))
        }

        _ => Err(InvalidJSON),
    }
}

/// Take 3 symbols after 'n' and expect them to be "ull"
fn parse_null(chars: &mut Chars) -> Result<(), ParseError> {
    const NULL_REST: [char; 3] = ['u', 'l', 'l'];

    if chars.take(3).eq(NULL_REST) {
        Ok(())
    } else {
        Err(InvalidJSON)
    }
}

fn parse_string(chars: &mut Chars) -> Result<String, ParseError> {
    let mut string = String::new();

    loop {
        let char = chars.next().ok_or(UnclosedStringLiteral)?;

        match char {
            '"' => break,
            '\n' => return Err(InvalidEscapeCharacter),
            '\\' => {
                let next_symbol = chars.next().ok_or(UnclosedStringLiteral)?;
                let escaped_char = match next_symbol {
                    '\\' => '\\',
                    '/' => '/',
                    '"' => '"',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    _ => {
                        return Err(InvalidEscapeCharacter);
                    }
                };
                string.push(escaped_char);
            }
            _ => string.push(char),
        }
    }
    Ok(string)
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
        let expect_err = InvalidJSON;

        assert_eq!(expect_err, err);
    }

    #[test]
    fn parse_string_happy() {
        let mut chars = r#"string""#.chars();
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, "string")
    }

    #[test]
    fn parse_string_quotes() {
        let mut chars = r#" `\"` ""#.chars();
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, " `\"` ")
    }

    #[test]
    fn parse_string_escapes_happy() {
        let mut chars = r#" `\\` ""#.chars();
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, " `\\` ")
    }

    #[test]
    fn parse_string_escapes_fail() {
        let mut chars = r#" `\` ""#.chars();
        let parsed = parse_string(&mut chars).err().unwrap();

        assert_eq!(parsed, InvalidEscapeCharacter)
    }

    #[test]
    fn parse_string_newline_happy() {
        let mut chars = r#"newline! \n another line""#.chars();
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, "newline! \n another line")
    }

    #[test]
    fn parse_string_newline_fail() {
        let mut chars = r"newline!
            another line"
            .chars();
        let parsed = parse_string(&mut chars).err().unwrap();

        assert_eq!(parsed, InvalidEscapeCharacter)
    }
}
