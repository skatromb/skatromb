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
    BooleanParsingError,
    NumericParsingError,
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

fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<JSON, ParseError> {
    chars.skip_whitespaces();

    let result = match chars.peek() {
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

        _ => Err(InvalidJSON),
    };

    result
}

fn parse_bool(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<bool, ParseError> {
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

// todo: write trait 'number terminated' -> bool
// todo: write trait process '-'
// todo: write trait process '0'
// todo: write trait process '.'
// todo: write trait process rest

fn parse_numeric(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<JSON, ParseError> {
    let mut num_str = String::new();
    let mut dot_found = false;

    // consume '-' if exists
    if let Some('-') = chars.peek() {
        let char = chars.next().expect("Peeked so should exist");
        num_str.push(char);
    }

    // parse other chars, rely on `.parse()` errors for double '.' and other errors
    loop {
        let char = chars.peek();

        match char {
            Some('0'..='9') => {
                let char = chars.next().expect("Peeked so should exist");
                num_str.push(char);
            }
            Some('.') => {
                if dot_found {
                    return Err(NumericParsingError);
                } else {
                    dot_found = true;

                    let char = chars.next().expect("Peeked so should exist");
                    num_str.push(char);
                }
            }
            Some(char) => {
                if char.is_whitespace() || *char == ',' {
                    break;
                }
            }
            _ => break,
        }
    }

    if dot_found {
        if let Ok(float) = num_str.parse() {
            return Ok(JSON::Float(float));
        }
    } else if let Ok(int) = num_str.parse() {
        return Ok(JSON::Int(int));
    }

    Err(NumericParsingError)
}

fn parse_string(chars: &mut impl Iterator<Item = char>) -> Result<String, ParseError> {
    let mut string = String::new();

    if chars.next() != Some('"') {
        return Err(InvalidJSON);
    };

    loop {
        let char = chars.next().ok_or(UnclosedStringLiteral)?;
        dbg!(char);

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
    dbg!(&string);

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
    fn parse_int_positive_happy() {
        let mut chars = "1".chars().peekable();
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Int(1))
    }

    #[test]
    fn parse_int_0_happy() {
        let mut chars = "0".chars().peekable();
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Int(0))
    }

    #[test]
    fn parse_int_negative_happy() {
        let mut chars = "-123".chars().peekable();
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Int(-123))
    }

    #[test]
    fn parse_int_starting_zero_fail() {
        let mut chars = "01".chars().peekable();
        let parsed = parse_numeric(&mut chars).err().unwrap();

        assert_eq!(parsed, NumericParsingError)
    }

    #[test]
    fn parse_float_positive_happy() {
        let mut chars = "1.2".chars().peekable();
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Float(1.2))
    }

    #[test]
    fn parse_float_starting_zero_happy() {
        let mut chars = "0.010".chars().peekable();
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Float(0.01))
    }

    #[test]
    fn parse_float_negative_happy() {
        let mut chars = "-123.456".chars().peekable();
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Float(-123.456))
    }

    #[test]
    fn parse_float_zero_zero_fail() {
        let mut chars = "00.1".chars().peekable();
        let parsed = parse_numeric(&mut chars).err().unwrap();

        assert_eq!(parsed, NumericParsingError)
    }

    #[test]
    fn parse_float_zero_number_fail() {
        let mut chars = "01.0".chars().peekable();
        let parsed = parse_numeric(&mut chars).err().unwrap();

        assert_eq!(parsed, NumericParsingError)
    }

    #[test]
    fn parse_float_multiple_dots_fail() {
        let mut chars = "0.0.0".chars().peekable();
        let parsed = parse_numeric(&mut chars).err().unwrap();

        assert_eq!(parsed, NumericParsingError)
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
