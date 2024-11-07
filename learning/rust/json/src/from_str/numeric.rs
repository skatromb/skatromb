use crate::from_str::{
    Chars,
    ParseError::{self, *},
    Peekable,
};
use crate::JSON;

mod parser;

pub(crate) fn parse_numeric(chars: &mut Peekable<Chars>) -> Result<JSON, ParseError> {
    dbg!(chars);
    unimplemented!();
    // let mut parsed = String::new();
    // let mut dot_found = false;
    // let mut done = false;

    // let mut parser = NumericParser { chars, parsed };
    // parser.end_reached()?;

    // // consume '-' if exists
    // if let Some('-') = chars.peek() {
    //     let char = chars.next().expect("Peeked so should exist");
    //     num_str.push(char);
    // }

    // // started with '0', should be followed only by '.'
    // let char = chars.peek();
    // match char {
    //     Some(&'0') => {
    //         let char = chars.next().expect("Peeked so should exist");
    //         num_str.push(char);
    //         unimplemented!();
    // match chars.peek() {
    //     dot_found = true;
    //     // TODO: AAAAA
    //     let char = chars.next().expect("Peeked so should exist");
    //     num_str.push(char);
    // } else {
    //     return Err(NumericParsingError);
    // }
    //     }
    //     _ => {}
    // }

    // parse other chars, rely on `.parse()` errors for double '.' and other errors
    // if !done {
    //     loop {
    //         let char = chars.peek();

    //         match char {
    //             Some('0'..='9') => {
    //                 let char = chars.next().expect("Peeked so should exist");
    //                 num_str.push(char);
    //             }
    //             Some('.') => {
    //                 if dot_found {
    //                     return Err(NumericParsingError);
    //                 } else {
    //                     dot_found = true;

    //                     let char = chars.next().expect("Peeked so should exist");
    //                     num_str.push(char);
    //                 }
    //             }
    //             Some(char) => {
    //                 if char.is_whitespace() || *char == ',' {
    //                     break;
    //                 }
    //             }
    //             _ => break,
    //         }
    //     }
    // }

    // if dot_found {
    //     if let Ok(float) = num_str.parse() {
    //         return Ok(JSON::Float(float));
    //     }
    // } else if let Ok(int) = num_str.parse() {
    //     return Ok(JSON::Int(int));
    // }

    // Err(NumericParsingError)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
