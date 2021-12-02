use std::fmt::{Display, Formatter};

/* Base overwriting error */

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

/* File reading error */

#[derive(Debug)]
pub struct FileReadError {
    message: String,
}

impl FileReadError {
    pub fn new(cause: &str) -> Self {
        FileReadError {
            message: format!("Cannot read page content from store: {}", cause),
        }
    }
}

impl Display for FileReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FileReadError {
    fn description(&self) -> &str {
        &self.message
    }
}