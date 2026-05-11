/*!
Crate for converting between different representations of SKI and Lambda terms.
*/

use lexor_core::{de_bruijn::DeBruijn, lambda::Lambda};
use lexor_parser::ski_parser::chumsky_parse as parse;

mod kiselyov;

pub fn convert_ski_to_lambda(str: &str) -> Result<String, String> {
    let ast = parse(str).map_err(|_| String::from("Syntax Error in ski->lambda conversion"))?;

    let raw_lambda: Lambda = ast.into();

    // For prettier names (each variable gets its own name) we convert to
    // DeBruijn and back to regular lambdas.
    let db: DeBruijn = raw_lambda.into();
    let clean_lambda: Lambda = db
        .try_into()
        .expect("We just converted to DeBruijn from Lambda, so converting back should always work");

    Ok(clean_lambda.to_string())
}
