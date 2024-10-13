use crate::JSON;
use std::collections::HashMap;
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

struct Parser<'a> {
    chars: Chars<'a>,
}

#[derive(Debug)]
pub struct ParseError;

    
fn parse<'a>(string: &'a str) -> Result<JSON, &'static str> {
    let mut chars = string.chars();
    
    match chars.find(|ch| !ch.is_whitespace())
    {
        Some(NULL_STARTS) => {
            parse_null(&mut chars).expect("Expected `null`, `true` or `false`");
            Ok(JSON::Null)
        },
        
        Some(STRING_STARTS) => {
            let string = parse_string(&mut chars);
            Ok(JSON::String(string))
        },
        
        _ => Err("String does not contain JSON"),
    }
}

fn parse_null(chars: &mut Chars) -> Result<(), ParseError> {
    if chars.take(3).eq(NULL_REST) {
        Ok(())
    } else { Err(ParseError) }
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
    'parsing: loop {
        let char = chars.next();
        let mut string = String::new();
        
        match char {
            None => return Err(ParseError),
            _ => {},
        }
    }
}

impl FromStr for JSON {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).map_err(|_| ParseError)
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
    fn parse_null() {
        let mut chars = "ull".chars();
        let parsed = super::parse_null(&mut chars).unwrap();
        
        assert_eq!((), parsed)
        
    }
}