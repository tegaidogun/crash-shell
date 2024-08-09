pub struct ShellError {
    message: String,
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Shell Error: {}", self.message)
    }
}
