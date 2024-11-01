#![allow(unused)]
use crate::JSON;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidJSON,
    InvalidControlCharacter,
    InvalidEscapeCharacter,
    UnclosedStringLiteral,
    UnclosedObjectLiteral,
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
        let mut chars = s.chars().peekable();
        let json = parse(&mut chars)?;

        skip_whitespaces(&mut chars);
        assert!(chars.peek().is_none());

        Ok(json)
    }
}

/// Skips whitespaces but doesn't consume first non-whitespace character unlike `.skip_while()`
fn skip_whitespaces(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<(), ParseError> {
    while chars.peek().ok_or(InvalidJSON)?.is_whitespace() {
        let _ = chars.next();
    }
    Ok(())
}

fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<JSON, ParseError> {
    skip_whitespaces(chars)?;

    let result = match chars.peek() {
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
    };

    result
}

fn parse_null(chars: &mut impl Iterator<Item = char>) -> Result<(), ParseError> {
    if chars.take(4).collect::<String>() == "null" {
        Ok(())
    } else {
        Err(InvalidJSON)
    }
}

fn parse_string(chars: &mut impl Iterator<Item = char>) -> Result<String, ParseError> {
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

fn parse_object(
    chars: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<HashMap<String, JSON>, ParseError> {
    if chars.next() != Some('{') {
        return Err(InvalidJSON);
    }
    let mut hash_map = HashMap::new();

    loop {
        skip_whitespaces(chars);
        let key = parse_string(chars)?;

        if chars.find(|char| !char.is_whitespace()) != Some(':') {
            return Err(InvalidJSON);
        }

        let value = parse(chars)?;

        hash_map.insert(key, value);

        skip_whitespaces(chars);
        match chars.peek() {
            Some(',') => {
                chars.next();
                continue;
            }
            Some('}') => {
                chars.next();
                return Ok(hash_map);
            }
            _ => return Err(UnclosedObjectLiteral),
        }
    }
}

fn parse_integer(chars: &mut impl Iterator<Item = char>) -> i64 {
    unimplemented!()
}

fn parse_float(chars: &mut impl Iterator<Item = char>) -> f64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn skip_whitespaces_dont_consume_char() {
        let chars = &mut " 1  2   321".chars().peekable();
        skip_whitespaces(chars);
        assert_eq!(chars.next().unwrap(), '1');

        skip_whitespaces(chars);
        assert_eq!(chars.next().unwrap(), '2');

        skip_whitespaces(chars);
        let string: String = chars.collect();
        assert_eq!("321", string);
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
    #[allow(clippy::unit_cmp)]
    fn parse_null_happy() {
        let mut chars = "null".chars().peekable();

        #[allow(clippy::let_unit_value)]
        let parsed = parse_null(&mut chars).unwrap();

        assert_eq!((), parsed)
    }

    #[test]
    fn parse_null_unhappy() {
        let mut chars = "NOT A NULL".chars().peekable();
        let err = parse_null(&mut chars).err().unwrap();
        let expect_err = InvalidJSON;

        assert_eq!(expect_err, err);
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
