use lexor_core::de_bruijn::{DeBruijn, LambdaReductionStrategy};
use lexor_core::lambda::Lambda;
use lexor_parser::lambda_parser;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum LambdaEvalError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Conversion error: {0}")]
    ConversionError(String),
}

pub fn evaluate_lambda(
    input: &str,
    strat: LambdaReductionStrategy,
) -> Result<String, LambdaEvalError> {
    let parsed = lambda_parser::chumsky_parse(input)
        .map_err(|_| LambdaEvalError::ParseError(String::from("Invalid expression!")))?;

    let debruijn: DeBruijn = parsed.into();

    let reduced = debruijn.evaluate(strat);

    let result: Lambda = reduced
        .try_into()
        .map_err(LambdaEvalError::ConversionError)?;

    Ok(result.to_string())
}
