use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Literal {
    Num(i64),
    Op(ArithOp),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Num(n) => write!(f, "{n}"),
            Self::Op(op) => write!(f, "{op}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Lt,
    Gt,
}

impl ArithOp {
    #[must_use]
    pub const fn arity(self) -> usize {
        2
    }

    #[must_use]
    pub const fn precedence(self) -> u8 {
        match self {
            Self::Eq | Self::Lt | Self::Gt => 1,
            Self::Add | Self::Sub => 2,
            Self::Mul | Self::Div => 3,
            Self::Pow => 4,
        }
    }

    #[must_use]
    pub const fn is_right_associative(self) -> bool {
        matches!(self, Self::Pow)
    }

    #[must_use]
    pub fn apply(self, a: i64, b: i64) -> Option<i64> {
        match self {
            Self::Add => a.checked_add(b),
            Self::Sub => a.checked_sub(b),
            Self::Mul => a.checked_mul(b),
            Self::Div => {
                if b == 0 {
                    None
                } else {
                    a.checked_div(b)
                }
            }
            Self::Pow => u32::try_from(b).ok().and_then(|exp| a.checked_pow(exp)),
            Self::Eq => Some(i64::from(a == b)),
            Self::Lt => Some(i64::from(a < b)),
            Self::Gt => Some(i64::from(a > b)),
        }
    }

    #[must_use]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '^' => Some(Self::Pow),
            '=' => Some(Self::Eq),
            '<' => Some(Self::Lt),
            '>' => Some(Self::Gt),
            _ => None,
        }
    }
}

impl fmt::Display for ArithOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Pow => "^",
            Self::Eq => "=",
            Self::Lt => "<",
            Self::Gt => ">",
        })
    }
}
