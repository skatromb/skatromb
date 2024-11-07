mod numeric;

use crate::from_str::numeric::parse_numeric;
use crate::JSON;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidJSON,
    InvalidControlCharacter,
    InvalidEscapeCharacter,
    UnclosedStringLiteral,
    UnclosedObjectLiteral,
    BooleanParsingError,
    NumericParsingError,
}

use ParseError::*;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Skips whitespaces, but doesn't consume first non-whitespace character, unlike `.skip_while()`
trait SkipWhitespaces: Iterator<Item = char> {
    fn skip_whitespaces(&mut self);
}

impl<I> SkipWhitespaces for Peekable<I>
where
    I: Iterator<Item = char>,
{
    fn skip_whitespaces(&mut self) {
        while let Some(char) = self.peek() {
            if char.is_whitespace() {
                self.next().expect("Already peeked so should exist");
            } else {
                break;
            }
        }
    }
}

impl FromStr for JSON {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let json = parse(&mut chars)?;

        chars.skip_whitespaces();
        if chars.peek().is_none() {
            return Ok(json);
        }

        Err(InvalidJSON)
    }
}

fn parse(chars: &mut Peekable<Chars>) -> Result<JSON, ParseError> {
    chars.skip_whitespaces();

    match chars.peek() {
        Some('f') | Some('t') => {
            let boolean = parse_bool(chars)?;
            Ok(JSON::Bool(boolean))
        }

        Some('n') => {
            parse_null(chars)?;
            Ok(JSON::Null)
        }

        Some('"') => {
            let string = parse_string(chars)?;
            Ok(JSON::String(string))
        }

        Some('0'..='9') | Some('-') => parse_numeric(chars),

        Some('{') => {
            let object = parse_object(chars)?;
            Ok(JSON::Object(object))
        }

        Some('[') => {
            let arr = parse_array(chars)?;
            Ok(JSON::Array(arr))
        }

        _ => Err(InvalidJSON),
    }
}

fn parse_bool(chars: &mut Peekable<Chars>) -> Result<bool, ParseError> {
    match chars.peek().ok_or(BooleanParsingError)? {
        't' => {
            if "true" == chars.take(4).collect::<String>() {
                return Ok(true);
            }
        }
        'f' => {
            if "false" == chars.take(5).collect::<String>() {
                return Ok(false);
            }
        }
        _ => {}
    }
    Err(BooleanParsingError)
}

fn parse_null(chars: &mut impl Iterator<Item = char>) -> Result<(), ParseError> {
    if chars.take(4).collect::<String>() == "null" {
        Ok(())
    } else {
        Err(InvalidJSON)
    }
}

fn parse_string(chars: &mut Peekable<Chars>) -> Result<String, ParseError> {
    let mut string = String::new();

    if chars.next() != Some('"') {
        return Err(InvalidJSON);
    };

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

fn parse_object(chars: &mut Peekable<Chars>) -> Result<HashMap<String, JSON>, ParseError> {
    if chars.next() != Some('{') {
        return Err(InvalidJSON);
    }
    let mut hash_map = HashMap::new();

    loop {
        chars.skip_whitespaces();
        let key = parse_string(chars)?;

        chars.skip_whitespaces();
        if chars.find(|char| !char.is_whitespace()) != Some(':') {
            return Err(InvalidJSON);
        }

        let value = parse(chars)?;

        hash_map.insert(key, value);

        chars.skip_whitespaces();
        match chars.peek() {
            Some(',') => {
                chars.next();
                continue;
            }
            Some('}') => {
                chars.next();
                return Ok(hash_map);
            }
            _ => {
                return Err(UnclosedObjectLiteral);
            }
        }
    }
}

fn parse_array(chars: &mut Peekable<Chars>) -> Result<Vec<JSON>, ParseError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn skip_whitespaces_dont_consume_char() {
        let chars = &mut " 1  2   321".chars().peekable();
        chars.skip_whitespaces();
        assert_eq!(chars.next().unwrap(), '1');

        chars.skip_whitespaces();
        assert_eq!(chars.next().unwrap(), '2');

        chars.skip_whitespaces();
        let string: String = chars.collect();
        assert_eq!("321", string);
    }

    #[test]
    fn skip_whitespaces_stops_on_iterator_end() {
        let chars = &mut " ".chars().peekable();
        chars.skip_whitespaces();

        assert!(chars.next().is_none());
    }

    #[test]
    fn parse_happy() {
        let json = JSON::Object(HashMap::from([
            ("key".to_string(), JSON::String("value".to_string())),
            (
                "another_key".to_string(),
                JSON::String("another_value".to_string()),
            ),
        ]));
        let parsed: JSON = r#" {"key": "value", "another_key": "another_value"} "#
            .parse()
            .unwrap();

        assert_eq!(json, parsed)
    }

    #[test]
    fn parse_bool_happy() {
        let mut chars = "true".chars().peekable();
        let parsed = parse_bool(&mut chars).unwrap();
        assert!(parsed);

        let mut chars = "false".chars().peekable();
        let parsed = parse_bool(&mut chars).unwrap();
        assert!(!parsed);
    }

    #[test]
    fn parse_bool_fail() {
        let mut chars = "fAAA!!!".chars().peekable();
        let parsed = parse_bool(&mut chars).err().unwrap();

        assert_eq!(parsed, BooleanParsingError);
    }

    #[test]
    #[allow(clippy::unit_cmp)]
    fn parse_null_happy() {
        let mut chars = "null".chars().peekable();

        #[allow(clippy::let_unit_value)]
        let parsed = parse_null(&mut chars).unwrap();

        assert_eq!((), parsed)
    }

    #[test]
    fn parse_null_fail() {
        let mut chars = "NOT A NULL".chars().peekable();
        let err = parse_null(&mut chars).err().unwrap();

        assert_eq!(InvalidJSON, err);
    }

    #[test]
    fn parse_string_happy() {
        let mut chars = r#""string""#.chars().peekable();
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, "string")
    }

    #[test]
    fn parse_string_quotes() {
        let mut chars = r#"" `\"` ""#.chars().peekable();
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, " `\"` ")
    }

    #[test]
    fn parse_string_escapes_happy() {
        let mut chars = r#"" `\\` ""#.chars().peekable();
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, " `\\` ")
    }

    #[test]
    fn parse_string_escapes_fail() {
        let mut chars = r#"" `\` ""#.chars().peekable();
        let parsed = parse_string(&mut chars).err().unwrap();

        assert_eq!(parsed, InvalidEscapeCharacter)
    }

    #[test]
    fn parse_string_newline_happy() {
        let mut chars = r#"" newline! \n another line ""#.chars().peekable();
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, " newline! \n another line ")
    }

    #[test]
    fn parse_string_newline_fail() {
        let mut chars = r"newline!
            another line"
            .chars()
            .peekable();
        let parsed = parse_string(&mut chars).err().unwrap();

        assert_eq!(parsed, InvalidJSON)
    }
}
