#[derive(Debug)]
pub enum Error {
    ClassLoadingError(String),
    ClassNotFound(String),
}

//TODO TEMP
impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self::ClassLoadingError(io_error.to_string())
    }
}
