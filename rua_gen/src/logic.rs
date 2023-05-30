//! This module contains the logic for the code generator.
use std::path::{Path, PathBuf};

use crate::{
    errors::RuaFsError,
    models::{RuaEnum, RuaFn, RuaMod, RuaStruct},
};

/// Implement this trait to build your own code generator.
pub trait Rua {
    /// Returns the path to the entry point of the module, i.e. the path to the
    /// folder containing the module.
    fn entry_path(&self) -> PathBuf;

    /// Checks if the path is a directory.
    fn is_dir(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        path.is_dir()
    }

    /// Checks if the file at the path is a file.
    fn is_file(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        path.is_file()
    }

    /// Checks if the file at the path exists.
    fn path_exists(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        path.exists()
    }

    /// Reads the file at the path specified. This is here just so that we
    /// don't have to rely on the [std::fs] module. It will make it easier
    /// for us to test things out.
    fn read_file(&self, path: impl AsRef<Path>) -> Result<String, RuaFsError> {
        let path = path.as_ref();
        std::fs::read_to_string(path).map_err(|e| RuaFsError::ReadFileErr {
            path: path.to_path_buf(),
            err: Box::new(e),
        })
    }

    /// Generates and writes the function.
    fn write_fn(&mut self, m: &RuaMod, f: &RuaFn);

    /// Generates and writes the struct.
    fn write_struct(&mut self, m: &RuaMod, s: &RuaStruct);

    /// Generates and writes the enum.
    fn write_enum(&mut self, m: &RuaMod, e: &RuaEnum);
}
