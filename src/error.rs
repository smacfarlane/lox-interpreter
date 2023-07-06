pub struct Error {
    line: usize,
    at: string,
    message: string,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Error{}: {}", self.line, self.at, self.message)
    }
}
