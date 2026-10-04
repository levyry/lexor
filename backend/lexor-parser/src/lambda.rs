use std::{iter::Peekable, str::Chars};

use crate::{ArithOp, Lambda, Literal, ParsingError, Result, is_operator};

pub fn parse(input: &str) -> Result<Lambda> {
    let mut parser = LambdaParser::new(input);
    let ast = parser.parse_expr()?;
    parser.skip_whitespace();
    if let Some(&c) = parser.chars.peek() {
        return Err(ParsingError::UnexpectedChar(c));
    }
    Ok(ast)
}

struct LambdaParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> LambdaParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Lambda> {
        self.skip_whitespace();
        if let Some(&c) = self.chars.peek()
            && (c == '\\' || c == 'λ')
        {
            return self.parse_abs();
        }
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Lambda> {
        let mut lhs = self.parse_app()?;

        loop {
            self.skip_whitespace();
            let Some(&c) = self.chars.peek() else { break; };
            let Some(op) = ArithOp::from_char(c) else { break; };

            let prec = op.precedence();
            if prec < min_prec {
                break;
            }

            self.chars.next();

            let next_min_prec = if op.is_right_associative() {
                prec
            } else {
                prec.saturating_add(1)
            };

            let rhs = self.parse_binary_expr(next_min_prec)?;
            lhs = Lambda::App(
                Box::new(Lambda::App(
                    Box::new(Lambda::Primitive(Literal::Op(op))),
                    Box::new(lhs),
                )),
                Box::new(rhs),
            );
        }

        Ok(lhs)
    }

    fn parse_abs(&mut self) -> Result<Lambda> {
        self.chars.next(); // Consume '\' or 'λ'
        self.skip_whitespace();

        let var = self.consume_ident()?;

        self.skip_whitespace();
        let dot = self.chars.next().ok_or(ParsingError::UnexpectedEOF)?;
        if dot != '.' {
            return Err(ParsingError::ExpectedChar('.', dot));
        }

        let body = self.parse_expr()?;
        Ok(Lambda::Abs(var, Box::new(body)))
    }

    fn parse_app(&mut self) -> Result<Lambda> {
        let mut lhs = self.parse_atom(true)?;

        loop {
            self.skip_whitespace();
            if let Some(&c) = self.chars.peek() {
                if is_operator(c) {
                    break;
                }
                if c.is_alphabetic() || c.is_ascii_digit() || c == '(' {
                    let rhs = self.parse_atom(false)?;
                    lhs = Lambda::App(Box::new(lhs), Box::new(rhs));
                    continue;
                }
            }
            break;
        }
        Ok(lhs)
    }

    fn parse_atom(&mut self, allow_op: bool) -> Result<Lambda> {
        self.skip_whitespace();
        let &c = self.chars.peek().ok_or(ParsingError::UnexpectedEOF)?;

        match c {
            '(' => {
                self.chars.next(); // Consume '('
                let expr = self.parse_expr()?;
                self.skip_whitespace();
                let closing = self.chars.next().ok_or(ParsingError::UnexpectedEOF)?;
                if closing != ')' {
                    return Err(ParsingError::ExpectedChar(')', closing));
                }
                Ok(expr)
            }
            d if d.is_ascii_digit() => {
                let n = self.consume_number()?;
                Ok(Lambda::Primitive(Literal::Num(n)))
            }
            op if allow_op && is_operator(op) => {
                let op = self.consume_operator()?;
                Ok(Lambda::Primitive(Literal::Op(op)))
            }
            alpha if alpha.is_alphabetic() => Ok(Lambda::Var(self.consume_ident()?)),
            _ => Err(ParsingError::UnexpectedChar(c)),
        }
    }

