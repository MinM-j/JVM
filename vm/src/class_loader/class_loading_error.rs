use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ClassLoadingError {
    // Link errors
    LinkageError(String),
    VerifyError(String),
    ClassFormatError(String),
    UnsupportedClassVersionError(String),
    NoClassDefFoundError(String),
    IncompatibleClassChangeError(String),

    // Resolution errors
    ClassCircularityError(String),
    IllegalAccessError(String),
    NoSuchFieldError(String),
    NoSuchMethodError(String),
    InstantiationError(String),
    AbstractMethodError(String),

    // Runtime errors
    ClassNotFoundException(String),
    SecurityException(String),
    OutOfMemoryError(String),

    // Custom errors
    IoError(std::io::Error),
    ParseError(String),
    InvalidJarFile(String),
    NetworkError(String),

    Other(String),
}

impl fmt::Display for ClassLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClassLoadingError::LinkageError(msg) => write!(f, "LinkageError: {}", msg),
            ClassLoadingError::VerifyError(msg) => write!(f, "VerifyError: {}", msg),
            ClassLoadingError::ClassFormatError(msg) => write!(f, "ClassFormatError: {}", msg),
            ClassLoadingError::UnsupportedClassVersionError(msg) => {
                write!(f, "Unsupported class version error: {}", msg)
            }
            ClassLoadingError::NoClassDefFoundError(msg) => {
                write!(f, "No class definition found error: {}", msg)
            }
            ClassLoadingError::IncompatibleClassChangeError(msg) => {
                write!(f, "Incompatible class change error: {}", msg)
            }

            // Resolution errors
            ClassLoadingError::ClassCircularityError(msg) => {
                write!(f, "Class circularity error: {}", msg)
            }
            ClassLoadingError::IllegalAccessError(msg) => {
                write!(f, "Illegal access error: {}", msg)
            }
            ClassLoadingError::NoSuchFieldError(msg) => write!(f, "No such field error: {}", msg),
            ClassLoadingError::NoSuchMethodError(msg) => write!(f, "No such method error: {}", msg),
            ClassLoadingError::InstantiationError(msg) => write!(f, "Instantiation error: {}", msg),
            ClassLoadingError::AbstractMethodError(msg) => {
                write!(f, "Abstract method error: {}", msg)
            }

            // Runtime errors
            ClassLoadingError::ClassNotFoundException(msg) => {
                write!(f, "Class not found exception: {}", msg)
            }
            ClassLoadingError::SecurityException(msg) => write!(f, "Security exception: {}", msg),
            ClassLoadingError::OutOfMemoryError(msg) => write!(f, "Out of memory error: {}", msg),

            // Custom errors
            ClassLoadingError::IoError(err) => write!(f, "I/O error: {}", err),
            ClassLoadingError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ClassLoadingError::InvalidJarFile(msg) => write!(f, "Invalid JAR file: {}", msg),
            ClassLoadingError::NetworkError(msg) => write!(f, "Network error: {}", msg),

            ClassLoadingError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for ClassLoadingError {}
