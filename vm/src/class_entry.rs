use std::{error::Error, fmt};
pub trait ClassEntry: fmt::Debug {
    fn resolve(&self, class_name: &str) -> Result<Option<Vec<u8>>, ClassLoadingError>;
}

#[derive(Debug)]
pub struct ClassLoadingError {
    message: String,
    source: Box<dyn Error>,
}

impl ClassLoadingError {
    pub fn new(error: impl Error + 'static) -> Self {
        Self {
            message: error.to_string(),
            source: Box::new(error),
        }
    }
}

impl fmt::Display for ClassLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ClassLoadingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
