use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct BaseOverwriteError {
    message: String,
}

impl BaseOverwriteError {
    pub fn new(message: &str) -> Self {
        BaseOverwriteError {
            message: message.to_owned(),
        }
    }
}

impl Display for BaseOverwriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BaseOverwriteError {
    fn description(&self) -> &str {
        &self.message
    }
}