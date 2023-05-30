//! Error types.

use std::{error::Error, path::PathBuf};

use proc_macro2::Span;

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

/// An error that occurs during a conversion.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ConversionError {
    /// The file that caused the error.
    path: Option<PathBuf>,
    /// The start of the error.
    start: Option<(usize, usize)>,
    /// The end of the error.
    end: Option<(usize, usize)>,
    /// The source type
    source_type: Option<String>,
    /// The target type.
    target_type: Option<String>,
    /// The message of this error.
    message: Option<String>,
    /// The source of this error.
    err_source: Option<Box<ConversionError>>,
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "{}", path.display())?;
        }
        if let Some((line, column)) = &self.start {
            write!(f, "(from {}:{}", line, column)?;
        } else {
            write!(f, "(")?;
        }
        if let Some((line, column)) = &self.end {
            write!(f, "to {}:{})", line, column)?;
        } else {
            write!(f, ")")?;
        }
        if let Some(source_type) = &self.source_type {
            write!(f, " from {}", source_type)?;
        }
        if let Some(target_type) = &self.target_type {
            write!(f, " to {}", target_type)?;
        }
        if let Some(message) = &self.message {
            write!(f, ": {}", message)?;
        }
        if let Some(err_source) = &self.err_source {
            write!(f, "\n- Caused by: {}", err_source)?;
        }
        Ok(())
    }
}

impl Error for ConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.err_source {
            Some(ref e) => Some(&**e),
            None => None,
        }
    }
}

/// A builder for a conversion error.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ConversionErrorBuilder {
    path: Option<PathBuf>,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
    source_type: Option<String>,
    target_type: Option<String>,
    message: Option<String>,
    err_source: Option<Box<ConversionError>>,
}

impl ConversionError {
    /// Creates a new conversion error builder.
    pub fn builder() -> ConversionErrorBuilder {
        ConversionErrorBuilder {
            path: None,
            start: None,
            end: None,
            source_type: None,
            target_type: None,
            err_source: None,
            message: None,
        }
    }

    /// Creates a new conversion error builder for the next error.
    pub fn builder_for_next(&self) -> ConversionErrorBuilder {
        ConversionErrorBuilder {
            path: None,
            start: None,
            end: None,
            source_type: None,
            target_type: None,
            message: None,
            err_source: Some(Box::new(self.clone())),
        }
    }
}

impl ConversionErrorBuilder {
    /// Sets the path to the file that caused the error.
    pub fn path(&mut self, path: PathBuf) -> &mut Self {
        self.path = Some(path);
        self
    }

    /// Sets the start of the error.
    pub fn start(&mut self, location: (usize, usize)) -> &mut Self {
        self.start = Some(location);
        self
    }

    /// Sets the end of the error.
    pub fn end(&mut self, location: (usize, usize)) -> &mut Self {
        self.end = Some(location);
        self
    }

    /// Sets the span of the error.
    pub(crate) fn span(&mut self, span: &Span) -> &mut Self {
        let start = span.start();
        let end = span.end();
        self.start = Some((start.line, start.column));
        self.end = Some((end.line, end.column));
        self
    }

    /// Sets the source type.
    pub fn source_type(&mut self, source_type: impl AsRef<str>) -> &mut Self {
        self.source_type = Some(source_type.as_ref().to_owned());
        self
    }

    /// Sets the target type.
    pub fn target_type(&mut self, target_type: impl AsRef<str>) -> &mut Self {
        self.target_type = Some(target_type.as_ref().to_owned());
        self
    }

    /// Sets the message of this error.
    pub fn message(&mut self, message: impl AsRef<str>) -> &mut Self {
        self.message = Some(message.as_ref().to_owned());
        self
    }

    /// Builds the conversion error.
    pub fn build(&mut self) -> ConversionError {
        let error = ConversionError {
            path: self.path.take(),
            start: self.start.take(),
            end: self.end.take(),
            source_type: self.source_type.take(),
            target_type: self.target_type.take(),
            err_source: self.err_source.take(),
            message: self.message.take(),
        };
        log::debug!("ConversionError: {}", error);
        error
    }
}
