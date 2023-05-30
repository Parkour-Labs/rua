//! Error types.

use std::{error::Error, path::PathBuf};

/// An error that occurs when parsing a file.
#[derive(Debug)]
pub enum RuaError {
    /// An error that occurs when reading a file.
    FsError(RuaFsError),
    /// An error that occurs when parsing a file.
    ParseError(ParseError),
    /// An error that occurs during a conversion.
    ConversionError(ConversionError),
}

impl std::fmt::Display for RuaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuaError::FsError(e) => write!(f, "{}", e),
            RuaError::ParseError(e) => write!(f, "{}", e),
            RuaError::ConversionError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for RuaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RuaError::FsError(e) => Some(e),
            RuaError::ParseError(e) => Some(e),
            RuaError::ConversionError(e) => Some(e),
        }
    }
}

/// An error that occurs when reading a file.
#[derive(Debug)]
pub enum RuaFsError {
    /// An error that occurs when reading a file.
    ReadFileErr {
        /// The path to the file that caused the error.
        path: PathBuf,
        /// The error that occurred.
        err: Box<dyn Error>,
    },
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

#[derive(Debug)]
pub struct ConversionError {
    /// The line number that caused the error.
    pub line: usize,
    /// The column number that caused the error.
    pub column: usize,
    /// The source type.
    pub source_type: Option<String>,
    /// The target type.
    pub target_type: Option<String>,
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(line {}, column {}): failed to convert",
            self.line, self.column
        )?;
        if let Some(source_type) = &self.source_type {
            write!(f, " from {}", source_type)?;
        }
        if let Some(target_type) = &self.target_type {
            write!(f, " to {}", target_type)?;
        }
        Ok(())
    }
}

impl Error for ConversionError {}