    fn consume_ident(&mut self) -> Result<String> {
        let mut id = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphabetic() || c.is_ascii_digit() || c == '_' {
                id.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        if id.is_empty() {
            Err(ParsingError::InvalidSyntax)
        } else {
            Ok(id)
        }
    }

    fn consume_number(&mut self) -> Result<i64> {
        let mut num = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        num.parse::<i64>().map_err(|_| ParsingError::InvalidSyntax)
    }

    fn consume_operator(&mut self) -> Result<ArithOp> {
        let c = self.chars.next().ok_or(ParsingError::UnexpectedEOF)?;
        ArithOp::from_char(c).ok_or(ParsingError::UnexpectedChar(c))
    }
}

#[cfg(test)]
mod lambda_tests {
    use super::*;

    macro_rules! app {
        ($lhs:expr, $rhs:expr) => {
            Lambda::App(Box::new($lhs), Box::new($rhs))
        };
    }
    fn var(s: &str) -> Lambda {
        Lambda::Var(s.to_owned())
    }
    fn abs(arg: &str, body: Lambda) -> Lambda {
        Lambda::Abs(arg.to_owned(), Box::new(body))
    }
    fn num(n: i64) -> Lambda {
        Lambda::Primitive(Literal::Num(n))
    }
    fn op(o: ArithOp) -> Lambda {
        Lambda::Primitive(Literal::Op(o))
    }

    #[test]
    fn test_variables() {
        assert_eq!(parse("x"), Ok(var("x")));
        assert_eq!(parse("foo_bar123"), Ok(var("foo_bar123")));
    }

    #[test]
    fn test_abstractions() {
        // Standard backslash
        assert_eq!(parse("\\x.x"), Ok(abs("x", var("x"))));
        // Greek lambda
        assert_eq!(parse("λfoo.foo"), Ok(abs("foo", var("foo"))));
    }

    #[test]
    fn test_nested_abstractions() {
        // \x.\y.x
        assert_eq!(parse("\\x.\\y.x"), Ok(abs("x", abs("y", var("x")))));
    }

    #[test]
    fn test_applications() {
        // x y z -> ((x y) z)
        assert_eq!(parse("x y z"), Ok(app!(app!(var("x"), var("y")), var("z"))));
    }

    #[test]
    fn test_parens_and_precedence() {
        // \x.(x y)
        assert_eq!(parse("\\x.(x y)"), Ok(abs("x", app!(var("x"), var("y")))));

        // (\x.x) y
        assert_eq!(parse("(\\x.x) y"), Ok(app!(abs("x", var("x")), var("y"))));
    }

    #[test]
    fn test_primitives_numbers_and_operators() {
        // Prefix
        assert_eq!(
            parse("+ 5 x"),
            Ok(app!(app!(op(ArithOp::Add), num(5)), var("x")))
        );

        // Infix
        assert_eq!(
            parse("5 + x"),
            Ok(app!(app!(op(ArithOp::Add), num(5)), var("x")))
        );

        // Abstraction with infix math: \x. x + 1
        assert_eq!(
            parse("\\x. x + 1"),
            Ok(abs("x", app!(app!(op(ArithOp::Add), var("x")), num(1))))
        );

        // Compound infix
        assert_eq!(
            parse("(6 * 7) + (20 - 5)"),
            Ok(app!(
                app!(
                    op(ArithOp::Add),
                    app!(app!(op(ArithOp::Mul), num(6)), num(7))
                ),
                app!(app!(op(ArithOp::Sub), num(20)), num(5))
            ))
        );
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(
            parse("  \\x  .  x  ( y  z )  "),
            Ok(abs("x", app!(var("x"), app!(var("y"), var("z")))))
        );
    }

    #[test]
    fn test_parsing_errors() {
        // Missing dot
        assert_eq!(parse("\\x x"), Err(ParsingError::ExpectedChar('.', 'x')));

        // Missing variable in abstraction
        assert_eq!(parse("\\.x"), Err(ParsingError::InvalidSyntax));

        assert_eq!(parse("(x y"), Err(ParsingError::UnexpectedEOF));
        assert_eq!(parse("(x y]"), Err(ParsingError::ExpectedChar(')', ']')));

        assert_eq!(parse(""), Err(ParsingError::UnexpectedEOF));
    }
}
