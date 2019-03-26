use failure::Fail;

#[derive(Fail, Debug)]
#[fail(display = "Failed to parse: {}", message)]
pub struct ParseError {
    pub message: String,
}

#[derive(Fail, Debug)]
#[fail(display = "Failed to compile")]
pub struct CompilationError;
