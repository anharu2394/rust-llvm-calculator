use crate::parser;
use failure::Error;

use inkwell::execution_engine::{ExecutionEngine, JitFunction};

pub fn compile_string(source: &str) -> Result<Option<JitFunction<SumFunc>>, Error> {
    let ast = parser::parse(&source)?;
    jit_compile(ast)
}

pub fn jit_compile(ast: Node) -> Option<JitFunction<SumFunc>> {}
