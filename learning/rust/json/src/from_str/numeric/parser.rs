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
    fn process_minus(&mut self) {}
}

// todo: write trait process zero integer

// todo: write trait process rest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_reached_fail() {
        let mut chars = "a".chars().peekable();
        let error = NumericParser::end_reached(chars.peek()).unwrap_err();

        assert_eq!(error, NumericParsingError);
    }

    #[test]
    fn end_reached_eof() {
        let mut chars = "".chars().peekable();
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(is_end);
    }

    #[test]
    fn end_reached_symbols_left() {
        let mut chars = "1".chars().peekable();
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(!is_end);
    }

    #[test]
    fn end_reached_whitespace() {
        let mut chars = " ".chars().peekable();
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(is_end);
    }

    #[test]
    fn end_reached_comma() {
        let mut chars = ",".chars().peekable();
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(is_end);
    }

    #[test]
    fn end_reached_dot() {
        let mut chars = ".".chars().peekable();
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(!is_end);
    }

    #[test]
    fn end_reached_digit() {
        let mut chars = "0".chars().peekable();
        let is_end = NumericParser::end_reached(chars.peek()).unwrap();

        assert!(!is_end);
    }
}
