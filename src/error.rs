use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct ErrorLoc {
    pub line: usize,
    pub at: usize,
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("unterminated string {0}")]
    UnterminatedString(ErrorLoc),
    #[error("unknown token type")]
    UnknownTokenType,
}

impl std::fmt::Display for ErrorLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line: {}@{}", self.line, self.at)
    }
}
