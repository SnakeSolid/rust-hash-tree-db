use bincode::Error as BincodeError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum DatabaseError {
    CreateFileError { message: String },
    EncodingError { message: String },
    OpenFileError { message: String },
    SerializeError { message: String },
    InvalidRange { message: String },
}

impl DatabaseError {
    pub fn create_file_error(error: IoError) -> DatabaseError {
        DatabaseError::CreateFileError {
            message: format!("{}", error),
        }
    }

    pub fn encoding_error(error: IoError) -> DatabaseError {
        DatabaseError::EncodingError {
            message: format!("{}", error),
        }
    }

    pub fn open_file_error(error: IoError) -> DatabaseError {
        DatabaseError::OpenFileError {
            message: format!("{}", error),
        }
    }

    pub fn serialize_error(error: BincodeError) -> DatabaseError {
        DatabaseError::SerializeError {
            message: format!("{}", error),
        }
    }

    pub fn invalid_range<S>(message: S) -> DatabaseError
    where
        S: Into<String>,
    {
        DatabaseError::InvalidRange {
            message: message.into(),
        }
    }
}

impl Error for DatabaseError {}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            DatabaseError::CreateFileError { message } => write!(f, "{}", message),
            DatabaseError::EncodingError { message } => write!(f, "{}", message),
            DatabaseError::OpenFileError { message } => write!(f, "{}", message),
            DatabaseError::SerializeError { message } => write!(f, "{}", message),
            DatabaseError::InvalidRange { message } => write!(f, "{}", message),
        }
    }
}
