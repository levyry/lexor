use crate::lambda_expr::LambdaExpr;
use chumsky::extra;
use chumsky::prelude::*;

pub fn parse(input: &str) -> Result<LambdaExpr, Vec<Simple<'_, char>>> {
    fn parser<'a>() -> impl Parser<'a, &'a str, LambdaExpr, extra::Err<Simple<'a, char>>> {
        recursive(|expr| {
            let var = text::ident()
                .map(|s: &str| LambdaExpr::Var(s.to_string()))
                .padded();

            let atom = var
                .or(expr.clone().delimited_by(just('('), just(')')))
                .padded();

            let app = atom.clone().foldl(atom.repeated(), |lhs, rhs| {
                LambdaExpr::App(Box::new(lhs), Box::new(rhs))
            });

            let lambda = just('λ')
                .or(just('\\'))
                .ignore_then(text::ident().padded())
                .then_ignore(just('.'))
                .then(expr.padded())
                .map(|(arg, body): (&str, LambdaExpr)| {
                    LambdaExpr::Abs(arg.to_string(), Box::new(body))
                });

            lambda.or(app)
        })
        .then_ignore(end())
    }

    parser().parse(input).into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lambda_expr::LambdaExpr;

    macro_rules! var {
        ($s:expr) => {
            LambdaExpr::Var($s.to_string())
        };
    }
    macro_rules! abs {
        ($p:expr, $b:expr) => {
            LambdaExpr::Abs($p.to_string(), Box::new($b))
        };
    }
    macro_rules! app {
        ($l:expr, $r:expr) => {
            LambdaExpr::App(Box::new($l), Box::new($r))
        };
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(parse("x"), Ok(var!("x")));
        assert_eq!(parse("foo"), Ok(var!("foo")));
        assert_eq!(parse("  bar  "), Ok(var!("bar")));
    }

    #[test]
    fn test_parse_abstraction_backslash() {
        // \x. x
        let expected = abs!("x", var!("x"));
        assert_eq!(parse(r"\x. x"), Ok(expected));
    }

    #[test]
    fn test_parse_abstraction_lambda() {
        let expected = abs!("x", var!("x"));
        assert_eq!(parse("λx. x"), Ok(expected));
    }

    #[test]
    fn test_parse_abstraction_body_extends_right() {
        let expected = abs!("x", app!(var!("y"), var!("z")));
        assert_eq!(parse(r"\x. y z"), Ok(expected));
    }

    #[test]
    fn test_parse_application_simple() {
        let expected = app!(var!("x"), var!("y"));
        assert_eq!(parse("x y"), Ok(expected));
    }

    #[test]
    fn test_parse_application_left_associative() {
        let expected = app!(app!(var!("x"), var!("y")), var!("z"));
        assert_eq!(parse("x y z"), Ok(expected));
    }

    #[test]
    fn test_parse_parentheses() {
        let expected = app!(var!("x"), app!(var!("y"), var!("z")));
        assert_eq!(parse("x (y z)"), Ok(expected));
    }

    #[test]
    fn test_parse_complex_nested() {
        let func = abs!("x", app!(var!("x"), var!("x")));
        let arg = abs!("y", var!("y"));
        let expected = app!(func, arg);

        assert_eq!(parse(r"(\x. x x) (\y. y)"), Ok(expected));
    }

    #[test]
    fn test_parse_nested_abstractions() {
        let expected = abs!("x", abs!("y", app!(var!("x"), var!("y"))));
        assert_eq!(parse(r"\x. \y. x y"), Ok(expected));
    }

    #[test]
    fn test_parse_error_incomplete() {
        assert!(parse(r"\x.").is_err());
    }

    #[test]
    fn test_parse_error_unbalanced_parens() {
        assert!(parse("(x y").is_err());
        assert!(parse("x y)").is_err());
    }

    #[test]
    fn test_parse_error_invalid_char() {
        assert!(parse("123").is_err());
        assert!(parse("?").is_err());
    }
}
