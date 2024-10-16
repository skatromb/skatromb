use crate::JSON;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::str::Chars;

mod keychars {
    pub(super) const NULL_STARTS: char = 'n';
    pub(super) const NULL_REST: [char; 3] = ['u', 'l', 'l'];
    
    pub(super) const STRING_STARTS: char = '"';
    pub(super) const STRING_ENDS:char = STRING_STARTS;
    
    pub(super) const OBJECT_STARTS: char = '{';
    pub(super) const OBJECT_ENDS:char = '}';
}

use keychars::*;

const GENERIC_ERROR: &str = "JSON couldn't be parsed";

#[derive(Debug)]
pub enum ParseError {
    InvalidJSON(&'static str),
    
}

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
   
fn parse(string: & str) -> Result<JSON, ParseError> {
    let mut chars = string.chars();
    
    match chars.find(|ch| !ch.is_whitespace())
    {
        Some(NULL_STARTS) => {
            parse_null(&mut chars)?;
            Ok(JSON::Null)
        },
        
        Some(STRING_STARTS) => {
            let string = parse_string(&mut chars);
            Ok(JSON::String(string))
        },
        
        Some(OBJECT_STARTS) => {
            let object = parse_object(&mut chars)?;
            Ok(JSON::Object(object))
        }
        
        _ => Err(ParseError::InvalidJSON("Incorrect symbol in JSON")),
    }
}

fn parse_null(chars: &mut Chars) -> Result<(), ParseError> {
    if chars.take(3).eq(NULL_REST) {
        Ok(())
    } else { Err(ParseError::InvalidJSON("Error while parsing `null`")) }
}

fn parse_string(chars: &mut Chars) -> String {
    println!("{chars:?}");
    unimplemented!()
}

fn parse_integer(chars: &mut Chars) -> i64 {
    unimplemented!()
}

fn parse_float(chars: &mut Chars) -> f64 {
    unimplemented!()
}

fn parse_object(chars: &mut Chars) -> Result<HashMap<String, JSON>, ParseError> {
    unimplemented!();
    let char = chars.next();
    let mut string = String::new();
    
    match char {
        None => return Err(ParseError::InvalidJSON("Error while parsing object")),
        _ => {unimplemented!()},
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn parse_happy() {
        let json = JSON::Object(HashMap::from([
            ("key".to_string(), JSON::String("value".to_string()))
        ]));
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
        
        let parsed = parse_null(&mut chars).expect_err(GENERIC_ERROR);
    }
}