use crate::ast::Node;
use crate::error::ParseError;
use failure::Error;

pub mod syntax {
    include!(concat!(env!("OUT_DIR"), "/calculator.rs"));
}

pub fn parse(x: &str) -> Result<Node, Error> {
    syntax::expr(x).map_err(|e| {
        ParseError {
            message: e.to_string(),
        }
        .into()
    })
}

#[cfg(test)]
mod tests {
    use super::parse;
    #[test]
    fn skip_space() {
        assert_eq!(parse("1 +\n1 ").unwrap(), parse("1+1").unwrap())
    }
}
