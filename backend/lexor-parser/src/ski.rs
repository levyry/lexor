use std::{iter::Peekable, str::Chars};

use crate::{ArithOp, Combinator, Literal, ParsingError, Result, is_operator};

const VALID_CHARS: &str = "SKIBC([";

pub fn parse(input: &str) -> Result<Combinator> {
    let mut parser = SkiParser::new(input);
    let root = parser.parse_expr()?;
    parser.skip_whitespace();
    if let Some(&c) = parser.chars.peek() {
        return Err(ParsingError::UnexpectedChar(c));
    }
    Ok(root)
}

// Helpers to make AST construction less verbose
macro_rules! app {
    ($lhs:expr, $rhs:expr) => {
        Combinator::App(Box::new($lhs), Box::new($rhs))
    };
}

const fn numer(n: i64) -> Combinator {
    Combinator::Primitive(Literal::Num(n))
}

const fn oper(o: ArithOp) -> Combinator {
    Combinator::Primitive(Literal::Op(o))
}

struct SkiParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> SkiParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek()
            && c.is_whitespace()
        {
            self.chars.next();
        }
    }

    fn parse_expr(&mut self) -> Result<Combinator> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Combinator> {
        let mut lhs = self.parse_juxtaposition()?;

        self.skip_whitespace();

        while let Some(&c) = self.chars.peek()
            && let Some(op) = ArithOp::from_char(c)
            && let prec = op.precedence()
            && prec >= min_prec
        {
            self.chars.next();

            let next_min_prec = if op.is_right_associative() {
                prec
            } else {
                prec.saturating_add(1)
            };

            let rhs = self.parse_binary_expr(next_min_prec)?;

            lhs = app!(app!(oper(op), lhs), rhs);

            self.skip_whitespace();
        }

        Ok(lhs)
    }

    fn parse_juxtaposition(&mut self) -> Result<Combinator> {
        let mut lhs = self.parse_atom(true)?;
        self.skip_whitespace();

        while let Some(&c) = self.chars.peek()
            && !is_operator(c)
            && (VALID_CHARS.contains(c.to_ascii_uppercase()) || c.is_ascii_digit())
        {
            let rhs = self.parse_atom(false)?;
            self.skip_whitespace();
            lhs = app!(lhs, rhs);
        }

        Ok(lhs)
    }

    fn parse_atom(&mut self, allow_op: bool) -> Result<Combinator> {
        self.skip_whitespace();

        let &c = self.chars.peek().ok_or(ParsingError::UnexpectedEOF)?;

        match c.to_ascii_uppercase() {
            '(' | '[' => {
                self.chars.next();
                let expr = self.parse_expr()?;
                self.skip_whitespace();
                let closing = self.chars.next().ok_or(ParsingError::UnexpectedEOF)?;
                if (c == '(' && closing != ')') || (c == '[' && closing != ']') {
                    return Err(ParsingError::ExpectedChar(
                        if c == '(' { ')' } else { ']' },
                        closing,
                    ));
                }
                Ok(expr)
            }
            comb @ ('S' | 'K' | 'I' | 'B' | 'C') => {
                self.chars.next();
                Ok(match comb {
                    'S' => Combinator::S,
                    'K' => Combinator::K,
                    'I' => Combinator::I,
                    'B' => Combinator::B,
                    'C' => Combinator::C,
                    _ => unreachable!("match pattern"),
                })
            }
            d if d.is_ascii_digit() => {
                let n = self.consume_number()?;
                Ok(numer(n))
            }
            op if allow_op && is_operator(op) => {
                let op = self.consume_operator()?;
                Ok(oper(op))
            }
            _ => Err(ParsingError::UnexpectedChar(c)),
        }
    }

    fn consume_number(&mut self) -> Result<i64> {
        let mut num = String::new();
        while let Some(&c) = self.chars.peek()
            && c.is_ascii_digit()
        {
            num.push(c);
            self.chars.next();
        }
        num.parse::<i64>().map_err(|_| ParsingError::InvalidSyntax)
    }

    fn consume_operator(&mut self) -> Result<ArithOp> {
        let c = self.chars.next().ok_or(ParsingError::UnexpectedEOF)?;
        ArithOp::from_char(c).ok_or(ParsingError::UnexpectedChar(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_combinators() {
        assert_eq!(parse("S"), Ok(Combinator::S));
        assert_eq!(parse("k"), Ok(Combinator::K));
        assert_eq!(parse("I"), Ok(Combinator::I));
        assert_eq!(parse("b"), Ok(Combinator::B));
        assert_eq!(parse("C"), Ok(Combinator::C));
    }

    #[test]
    fn test_left_associativity() {
        assert_eq!(
            parse("SKI"),
            Ok(app!(app!(Combinator::S, Combinator::K), Combinator::I))
        );
    }

    #[test]
    fn test_parentheses_and_brackets() {
        assert_eq!(
            parse("S(KI)"),
            Ok(app!(Combinator::S, app!(Combinator::K, Combinator::I)))
        );
        assert_eq!(
            parse("S[KI]"),
            Ok(app!(Combinator::S, app!(Combinator::K, Combinator::I)))
        );
    }

    #[test]
    fn test_primitives_numbers_and_operators() {
        assert_eq!(parse("123"), Ok(numer(123)));

        assert_eq!(
            parse("+ 5 10"),
            Ok(app!(app!(oper(ArithOp::Add), numer(5)), numer(10)))
        );

        assert_eq!(
            parse("5 + 10"),
            Ok(app!(app!(oper(ArithOp::Add), numer(5)), numer(10)))
        );

        assert_eq!(
            parse("S + I"),
            Ok(app!(app!(oper(ArithOp::Add), Combinator::S), Combinator::I))
        );

        assert_eq!(
            parse("(6 * 7) + (20 - 5)"),
            Ok(app!(
                app!(
                    oper(ArithOp::Add),
                    app!(app!(oper(ArithOp::Mul), numer(6)), numer(7))
                ),
                app!(app!(oper(ArithOp::Sub), numer(20)), numer(5))
            ))
        );

        assert_eq!(
            parse("6 * 7 + 20 - 5"),
            Ok(app!(
                app!(
                    oper(ArithOp::Sub),
                    app!(
                        app!(
                            oper(ArithOp::Add),
                            app!(app!(oper(ArithOp::Mul), numer(6)), numer(7))
                        ),
                        numer(20)
                    )
                ),
                numer(5)
            ))
        );
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(
            parse("  S \n ( K \t I )  "),
            Ok(app!(Combinator::S, app!(Combinator::K, Combinator::I)))
        );
    }

    #[test]
    fn test_parsing_errors() {
        assert_eq!(parse("S("), Err(ParsingError::UnexpectedEOF));
        assert_eq!(parse("S(K]"), Err(ParsingError::ExpectedChar(')', ']')));
        assert_eq!(parse("S[K)"), Err(ParsingError::ExpectedChar(']', ')')));
        assert_eq!(parse("S x I"), Err(ParsingError::UnexpectedChar('x')));
        assert_eq!(parse(""), Err(ParsingError::UnexpectedEOF));
    }
}
