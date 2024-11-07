use crate::from_str::{
    Chars,
    ParseError::{self, *},
    Peekable,
};
use crate::JSON;

struct NumericParser<'a, 'b> {
    chars: &'a mut Peekable<Chars<'b>>,
    parsed: String,
    dot_found: bool,
}

/// Struct for parsing numeric string
impl<'a, 'b> NumericParser<'a, 'b> {
    /// Construct new parser with empty string and not yet found dot
    fn new(chars: &'a mut Peekable<Chars<'b>>) -> Self {
        Self {
            chars,
            parsed: String::new(),
            dot_found: false,
        }
    }

    /// Check if number string terminated here
    fn end_reached(peeked_char: Option<&char>) -> Result<bool, ParseError> {
        match peeked_char {
            Some(&char) => {
                if char.is_ascii_digit() || char == '.' {
                    return Ok(false);
                } else if char.is_whitespace() || char == ',' {
                    return Ok(true);
                }
            }
            None => {
                return Ok(true);
            }
        }
        Err(NumericParsingError)
    }

    /// process '.'
    fn process_dot(&mut self) -> Result<(), ParseError> {
        if self.dot_found {
            return Err(NumericParsingError);
        }

        if let Some('.') = self.chars.next() {
            self.dot_found = true;
            self.parsed.push('.');

            if let Some(char) = self.chars.next() {
                if char.is_ascii_digit() {
                    self.parsed.push(char);
                    return Ok(());
                }
            }
        }
        Err(NumericParsingError)
    }

    /// process '-'
    fn process_minus(&mut self) -> Result<(), ParseError> {
        if let Some('-') = self.chars.peek() {
            let char = self.chars.next().expect("Peeked so should exist");
            self.parsed.push(char);
            Ok(())
        } else {
            Err(NumericParsingError)
        }
    }
}

// todo: write trait process zero integer

// todo: write trait process rest

#[cfg(test)]
mod tests {
    use super::*;

    fn peekable(string: &str) -> Peekable<Chars> {
        string.chars().peekable()
    }

    #[test]
    fn end_reached_fail() {
        let mut chars = peekable("a");
        let error = NumericParser::end_reached(chars.peek()).unwrap_err();

        assert_eq!(error, NumericParsingError);
    }

    #[test]
    fn end_reached_eof() {
        let mut chars = peekable("");
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(is_end);
    }

    #[test]
    fn end_reached_symbols_left() {
        let mut chars = peekable("1");
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(!is_end);
    }

    #[test]
    fn end_reached_whitespace() {
        let mut chars = peekable(" ");
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(is_end);
    }

    #[test]
    fn end_reached_comma() {
        let mut chars = peekable(",");
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(is_end);
    }

    #[test]
    fn end_reached_dot() {
        let mut chars = peekable(".");
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(!is_end);
    }

    #[test]
    fn end_reached_digit() {
        let mut chars = peekable("0");
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(!is_end);
    }

    #[test]
    fn process_dot_happy() {
        let mut chars = peekable(".00");
        let mut parser = NumericParser::new(&mut chars);

        let result = parser.process_dot();

        assert!(result.is_ok());
        assert!(parser.dot_found);
        assert_eq!(parser.parsed, ".0");
    }

    #[test]
    fn process_dot_happy_end() {
        let mut chars = peekable(".0");
        let mut parser = NumericParser::new(&mut chars);

        let result = parser.process_dot();

        assert!(result.is_ok());
        assert!(parser.dot_found);
        assert_eq!(parser.parsed, ".0");
    }

    #[test]
    fn process_dot_end_fail() {
        let mut chars = peekable(".");
        let mut parser = NumericParser::new(&mut chars);

        let result = parser.process_dot();

        assert_eq!(result.unwrap_err(), NumericParsingError);
        assert!(parser.dot_found);
    }

    #[test]
    fn process_dot_comma_end_fail() {
        let mut chars = peekable(".,");
        let mut parser = NumericParser::new(&mut chars);

        let result = parser.process_dot();

        assert_eq!(result.unwrap_err(), NumericParsingError);
        assert!(parser.dot_found);
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

        assert_eq!(parser.parsed, "-");
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
