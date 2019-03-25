use crate::parser;
use failure::Error;

pub fn compile_string(source: &str) -> Result<CompilationResult, Error> {
    let ast = parser::parse(&source)?;
}
