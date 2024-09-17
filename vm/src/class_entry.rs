use std::{error::Error, fmt};
pub trait ClassEntry: fmt::Debug {
    fn resolve(&self, class_name: &str) -> Result<Option<Vec<u8>>, ClassLoadingError>;
}

#[derive(Debug)]
pub struct ClassLoadingError {
    message: String,
    source: Box<dyn Error>,
}
