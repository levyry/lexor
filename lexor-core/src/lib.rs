#![allow(unused)]

use LambdaExpr::{Abs, App, Var};

#[derive(Clone, Debug)]
enum LambdaExpr {
    Var(String),
    Abs(String, Box<Self>),
    App(Box<Self>, Box<Self>),
}

impl LambdaExpr {
    fn eval(self, s: &str) -> Self {
        s.split_ascii_whitespace().fold(self, |acc, arg| {
            let argument = Var(arg.to_string());

            match acc {
                Abs(param, body) => body.substitute(&param, &argument),
                _ => App(Box::new(acc), Box::new(argument)),
            }
        })
    }

    fn substitute(self, var_name: &str, replacement: &Self) -> Self {
        match self {
            Var(ref name) => {
                if name == var_name {
                    replacement.clone()
                } else {
                    self
                }
            }

            Abs(ref name, ref original_expr) => {
                if name == var_name {
                    self
                } else {
                    let new_expr = original_expr.clone().substitute(var_name, replacement);
                    Abs(name.clone(), Box::new(new_expr))
                }
            }

            App(lhs, rhs) => {
                let new_lhs = lhs.substitute(var_name, replacement);
                let new_rhs = rhs.substitute(var_name, replacement);

                App(Box::new(new_lhs), Box::new(new_rhs))
            }
        }
    }

    fn print(self) -> String {
        match self {
            Self::Var(name) => name,
            Self::Abs(name, lambda_expr) => format!("λ{}.{}", name, lambda_expr.print()),
            Self::App(lhs, rhs) => format!("({}){}", lhs.print(), rhs.print()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let x = String::from("x");
        let y = String::from("y");

        let var = Var(y);
        let identity = Abs(x.clone(), Box::new(Var(x)));
        let app = App(Box::new(identity.clone()), Box::new(var.clone()));

        assert_eq!(String::from("y"), var.print());
        assert_eq!(String::from("λx.x"), identity.print());
        assert_eq!(String::from("(λx.x)y"), app.print());
    }

    #[test]
    fn i_combinator() {
        let x = String::from("x");

        let i_combinator = Abs(x.clone(), Box::new(Var(x)));

        assert_eq!(String::from("y"), i_combinator.eval("y").print());
    }

    #[test]
    fn k_combinator() {
        let x = String::from("x");
        let y = String::from("y");

        let k_combinator = Abs(x.clone(), Box::new(Abs(y, Box::new(Var(x)))));

        let result = k_combinator.eval("a b");
        assert_eq!(String::from("a"), result.print());
    }

    #[test]
    fn shadowing_correctness() {
        let x = String::from("x");
        let expr = Abs(x.clone(), Box::new(Abs(x.clone(), Box::new(Var(x)))));

        let result = expr.eval("y");

        assert_eq!(String::from("λx.x"), result.print());
    }
}
