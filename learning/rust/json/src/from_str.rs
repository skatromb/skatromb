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
    UnclosedArrayLiteral,
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
        Some('n') => {
            parse_null(chars)?;
            Ok(JSON::Null)
        }

        Some('f') | Some('t') => {
            let boolean = parse_bool(chars)?;
            Ok(JSON::Bool(boolean))
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
        let key = match chars.peek() {
            Some('"') => parse_string(chars)?,
            Some('}') => {
                chars.next();
                return Ok(hash_map);
            }
            _ => {
                return Err(UnclosedObjectLiteral);
            }
        };

        chars.skip_whitespaces();
        if chars.find(|char| !char.is_whitespace()) != Some(':') {
            return Err(InvalidJSON);
        }

        let value = parse(chars)?;

        hash_map.insert(key, value);

        chars.skip_whitespaces();
        if let Some(',') = chars.peek() {
            chars.next();
            continue;
        }
    }
}

fn parse_array(chars: &mut Peekable<Chars>) -> Result<Vec<JSON>, ParseError> {
    if chars.next() != Some('[') {
        return Err(InvalidJSON);
    }

    let mut array = Vec::new();

    loop {
        chars.skip_whitespaces();
        let value = match chars.peek() {
            Some(']') => {
                chars.next();
                return Ok(array);
            }
            _ => parse(chars)?,
        };
        // todo: check that Err will be returned in case of unclosed array

        array.push(value);

        chars.skip_whitespaces();
        if let Some(',') = chars.peek() {
            chars.next();
            continue;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn peekable(string: &str) -> Peekable<Chars> {
        string.chars().peekable()
    }

    #[test]
    fn skip_whitespaces_dont_consume_char() {
        let chars = &mut peekable(" 1  2   321");
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
        let chars = &mut peekable(" ");
        chars.skip_whitespaces();

        assert!(chars.next().is_none());
    }

    /// Returns `JSON::Object`, corresponding to JSON
    /// `{"null": null, "boolean": true, "integer": 123, "float": 1234.56789,
    /// "string": "Hello, world!"}`
    fn diverse_hash_map() -> HashMap<String, JSON> {
        let mut map = HashMap::new();

        map.insert("null".to_string(), JSON::Null);
        map.insert("boolean".to_string(), JSON::Bool(true));
        map.insert("integer".to_string(), JSON::Int(123));
        map.insert("float".to_string(), JSON::Float(1234.56789));
        map.insert(
            "string".to_string(),
            JSON::String("Hello, world!".to_string()),
        );

        map
    }

    /// Creates `JSON::Array` corresponding to JSON
    /// `[null, true, 123, 1234.56789, "Hello, world!"]`
    fn diverse_array() -> Vec<JSON> {
        vec![
            JSON::Null,
            JSON::Bool(true),
            JSON::Int(123),
            JSON::Float(1234.56789),
            JSON::String("Hello, world!".to_string()),
        ]
    }

    /// Creates JSON::Objct that combines `diverse_hash_map` and `diverse_array`
    ///
    /// `{"null": null, "boolean": true, "integer": 123, "float": 1234.56789,
    /// "string": "Hello, world!",
    /// "array": [null, true, 123, 1234.56789, "Hello, world!"],
    /// "nested_object": {"null": null, "boolean": true, "integer": 123, "float": 1234.56789,
    /// "string": "Hello, world!"}
    /// }`
    fn diverse_json() -> JSON {
        let mut map = diverse_hash_map();

        map.insert("array".to_string(), JSON::Array(diverse_array()));
        map.insert(
            "nested_object".to_string(),
            JSON::Object(diverse_hash_map()),
        );

        JSON::Object(map)
    }

    #[test]
    fn parse_diverse_json() {
        let json = diverse_json();

        match &json {
            JSON::Object(json) => {
                assert_eq!(json.len(), 7);
            }
            _ => panic!("Test JSON should be JSON Object at the top level!"),
        }

        let mut json_string = peekable(
            r#"
            {
                "null": null,
                "boolean": true,
                "integer": 123,
                "float": 1234.56789,
                "string": "Hello, world!",
                "nested_object": {
                    "null": null,
                    "boolean": true,
                    "integer": 123,
                    "float": 1234.56789,
                    "string": "Hello, world!"
                },
                "array": [
                    null,
                    true,
                    123,
                    1234.56789,
                    "Hello, world!"
                ]
            }
            "#,
        );

        let parsed = parse(&mut json_string).unwrap();

        assert_eq!(parsed, json);
    }

    #[test]
    fn parse_object_happy() {
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
    fn parse_object_empty_happy() {
        let json = JSON::Object(HashMap::new());
        let parsed: JSON = r#" {} "#.parse().unwrap();

        assert_eq!(json, parsed)
    }

    #[test]
    fn parse_bool_happy() {
        let mut chars = peekable("true");
        let parsed = parse_bool(&mut chars).unwrap();
        assert!(parsed);

        let mut chars = peekable("false");
        let parsed = parse_bool(&mut chars).unwrap();
        assert!(!parsed);
    }

    #[test]
    fn parse_bool_fail() {
        let mut chars = peekable("fAAA!!!");
        let parsed = parse_bool(&mut chars).err().unwrap();

        assert_eq!(parsed, BooleanParsingError);
    }

    #[test]
    #[allow(clippy::unit_cmp)]
    fn parse_null_happy() {
        let mut chars = peekable("null");

        #[allow(clippy::let_unit_value)]
        let parsed = parse_null(&mut chars).unwrap();

        assert_eq!((), parsed)
    }

    #[test]
    fn parse_null_fail() {
        let mut chars = peekable("NOT A NULL");
        let err = parse_null(&mut chars).err().unwrap();

        assert_eq!(InvalidJSON, err);
    }

    #[test]
    fn parse_string_happy() {
        let mut chars = peekable(r#""string""#);
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, "string")
    }

    #[test]
    fn parse_string_empty() {
        let mut chars = peekable(r#""""#);
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, "")
    }

    #[test]
    fn parse_string_quotes() {
        let mut chars = peekable(r#"" `\"` ""#);
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, " `\"` ")
    }

    #[test]
    fn parse_string_escapes_happy() {
        let mut chars = peekable(r#"" `\\` ""#);
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, " `\\` ")
    }

    #[test]
    fn parse_string_escapes_fail() {
        let mut chars = peekable(r#"" `\` ""#);
        let parsed = parse_string(&mut chars).err().unwrap();

        assert_eq!(parsed, InvalidEscapeCharacter)
    }

    #[test]
    fn parse_string_newline_happy() {
        let mut chars = peekable(r#"" newline! \n another line ""#);
        let parsed = parse_string(&mut chars).unwrap();

        assert_eq!(parsed, " newline! \n another line ")
    }

    #[test]
    fn parse_string_newline_fail() {
        let mut chars = peekable(
            r"newline!
            another line",
        );
        let parsed = parse_string(&mut chars).err().unwrap();

        assert_eq!(parsed, InvalidJSON)
    }

    #[test]
    fn parse_array_diverse() {
        let array = diverse_array();
        let mut chars = peekable(
            r#"[
                null,
                true,
                123,
                1234.56789,
                "Hello, world!"
            ]"#,
        );

        let parsed = parse_array(&mut chars).unwrap();

        assert_eq!(parsed, array);
    }

    #[test]
    fn parse_array_empty_happy() {
        let mut chars = peekable("[]");
        let array = vec![];

        let parsed = parse_array(&mut chars).unwrap();

        assert_eq!(parsed, array);
    }
}
