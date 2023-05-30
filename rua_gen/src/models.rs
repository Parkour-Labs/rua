//! This module contains the core traits to the rua crate. These traits
//! are to be implemented on the models from the [syn] crate. This is just
//! so that we will have a nice layer of abstraction between the models and
//! the ast.

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    rc::Rc,
};

use syn::Type;

use crate::{errors::RuaFsError, logic::Module};

/// Something that has attributes.
pub trait RuaHasAttr {
    // /// Returns [true] if the thing has the attribute specified.
    // fn has_attr<T: RuaAttr>(&self, attr: &T) -> bool {
    //     self.attrs().iter().any(|a| a.name() == attr.name())
    // }

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
    fn ty(&self) -> RuaType;
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
    fn ret_ty(&self) -> Option<RuaType>;
}

impl Debug for dyn RuaFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuaFn")
            .field("name", &self.name())
            .field("is_pub", &self.is_pub())
            .finish()
    }
}

/// A struct in Rust.
pub trait RuaStruct:
    RuaNamed + RuaVisible + RuaWithFields + RuaHasAttr
{
}

impl Debug for dyn RuaStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuaStruct")
            .field("name", &self.name())
            .field("is_pub", &self.is_pub())
            .finish()
    }
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

impl Debug for dyn RuaEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuaEnum")
            .field("name", &self.name())
            .field("is_pub", &self.is_pub())
            .finish()
    }
}

/// A variant of an enum in Rust.
pub trait RuaEnumVariant: RuaNamed + RuaWithFields {
    /// Returns [true] if the variant is a unit variant.
    fn is_unit(&self) -> bool;
}

/// A module in Rust.
pub trait RuaMod: RuaNamed + RuaVisible {}

impl Debug for dyn RuaMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuaMod")
            .field("name", &self.name())
            .field("is_pub", &self.is_pub())
            .finish()
    }
}

/// An attribute in Rust. This can be scanned for and used as a filter.
/// In the cause of `rua`, this will be set to `[rua]` from the `rua_annot`
/// crate.
pub trait RuaAttr: RuaNamed {}

impl RuaNamed for &str {
    fn name(&self) -> String {
        self.to_string()
    }
}

impl RuaAttr for &str {}

/// Represents a type in Rust.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum RuaType {
    /// Represents the 8-bit signed integer type [`i8`].
    I8,
    /// Represents the 16-bit signed integer type [`i16`].
    I16,
    /// Represents the 32-bit signed integer type [`i32`].
    I32,
    /// Represents the 64-bit signed integer type [`i64`].
    I64,
    /// Represents the 128-bit signed integer type [`i128`].
    I128,
    /// Represents the 8-bit unsigned integer type [`u8`].
    U8,
    /// Represents the 16-bit unsigned integer type [`u16`].
    U16,
    /// Represents the 32-bit unsigned integer type [`u32`].
    U32,
    /// Represents the 64-bit unsigned integer type [`u64`].
    U64,
    /// Represents the 128-bit unsigned integer type [`u128`].
    U128,
    /// Represents the 32-bit floating point type [`f32`].
    F32,
    /// Represents the 64-bit floating point type [`f64`].
    F64,
    /// Represents the [`bool`] type.
    Bool,
    /// Represents the [`char`] type.
    Char,
    /// Represents the [`str`] type.
    Str,
    /// Represents the [`String`] type.
    String,
    /// Represents the slice type [`&[T]`].
    Slice(Box<RuaType>),
    /// Represents the array type [`[T; N]`].
    Array(Box<RuaType>, usize),
    /// Represents the tuple type [`(T1, T2, ..., Tn)`].
    Tuple(Vec<RuaType>),
    /// Represents a struct type.
    Struct(Rc<dyn RuaStruct>),
    /// Represents an enum type.
    Enum(Rc<dyn RuaEnum>),
    /// Represents a function type.
    Pointer {
        /// Whether if the pointer is a const pointer. If this is not, then it
        /// is a mutable pointer.
        is_const: bool,
        /// The type of the pointer.
        ty: Box<RuaType>,
    },
    /// Represents a reference type.
    Reference(
        /// Whether if the reference is a mutable reference. If this is not,
        /// then it is an immutable reference.
        bool,
        /// The type of the reference.
        Box<RuaType>,
    ),
    /// Represents a function type.
    Fn(Rc<dyn RuaFn>),
    /// Represents a generic type.
    Unit,
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
        std::fs::read_to_string(path).map_err(|e| RuaFsError::ReadFileErr {
            path: path.to_path_buf(),
            err: Box::new(e),
        })
    }

    /// Whether if the item should be included in the scan. Note that if the
    /// item is not public, it will not be passed to this layer for scanning.
    fn should_include<T: RuaHasAttr>(&self, item: &T) -> bool {
        item.attrs().iter().any(|a| a.name() == "rua")
    }

    /// Generates and writes the function.
    fn write_fn<T: RuaFn>(&mut self, m: &Module, f: &T);

    /// Generates and writes the struct.
    fn write_struct<T: RuaStruct>(&mut self, m: &Module, s: &T);

    /// Generates and writes the enum.
    fn write_enum<T: RuaEnum>(&mut self, m: &Module, e: &T);
}
