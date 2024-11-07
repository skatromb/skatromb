use crate::from_str::{
    Chars,
    ParseError::{self, *},
    Peekable,
};
use crate::JSON;

pub(super) fn parse_numeric(chars: &mut Peekable<Chars>) -> Result<JSON, ParseError> {
    let parser = NumericParser::new(chars);
    parser.parse()
}

struct NumericParser<'a, 'b> {
    chars: &'a mut Peekable<Chars<'b>>,
    collected: String,
    contains_fractional_part: bool,
}

/// super for parsing numeric string
impl<'a, 'b> NumericParser<'a, 'b> {
    /// Construct new parser with empty string and not yet found dot
    fn new(chars: &'a mut Peekable<Chars<'b>>) -> Self {
        Self {
            chars,
            collected: String::new(),
            contains_fractional_part: false,
        }
    }

    /// Main function that consumes numeric string
    fn parse(mut self) -> Result<JSON, ParseError> {
        if let Some('-') = self.peek() {
            self.process_minus()?
        }

        // if starts with 0
        if let Some('0') = self.peek() {
            self.process_leading_zero()?;
        } else {
            self.process_digits();
        }

        // if has fractional part
        if let Some('.') = self.peek() {
            self.process_dot()?;
            self.process_digits();
        }

        if !self.end_reached() {
            return Err(NumericParsingError);
        }

        if self.contains_fractional_part {
            if let Ok(float) = self.collected.parse() {
                return Ok(JSON::Float(float));
            }
        }

        if let Ok(int) = self.collected.parse() {
            return Ok(JSON::Int(int));
        }

        Err(NumericParsingError)
    }

    /// Looks up for the next char using `peek()`
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Consumes and returns next char using `next()`
    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Push char to `parsed`
    fn push(&mut self, char: char) {
        self.collected.push(char)
    }

    /// Check if end of number reached
    fn end_reached(&mut self) -> bool {
        match self.peek() {
            Some(&char) => char.is_whitespace() || char == ',',
            None => true,
        }
    }

    /// process '.'
    fn process_dot(&mut self) -> Result<(), ParseError> {
        if self.contains_fractional_part {
            return Err(NumericParsingError);
        }

        if let Some('.') = self.next() {
            self.contains_fractional_part = true;
            self.collected.push('.');

            if let Some(char) = self.next() {
                if char.is_ascii_digit() {
                    self.push(char);
                    return Ok(());
                }
            }
        }
        Err(NumericParsingError)
    }

    /// process '-'
    fn process_minus(&mut self) -> Result<(), ParseError> {
        if let Some('-') = self.peek() {
            let char = self.next().expect("Peeked so should exist");
            self.push(char);
            Ok(())
        } else {
            Err(NumericParsingError)
        }
    }

    /// process leading '0'
    fn process_leading_zero(&mut self) -> Result<(), ParseError> {
        if let Some('0') = self.peek() {
            let char = self.next().expect("peeked so should exist");
            self.push(char);
            Ok(())
        } else {
            Err(NumericParsingError)
        }
    }

    /// process rest digits in integer of fractional part
    fn process_digits(&mut self) {
        while let Some('0'..='9') = self.peek() {
            let char = self.next().expect("Peeked so should exist");
            self.push(char)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peekable(string: &str) -> Peekable<Chars> {
        string.chars().peekable()
    }

    #[test]
    fn parse_int_positive_happy() {
        let mut chars = peekable("1");
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Int(1))
    }

    #[test]
    fn parse_int_0_happy() {
        let mut chars = peekable("0");
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Int(0))
    }

    #[test]
    fn parse_int_negative_happy() {
        let mut chars = peekable("-123");
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Int(-123))
    }

    #[test]
    fn parse_int_starting_zero_fail() {
        let mut chars = peekable("01");
        let parsed = parse_numeric(&mut chars).err().unwrap();

        assert_eq!(parsed, NumericParsingError)
    }

    #[test]
    fn parse_float_positive_happy() {
        let mut chars = peekable("1.2");
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Float(1.2))
    }

    #[test]
    fn parse_float_starting_zero_happy() {
        let mut chars = peekable("0.010");
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Float(0.01))
    }

    #[test]
    fn parse_float_negative_happy() {
        let mut chars = peekable("-123.456");
        let parsed = parse_numeric(&mut chars).unwrap();

        assert_eq!(parsed, JSON::Float(-123.456))
    }

    #[test]
    fn parse_float_zero_zero_fail() {
        let mut chars = peekable("00.1");
        let parsed = parse_numeric(&mut chars).err().unwrap();

        assert_eq!(parsed, NumericParsingError)
    }

    #[test]
    fn parse_float_zero_number_fail() {
        let mut chars = peekable("01.0");
        let parsed = parse_numeric(&mut chars).err().unwrap();

        assert_eq!(parsed, NumericParsingError)
    }

    #[test]
    fn parse_float_multiple_dots_fail() {
        let mut chars = peekable("0.0.0");
        let parsed = parse_numeric(&mut chars);
        dbg!(&parsed); //.err().unwrap();

        // assert_eq!(parsed, NumericParsingError)
    }

    #[test]
    fn process_dot_happy() {
        let mut chars = peekable(".00");
        let mut parser = NumericParser::new(&mut chars);

        let result = parser.process_dot();

        assert!(result.is_ok());
        assert!(parser.contains_fractional_part);
        assert_eq!(parser.collected, ".0");
    }

    #[test]
    fn process_dot_happy_end() {
        let mut chars = peekable(".0");
        let mut parser = NumericParser::new(&mut chars);

        let result = parser.process_dot();

        assert!(result.is_ok());
        assert!(parser.contains_fractional_part);
        assert_eq!(parser.collected, ".0");
    }

    #[test]
    fn process_dot_end_fail() {
        let mut chars = peekable(".");
        let mut parser = NumericParser::new(&mut chars);

        let result = parser.process_dot();

        assert_eq!(result.unwrap_err(), NumericParsingError);
        assert!(parser.contains_fractional_part);
    }

    #[test]
    fn process_dot_comma_end_fail() {
        let mut chars = peekable(".,");
        let mut parser = NumericParser::new(&mut chars);

        let result = parser.process_dot();

        assert_eq!(result.unwrap_err(), NumericParsingError);
        assert!(parser.contains_fractional_part);
    }

    #[test]
    fn process_dot_two_comams_end_fail() {
        let mut chars = peekable(".0.");
        let mut parser = NumericParser::new(&mut chars);

        parser.process_dot().unwrap();
        let result = parser.process_dot();

        assert_eq!(result.unwrap_err(), NumericParsingError);
    }

    #[test]
    fn process_minus_happy() {
        let mut chars = peekable("-");
        let mut parser = NumericParser::new(&mut chars);

        parser.process_minus().unwrap();

        assert_eq!(parser.collected, "-");
        assert!(parser.chars.next().is_none());
    }

    #[test]
    fn process_minus_fail() {
        let mut chars = peekable("+");
        let mut parser = NumericParser::new(&mut chars);

        let error = parser.process_minus().unwrap_err();

        assert_eq!(error, NumericParsingError);
    }
}
