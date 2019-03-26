use failure::Fail;

#[derive(Fail, Debug)]
#[fail(display = "Failed to parse: {}", message)]
pub struct ParseError {
    pub message: String,
}

#[derive(Fail, Debug)]
#[fail(display = "Failed to create a execution engine: {}", llvm_message)]
pub struct CreateExecutionEngineError {
    pub llvm_message: String,
}
