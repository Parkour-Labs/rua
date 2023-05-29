use std::{error::Error, path::PathBuf};

/// An error that occurs when parsing a file.
#[derive(Debug)]
pub enum RuaError {
    /// An error that occurs when reading a file.
    FsError(RuaFsError),
    /// An error that occurs when parsing a file.
    ParseError(ParseError),
}

impl std::fmt::Display for RuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuaError::FsError(e) => write!(f, "{}", e),
            RuaError::ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for RuaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RuaError::FsError(e) => Some(e),
            RuaError::ParseError(e) => Some(e),
        }
    }
}

/// An error that occurs when reading a file.
#[derive(Debug)]
pub enum RuaFsError {
    /// An error that occurs when reading a file.
    ReadFileErr { path: PathBuf, err: Box<dyn Error> },
    /// An error that occurs when reading a file.
    FileNotFoundErr(PathBuf),
}

impl std::fmt::Display for RuaFsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuaFsError::ReadFileErr { path, err } => {
                write!(f, "failed to read {:?}: {}", path, err)
            }
            RuaFsError::FileNotFoundErr(path) => {
                write!(f, "file not found: {:?}", path)
            }
        }
    }
}

impl Error for RuaFsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RuaFsError::ReadFileErr { err, .. } => Some(&**err),
            RuaFsError::FileNotFoundErr(_) => None,
        }
    }
}

/// An error that occurs when parsing a file.
#[derive(Debug)]
pub struct ParseError {
    /// The path to the file that caused the error.
    pub path: PathBuf,
    /// The error that occurred.
    pub err: Box<dyn Error>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.path, self.err)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.err)
    }
}
