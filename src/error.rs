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

#[derive(Error, Debug, PartialEq)]
pub enum EvaluationError {
    // TODO: Interpreter? Object?
    #[error("operands must be numbers {0}")]
    Comparision(String),
    #[error("operand must be number")]
    Negation,
    #[error("cannot perform {0} on non-numeric values")]
    Arithmatic(String),
    #[error("cannot concatinate non-string with string")]
    StringConcatination,
}

impl std::fmt::Display for ErrorLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line: {}@{}", self.line, self.at)
    }
}
