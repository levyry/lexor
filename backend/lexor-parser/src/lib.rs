#![allow(clippy::missing_errors_doc)]

mod lambda;
mod ski;

pub use lambda::parse as parse_lambda;
pub use ski::parse as parse_ski;
use serde::{Deserialize, Serialize};

use thiserror::Error;

pub use lexor_core::combinator::Combinator;
pub use lexor_core::lambda::Lambda;
pub use lexor_core::literal::{ArithOp, Literal};

#[derive(Error, Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum ParsingError {
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Unexpected character: '{0}'")]
    UnexpectedChar(char),
    #[error("Expected '{0}', found '{1}'")]
    ExpectedChar(char, char),
    #[error("Invalid syntax")]
    InvalidSyntax,
}

type Result<T> = std::result::Result<T, ParsingError>;

const fn is_operator(c: char) -> bool {
    ArithOp::from_char(c).is_some()
}
