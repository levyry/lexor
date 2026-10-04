#![feature(iter_advance_by)]
pub mod de_bruijn;
pub mod literal;
mod name_generator;

pub use literal::{ArithOp, Literal};

pub mod lambda {
    use crate::{
        combinator::Combinator, de_bruijn::DeBruijn, literal::Literal,
        name_generator::NameGenerator,
    };
    use core::fmt;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Lambda {
        Var(String),
        Abs(String, Box<Self>),
        App(Box<Self>, Box<Self>),
        Primitive(Literal),
    }

    impl fmt::Display for Lambda {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Var(name) => write!(f, "{name}"),
                Self::Abs(name, lambda_expr) => write!(f, " \\{name}.{lambda_expr}"),
                Self::App(lhs, rhs) => {
                    if let Self::App(op_box, left) = &**lhs
                        && let Self::Primitive(Literal::Op(op)) = &**op_box
                    {
                        write!(f, "({left} {op} {rhs})")
                    } else if matches!(**rhs, Self::App(_, _)) {
                        write!(f, "({lhs})({rhs})")
                    } else {
                        write!(f, "({lhs}){rhs}")
                    }
                }
                Self::Primitive(p) => write!(f, "{p}"),
            }
        }
    }

    impl TryFrom<DeBruijn> for Lambda {
        type Error = String;
        fn try_from(value: DeBruijn) -> Result<Self, Self::Error> {
            fn convert(
                db: DeBruijn,
                scope: &mut Vec<String>,
                alphabet: &mut impl Iterator<Item = String>,
            ) -> Result<Lambda, String> {
                match db {
                    DeBruijn::BVar(index) => {
                        let name = scope
                            .iter()
                            .rev()
                            .nth(index)
                            .ok_or_else(|| "De Bruijn index out of bounds".to_owned())?
                            .clone();
                        Ok(Lambda::Var(name))
                    }
                    DeBruijn::FVar(name) => Ok(Lambda::Var(name)),
                    DeBruijn::Abs(body) => {
                        let name = alphabet.next().ok_or("Ran out of names")?;
                        scope.push(name.clone());
                        let body_lambda_expr = convert(*body, scope, alphabet)?;
                        scope.pop();
                        Ok(Lambda::Abs(name, Box::new(body_lambda_expr)))
                    }
                    DeBruijn::App(lhs, rhs) => Ok(Lambda::App(
                        Box::new(convert(*lhs, scope, alphabet)?),
                        Box::new(convert(*rhs, scope, alphabet)?),
                    )),
                    DeBruijn::Primitive(p) => Ok(Lambda::Primitive(p)),
                }
            }
            convert(value, &mut Vec::new(), &mut NameGenerator::new())
        }
    }

    impl From<Combinator> for Lambda {
        fn from(value: Combinator) -> Self {
            macro_rules! var {
                ($name:ident) => {
                    Lambda::Var(stringify!($name).to_string())
                };
            }
            macro_rules! abs {
                ($arg:ident, $body:expr) => {
                    Lambda::Abs(stringify!($arg).to_string(), Box::new($body))
                };
            }
            macro_rules! app {
                ($lhs:expr, $rhs:expr) => {
                    Lambda::App(Box::new($lhs), Box::new($rhs))
                };
            }

            match value {
                Combinator::S => abs!(
                    x,
                    abs!(
                        y,
                        abs!(z, app!(app!(var!(x), var!(z)), app!(var!(y), var!(z))))
                    )
                ),
                Combinator::K => abs!(x, abs!(y, var!(x))),
                Combinator::I => abs!(x, var!(x)),
                Combinator::B => abs!(x, abs!(y, abs!(z, app!(var!(x), app!(var!(y), var!(z)))))),
                Combinator::C => abs!(x, abs!(y, abs!(z, app!(app!(var!(x), var!(z)), var!(y))))),
                Combinator::App(lhs, rhs) => app!((*lhs).into(), (*rhs).into()),
                Combinator::Primitive(p) => Self::Primitive(p),
            }
        }
    }
}

pub mod combinator {
    use core::fmt;
    use std::ops::BitAnd;

    use serde::{Deserialize, Serialize};

    use crate::literal::Literal;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Combinator {
        S,
        K,
        I,
        B,
        C,
        App(Box<Self>, Box<Self>),
        Primitive(Literal),
    }

    impl BitAnd for Combinator {
        type Output = Self;
        fn bitand(self, rhs: Self) -> Self::Output {
            Self::App(Box::new(self), Box::new(rhs))
        }
    }

    impl fmt::Display for Combinator {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::S => write!(f, "S"),
                Self::K => write!(f, "K"),
                Self::I => write!(f, "I"),
                Self::B => write!(f, "B"),
                Self::C => write!(f, "C"),
                Self::App(lhs, rhs) => {
                    if let Self::App(op_box, left) = &**lhs
                        && let Self::Primitive(Literal::Op(op)) = &**op_box
                    {
                        write!(f, "({left} {op} {rhs})")
                    } else if matches!(**rhs, Self::App(_, _)) {
                        write!(f, "{lhs}({rhs})")
                    } else {
                        write!(f, "{lhs}{rhs}")
                    }
                }
                Self::Primitive(p) => write!(f, "{p}"),
            }
        }
    }

    impl Combinator {
        #[must_use]
        pub fn church(n: u64) -> Self {
            match n {
                0 => Self::K & Self::I,
                1 => Self::I,
                _ => {
                    let succ =
                        |term: Self| (Self::S & (Self::S & (Self::K & Self::S) & Self::K)) & term;
                    let mut curr = Self::I;
                    for _ in 1..n {
                        curr = succ(curr);
                    }
                    curr
                }
            }
        }

        #[must_use]
        pub fn desugar_church(self) -> Self {
            match self {
                Self::Primitive(Literal::Num(n)) if n >= 0 => {
                    u64::try_from(n).ok().map_or(self, Self::church)
                }
                Self::App(lhs, rhs) => Self::App(
                    Box::new(lhs.desugar_church()),
                    Box::new(rhs.desugar_church()),
                ),
                other => other,
            }
        }
    }
}
