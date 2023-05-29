//! This module contains the core traits to the rua crate. These traits
//! are to be implemented on the models from the [syn] crate. This is just
//! so that we will have a nice layer of abstraction between the models and
//! the ast.

use std::path::{Path, PathBuf};

use syn::Type;

use crate::errors::RuaFsError;

/// Something that has attributes.
pub trait RuaHasAttr {
    /// Returns [true] if the thing has the attribute specified.
    fn has_attr<T: RuaAttr>(&self, attr: T) -> bool {
        self.attrs().iter().any(|a| a.name() == attr.name())
    }

    /// The attributes of the thing.
    fn attrs(&self) -> Vec<&dyn RuaAttr>;
}

/// Something that has a name.
pub trait RuaNamed {
    /// Returns the name of the thing.
    fn name(&self) -> String;
}

/// Something that has an optional name.
pub trait RuaNamedOpt {
    /// Returns the name of the thing if it has one.
    fn name_opt(&self) -> Option<String>;
}

/// Something that is visible. This is used to filter out private things.
pub trait RuaVisible {
    /// Returns [true] if the thing is public.
    fn is_pub(&self) -> bool;
}

/// Something that has a type.
pub trait RuaTyped {
    /// Returns the type of the thing.
    fn ty(&self) -> &Type;
}

/// Something that has a type as a string.
pub trait RuaStrTyped {
    /// Returns the type of the thing.
    fn ty(&self) -> &str;
}

/// Somthing that has fields.
pub trait RuaWithFields {
    /// Returns the fields of the thing.
    fn field(&self) -> Vec<&dyn RuaVar>;
}

/// A variable in Rust. It is consisted of two parts: the name and the type.
///
/// The name is optional because some times, say, in the return type of a
/// function, the name is not needed.
pub trait RuaVar: RuaNamedOpt + RuaTyped {}

/// A function in Rust.
pub trait RuaFn: RuaNamed + RuaVisible + RuaHasAttr {
    /// The arguments of the function.
    fn fn_args(&self) -> Vec<&dyn RuaVar>;

    /// The return type of the function.
    fn ret_ty(&self) -> Option<&Type>;
}

/// A struct in Rust.
pub trait RuaStruct:
    RuaNamed + RuaVisible + RuaWithFields + RuaHasAttr
{
}

/// An enum in Rust.
pub trait RuaEnum: RuaNamed + RuaVisible + RuaHasAttr {
    /// Gets the variants of the enum.
    fn variants(&self) -> Vec<&dyn RuaEnumVariant>;
    /// Gets whether if the enum is purely a unit enum.
    fn is_unit(&self) -> bool {
        self.variants().iter().all(|v| v.is_unit())
    }
}

/// A variant of an enum in Rust.
pub trait RuaEnumVariant: RuaNamed + RuaWithFields {
    /// Returns [true] if the variant is a unit variant.
    fn is_unit(&self) -> bool;
}

/// A module in Rust.
pub trait RuaMod: RuaNamed + RuaVisible {}

/// An attribute in Rust. This can be scanned for and used as a filter.
/// In the cause of `rua`, this will be set to `[rua]` from the `rua_annot`
/// crate.
pub trait RuaAttr: RuaNamed {
    /// Returns the arguments of the attribute.
    fn args(&self) -> Vec<String> {
        log::info!("TODO: Implement RuaAttr::args");
        vec![]
    }
}

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
        std::fs::read_to_string(path).map_err(|e| RuaFsError {
            path: path.to_path_buf(),
            err: Box::new(e),
        })
    }

    /// Sets the module to be scanned.
    fn set_module<T: RuaMod>(&mut self, m: &T);

    /// Whether if the item should be included in the scan. Note that if the
    /// item is not public, it will not be passed to this layer for scanning.
    fn should_include<T: RuaHasAttr>(&self, item: &T) -> bool;

    /// Generates and writes the function.
    fn write_fn<T: RuaFn>(&mut self, f: &T);

    /// Generates and writes the struct.
    fn write_struct<T: RuaStruct>(&mut self, s: &T);

    /// Generates and writes the enum.
    fn write_enum<T: RuaEnum>(&mut self, e: &T);
}
