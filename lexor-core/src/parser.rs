use crate::lambda::Lambda;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char),
    UnexpectedEOF,
    Expected(char),
}

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    // Parses: x, (expr), \x. expr
    fn parse_atom(&mut self) -> Result<Lambda, ParseError> {
        self.consume_whitespace();
        match self.chars.peek() {
            Some(&'(') => {
                self.chars.next(); // eat (
                let expr = self.parse_expr()?;
                self.consume_whitespace();
                if self.chars.next() == Some(')') {
                    Ok(expr)
                } else {
                    Err(ParseError::Expected(')'))
                }
            }
            Some(&'\\' | &'λ') => self.parse_abs(),
            Some(&c) if c.is_alphabetic() => {
                let mut name = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        name.push(self.chars.next().expect("peeked"));
                    } else {
                        break;
                    }
                }
                Ok(Lambda::Var(name))
            }
            Some(&c) => Err(ParseError::UnexpectedChar(c)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_abs(&mut self) -> Result<Lambda, ParseError> {
        self.consume_whitespace();
        self.chars.next(); // eat \ or λ

        self.consume_whitespace();
        // Parse param name
        let Lambda::Var(param) = self.parse_atom()? else {
            return Err(ParseError::Expected('a')); // Expected identifier
        };

        self.consume_whitespace();
        // Allow dot '.' to be optional if you prefer, but standard is strict
        if self.chars.next() == Some('.') {
            let body = self.parse_expr()?;
            Ok(Lambda::Abs(param, Box::new(body)))
        } else {
            Err(ParseError::Expected('.'))
        }
    }

    // Parses: atom atom atom ...
    fn parse_expr(&mut self) -> Result<Lambda, ParseError> {
        let mut lhs = self.parse_atom()?;

        loop {
            self.consume_whitespace();
            match self.chars.peek() {
                None | Some(')') => break,
                _ => {
                    let rhs = self.parse_atom()?;
                    lhs = Lambda::App(Box::new(lhs), Box::new(rhs));
                }
            }
        }
        Ok(lhs)
    }
}

///
/// # Errors
///
pub fn parse(input: &str) -> Result<Lambda, ParseError> {
    let mut parser = Parser::new(input);
    parser.parse_expr()
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn playground() {

//     }
// }
