//! The models used by `rua_gen`.
use std::path::PathBuf;

use crate::errors::ConversionError;
use rua_macros::rua_model_derive;

pub use rua_name::*;

/// Types related to names.
mod rua_name {
    use std::fmt::Display;

    use super::*;

    trait RuaCased {
        fn is_snake_case(&self) -> bool;
        fn is_camel_case(&self) -> bool;
        fn is_pascal_case(&self) -> bool;
        fn to_snake_case(&self) -> String;
        fn to_camel_case(&self) -> String;
        fn to_pascal_case(&self) -> String;
    }

    impl<T: AsRef<str>> RuaCased for T {
        fn is_snake_case(&self) -> bool {
            let s = self.as_ref();
            if s.is_empty() {
                return false;
            }
            s.chars().all(|c| c.is_ascii_lowercase() || c == '_')
        }

        fn is_camel_case(&self) -> bool {
            let s = self.as_ref();
            if s.is_empty() {
                return false;
            }
            if s.is_snake_case() {
                return false;
            }
            match s.chars().next() {
                Some(val) => val.is_ascii_lowercase(),
                None => false,
            }
        }

        fn is_pascal_case(&self) -> bool {
            let s = self.as_ref();
            if s.is_empty() {
                return false;
            }
            if s.is_snake_case() {
                return false;
            }
            match s.chars().next() {
                Some(val) => val.is_ascii_uppercase(),
                None => false,
            }
        }

        fn to_snake_case(&self) -> String {
            let s = self.as_ref();
            if s.is_empty() {
                return String::new();
            }
            let mut chars = s.chars();
            let first = chars.next();
            // just to be safe
            if first.is_none() {
                return String::new();
            }
            let first = first.unwrap();
            let rest = chars
                .map(|c| {
                    if c.is_ascii_uppercase() {
                        format!("_{}", c.to_ascii_lowercase())
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>();
            format!("{}{}", first.to_ascii_lowercase(), rest)
        }

        fn to_camel_case(&self) -> String {
            let s = self.as_ref();
            if s.is_empty() {
                return String::new();
            }
            let chars = s.chars();
            let mut res = String::new();
            let mut prev_is_dash = false;
            for (i, c) in chars.enumerate() {
                if i == 0 {
                    res.push(c.to_ascii_lowercase());
                    continue;
                }
                if c == '_' {
                    prev_is_dash = true;
                    continue;
                }
                if prev_is_dash {
                    res.push(c.to_ascii_uppercase());
                    prev_is_dash = false;
                    continue;
                }
                res.push(c);
            }
            res
        }

        fn to_pascal_case(&self) -> String {
            let camel = self.to_camel_case();
            let first = camel.chars().next();
            if first.is_none() {
                return String::new();
            }
            let first = first.unwrap();
            format!("{}{}", first.to_ascii_uppercase(), &camel[1..])
        }
    }

    /// Represents something that has a name.
    pub trait RuaNamed {
        /// Returns the name of the item.
        fn name(&self) -> &RuaName;
    }

    /// Represents a name with a case.
    #[rua_model_derive]
    pub struct RuaName {
        name: String,
        case: RuaCase,
    }

    impl Display for RuaName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}({})", self.case, self.name)
        }
    }

    /// The case of a name.
    #[rua_model_derive]
    pub enum RuaCase {
        /// snake_case
        SnakeCase,
        /// camelCase
        CamelCase,
        /// PascalCase
        PascalCase,
    }

    impl Display for RuaCase {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                RuaCase::SnakeCase => write!(f, "snake_case"),
                RuaCase::CamelCase => write!(f, "camelCase"),
                RuaCase::PascalCase => write!(f, "PascalCase"),
            }
        }
    }

    impl RuaCase {
        /// Converts a string to the case.
        pub fn convert(&self, s: impl AsRef<str>) -> String {
            match self {
                RuaCase::SnakeCase => s.to_snake_case(),
                RuaCase::CamelCase => s.to_camel_case(),
                RuaCase::PascalCase => s.to_pascal_case(),
            }
        }

        /// Checks if a string is in the case.
        pub fn check(&self, s: impl AsRef<str>) -> bool {
            match self {
                RuaCase::SnakeCase => s.is_snake_case(),
                RuaCase::CamelCase => s.is_camel_case(),
                RuaCase::PascalCase => s.is_pascal_case(),
            }
        }
    }

    impl RuaName {
        /// Creates a new name.
        pub fn new(name: impl AsRef<str>, case: RuaCase) -> Self {
            if !case.check(name.as_ref()) {
                log::warn!("{} is not a valid {} name", name.as_ref(), case);
            }
            Self {
                name: name.as_ref().to_string(),
                case,
            }
        }

        /// Checks if the name is in the case.
        pub fn check(&self) -> bool {
            self.case.check(&self.name)
        }

        /// Converts the name to the case.
        pub fn convert(&self, case: RuaCase) -> Self {
            Self::new(case.convert(&self.name), case)
        }

        /// Returns the name.
        pub fn get_name(&self) -> &str {
            &self.name
        }

        /// Returns the name in the case.
        pub fn get_name_with_case(&self, case: &RuaCase) -> String {
            case.convert(&self.name)
        }
    }

    pub use syn_convert::*;

    mod syn_convert {
        use proc_macro2::Ident;

        use super::*;

        impl TryFrom<&Ident> for RuaName {
            type Error = ConversionError;

            fn try_from(value: &Ident) -> Result<Self, Self::Error> {
                let name = value.to_string();
                let case = if name.is_snake_case() {
                    RuaCase::SnakeCase
                } else if name.is_camel_case() {
                    RuaCase::CamelCase
                } else if name.is_pascal_case() {
                    RuaCase::PascalCase
                } else {
                    // TODO: make the case handling more robust
                    return Err(ConversionError::builder()
                        .span(&value.span())
                        .source_type("syn::Ident")
                        .target_type("RuaName")
                        .message(format!(
                            "The name of the ident is not a valid case: {}",
                            name
                        ))
                        .build());
                };
                Ok(Self { name, case })
            }
        }
    }
}

pub use rua_mod::*;

/// Types related to modules.
mod rua_mod {
    use super::*;

    /// Represents a module.
    #[rua_model_derive]
    pub struct RuaMod {
        /// Represents the name of the module.
        name: RuaName,
        /// Represents the type of the module.
        ty: RuaModType,
        /// Represents the root path of the module.
        root_path: Option<PathBuf>,
        /// Whether if the module is public.
        is_public: bool,
    }

    /// The type of a module.
    #[rua_model_derive]
    pub enum RuaModType {
        /// Represents a crate module.
        CrateModule,
        /// Represents a file module.
        FileModule,
    }

    impl RuaNamed for RuaMod {
        fn name(&self) -> &RuaName {
            &self.name
        }
    }

    impl RuaMod {
        /// Creates a new module.
        pub fn new(
            name: impl AsRef<str>,
            ty: RuaModType,
            root_path: Option<PathBuf>,
            is_public: bool,
        ) -> Self {
            Self {
                // A module should have a snake case name.
                name: RuaName::new(name, RuaCase::SnakeCase),
                ty,
                root_path,
                is_public,
            }
        }

        /// Returns the type of the module.
        pub fn ty(&self) -> &RuaModType {
            &self.ty
        }

        /// Returns the path to the root of the module.
        pub fn root_path(&self) -> &Option<PathBuf> {
            &self.root_path
        }
    }

    pub use syn_convert::*;
    mod syn_convert {
        use super::*;

        impl From<syn::ItemMod> for RuaMod {
            fn from(item: syn::ItemMod) -> Self {
                let name = item.ident.to_string();
                let ty = RuaModType::FileModule;
                let is_public = match item.vis {
                    syn::Visibility::Public(_) => true,
                    _ => false,
                };
                Self::new(name, ty, None, is_public)
            }
        }
    }
}

pub use rua_type::*;

mod rua_type {
    use super::*;
    /// Represents a type in Rust.
    #[rua_model_derive]
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
        /// Represents the ['isize'] type.
        Isize,
        /// Represents the ['usize'] type.
        Usize,
        /// Represents the [`char`] type.
        Char,
        /// Represents the [`str`] type.
        Str,
        /// Represents the [`String`] type.
        String,
        /// Represents the slice type [`&[T]`].
        Slice(RuaSlice),
        /// Represents the array type [`[T; N]`].
        Array(RuaArray),
        /// Represents the tuple type [`(T1, T2, ..., Tn)`].
        Tuple(RuaTuple),
        /// Represents a struct type.
        Struct(RuaStruct),
        /// Represents an enum type.
        Enum(RuaEnum),
        /// Represents a function type.
        Pointer(RuaPointer),
        /// Represents a reference type.
        Reference(RuaReference),
        /// Represents a function type.
        Fn(RuaFn),
        /// Represents a custom type.
        Custom(RuaName),
        /// Represents a generic type.
        Unit,
    }

    pub use syn_convert::*;
    mod syn_convert {
        use proc_macro2::Ident;
        use syn::{spanned::Spanned, BareFnArg, ReturnType, Type, TypePath};

        use super::*;

        // TODO: finish this conversion impl
        impl TryFrom<&Type> for RuaType {
            type Error = ConversionError;

            fn try_from(value: &Type) -> Result<Self, Self::Error> {
                let generate_error = |msg: &str| {
                    Err(ConversionError::builder()
                        .span(&value.span())
                        .source_type("syn::Type")
                        .target_type("RuaType")
                        .message(msg)
                        .build())
                };
                let err_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::Type")
                        .target_type("RuaType")
                        .build()
                };
                match value {
                    Type::Array(arr_item) => {
                        arr_item.try_into()
                            .map_err(err_mapper)
                            .map(Self::Array)
                    }
                    Type::BareFn(fn_item) => {
                        fn_item.try_into()
                            .map_err(err_mapper)
                            .map(|fn_:RuaBareFn| fn_.into())
                            .map(Self::Fn)
                    }
                    Type::Group(_) => {
                        generate_error("unsupported type Group")
                    }
                    Type::ImplTrait(_) => {
                        generate_error("unsupported type ImplTrait")
                    }
                    Type::Infer(_) => {
                        generate_error(
                            "unsupported type Infer, please specify the type explicitly",
                        )
                    }
                    Type::Macro(_) => {
                        generate_error("unsupported type Macro")
                    }
                    Type::Never(_) => {
                        generate_error("unsupported type Never")
                    }
                    Type::Paren(_) => {
                        generate_error("unsupported type Paren")
                    }
                    Type::Path(path_item) => {
                        path_item.try_into()
                            .map_err(err_mapper)
                    }
                    Type::Ptr(ptr_item) => {
                        ptr_item.try_into()
                            .map_err(err_mapper)
                            .map(Self::Pointer)
                    }
                    Type::Reference(ref_item) => {
                        ref_item.try_into()
                            .map_err(err_mapper)
                            .map(Self::Reference)
                    }
                    Type::Slice(slice_item) => {
                        slice_item.try_into()
                            .map_err(err_mapper)
                            .map(Self::Slice)
                    }
                    Type::TraitObject(_) => {
                        generate_error("unsupported type TraitObject")
                    }
                    Type::Tuple(tuple_item) => {
                        tuple_item.try_into()
                            .map_err(err_mapper)
                            .map(Self::Tuple)
                    }
                    Type::Verbatim(ts) => {
                        let ts = ts.to_string();
                        generate_error(
                            format!("unsupported type Verbatim: {}", ts)
                                .as_str(),
                        )
                    }
                    _ => generate_error("unsupported type"),
                }
            }
        }

        impl TryFrom<&ReturnType> for RuaType {
            type Error = ConversionError;

            fn try_from(value: &ReturnType) -> Result<Self, Self::Error> {
                let err_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::ReturnType")
                        .target_type("RuaType")
                        .build()
                };
                match value {
                    ReturnType::Default => Ok(RuaType::Unit),
                    ReturnType::Type(_, ty) => {
                        ty.as_ref().try_into().map_err(err_mapper)
                    }
                }
            }
        }

        impl TryFrom<&BareFnArg> for RuaType {
            type Error = ConversionError;

            fn try_from(value: &BareFnArg) -> Result<Self, Self::Error> {
                let err_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::BareFnArg")
                        .target_type("RuaType")
                        .build()
                };
                (&value.ty).try_into().map_err(err_mapper)
            }
        }

        impl TryFrom<&TypePath> for RuaType {
            type Error = ConversionError;

            fn try_from(value: &TypePath) -> Result<Self, Self::Error> {
                let err_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::TypePath")
                        .target_type("RuaType")
                        .build()
                };
                let path = &value.path;
                let segments = &path.segments;
                // find the last segment
                let last_segment = segments.last().ok_or_else(|| {
                    err_mapper(
                        ConversionError::builder()
                            .message("empty path")
                            .build(),
                    )
                })?;
                (&last_segment.ident).try_into().map_err(err_mapper)
            }
        }

        impl TryFrom<&Ident> for RuaType {
            type Error = ConversionError;

            fn try_from(value: &Ident) -> Result<Self, Self::Error> {
                let err_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::Ident")
                        .target_type("RuaType")
                        .build()
                };
                if value.eq("u8") {
                    Ok(RuaType::U8)
                } else if value.eq("u16") {
                    Ok(RuaType::U16)
                } else if value.eq("u32") {
                    Ok(RuaType::U32)
                } else if value.eq("u64") {
                    Ok(RuaType::U64)
                } else if value.eq("u128") {
                    Ok(RuaType::U128)
                } else if value.eq("usize") {
                    Ok(RuaType::Usize)
                } else if value.eq("i8") {
                    Ok(RuaType::I8)
                } else if value.eq("i16") {
                    Ok(RuaType::I16)
                } else if value.eq("i32") {
                    Ok(RuaType::I32)
                } else if value.eq("i64") {
                    Ok(RuaType::I64)
                } else if value.eq("i128") {
                    Ok(RuaType::I128)
                } else if value.eq("isize") {
                    Ok(RuaType::Isize)
                } else if value.eq("f32") {
                    Ok(RuaType::F32)
                } else if value.eq("f64") {
                    Ok(RuaType::F64)
                } else if value.eq("bool") {
                    Ok(RuaType::Bool)
                } else if value.eq("char") || value.eq("Char") {
                    Ok(RuaType::Char)
                } else if value.eq("str") {
                    Ok(RuaType::Str)
                } else if value.eq("String") {
                    Ok(RuaType::String)
                } else if value.eq("unit") || value.eq("Unit") || value.eq("()")
                {
                    Ok(RuaType::Unit)
                } else {
                    Ok(RuaType::Custom(value.try_into().map_err(err_mapper)?))
                }
            }
        }
    }
}

pub use rua_var::*;

mod rua_var {
    use super::*;

    /// Represents a variable in Rust. A variable in Rust has a name and a type.
    #[rua_model_derive]
    pub struct RuaVar {
        /// Represents the name of the variable.
        pub name: RuaName,
        /// Represents the type of the variable.
        pub ty: Box<RuaType>,
    }

    pub use syn_convert::*;
    mod syn_convert {
        use super::*;
        use syn::{spanned::Spanned, Field, FnArg, PatType};

        impl TryFrom<&FnArg> for RuaVar {
            type Error = ConversionError;

            fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::FnArg")
                        .target_type("RuaVar")
                        .build()
                };
                let generate_error = |msg: &str| {
                    Err(ConversionError::builder()
                        .span(&value.span())
                        .source_type("syn::FnArg")
                        .target_type("RuaVar")
                        .message(msg)
                        .build())
                };
                match value {
                    FnArg::Receiver(_) => {
                        generate_error("receiver is not supported")
                    }
                    FnArg::Typed(typed) => {
                        typed.try_into().map_err(error_mapper)
                    }
                }
            }
        }

        impl TryFrom<&PatType> for RuaVar {
            type Error = ConversionError;

            fn try_from(value: &PatType) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::PatType")
                        .target_type("RuaVar")
                        .build()
                };
                let generate_error = |msg: &str| {
                    Err(ConversionError::builder()
                        .span(&value.span())
                        .source_type("syn::PatType")
                        .target_type("RuaVar")
                        .message(msg)
                        .build())
                };
                let var_name =
                    match value.pat.as_ref() {
                        syn::Pat::Ident(ident) => {
                            (&ident.ident).try_into().map_err(error_mapper)?
                        }
                        _ => return generate_error(
                            "unsupported pattern type, only Ident is supported",
                        ),
                    };
                let var_ty =
                    value.ty.as_ref().try_into().map_err(error_mapper)?;
                Ok(RuaVar {
                    name: var_name,
                    ty: Box::new(var_ty),
                })
            }
        }

        impl TryFrom<&Field> for RuaVar {
            type Error = ConversionError;

            fn try_from(value: &Field) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::Field")
                        .target_type("RuaVar")
                        .build()
                };
                let generate_error = |msg: &str| {
                    Err(ConversionError::builder()
                        .span(&value.span())
                        .source_type("syn::Field")
                        .target_type("RuaVar")
                        .message(msg)
                        .build())
                };
                let var_name: RuaName;
                match value.ident.as_ref() {
                    Some(ident) => {
                        var_name = ident.try_into().map_err(error_mapper)?
                    }
                    None => return generate_error("field name is required"),
                };
                let var_ty = (&value.ty).try_into().map_err(error_mapper)?;
                Ok(RuaVar {
                    name: var_name,
                    ty: Box::new(var_ty),
                })
            }
        }
    }
}

pub use rua_slice::*;
mod rua_slice {
    use super::*;
    /// Represents an array slice in Rust, i.e `&[T]`.
    #[rua_model_derive]
    pub struct RuaSlice {
        /// Represents the type of the slice.
        pub ty: Box<RuaType>,
    }

    impl From<RuaSlice> for RuaType {
        fn from(value: RuaSlice) -> Self {
            RuaType::Slice(value)
        }
    }

    pub use syn_convert::*;
    mod syn_convert {
        use syn::{spanned::Spanned, TypeSlice};

        use super::*;

        impl TryFrom<&TypeSlice> for RuaSlice {
            type Error = ConversionError;

            fn try_from(value: &TypeSlice) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::TypeSlice")
                        .target_type("RuaSlice")
                        .build()
                };
                let ty =
                    value.elem.as_ref().try_into().map_err(error_mapper)?;
                Ok(RuaSlice { ty: Box::new(ty) })
            }
        }
    }
}

pub use rua_array::*;
mod rua_array {
    use super::*;
    /// Represents an array in Rust, i.e `[T; N]`.
    #[rua_model_derive]
    pub struct RuaArray {
        /// Represents the type of the array.
        pub ty: Box<RuaType>,
        /// Represents the length of the array.
        pub len: RuaArrayLen,
    }

    /// Represents the length of an array.
    #[rua_model_derive]
    pub enum RuaArrayLen {
        /// Represents a constant length.
        Num(usize),
        /// Represents a variable length. The String is the name of the variable.
        Const(String),
    }

    impl From<RuaArray> for RuaType {
        fn from(value: RuaArray) -> Self {
            RuaType::Array(value)
        }
    }

    pub use syn_convert::*;
    mod syn_convert {
        use super::*;
        use syn::{spanned::Spanned, Expr, TypeArray};

        impl TryFrom<&TypeArray> for RuaArray {
            type Error = ConversionError;

            fn try_from(value: &TypeArray) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::TypeArray")
                        .target_type("RuaArray")
                        .build()
                };
                let ty =
                    value.elem.as_ref().try_into().map_err(error_mapper)?;
                let len = (&value.len).try_into().map_err(error_mapper)?;
                Ok(RuaArray {
                    ty: Box::new(ty),
                    len,
                })
            }
        }

        impl TryFrom<&Expr> for RuaArrayLen {
            type Error = ConversionError;

            fn try_from(value: &Expr) -> Result<Self, Self::Error> {
                let generate_error = |msg: &str| {
                    Err(ConversionError::builder()
                        .span(&value.span())
                        .source_type("syn::Expr")
                        .target_type("RuaArrayLen")
                        .message(msg)
                        .build())
                };
                match value {
                    Expr::Lit(lit) => match lit.lit {
                        syn::Lit::Int(ref int) => {
                            let len = int.base10_parse::<usize>();
                            if len.is_err() {
                                return generate_error("failed to parse usize");
                            }
                            Ok(RuaArrayLen::Num(len.unwrap()))
                        }
                        _ => generate_error("unsupported literal type"),
                    },
                    Expr::Path(ref path) => {
                        let path_segments = &path.path.segments;
                        if path_segments.len() != 1 {
                            return generate_error(
                                "unsupported path segments length",
                            );
                        }
                        let path_segment = &path_segments[0];
                        let ident = &path_segment.ident;
                        Ok(RuaArrayLen::Const(ident.to_string()))
                    }
                    _ => generate_error("unsupported expression type"),
                }
            }
        }
    }
}

pub use rua_tuple::*;
mod rua_tuple {
    use super::*;
    /// Represents a tuple in Rust, i.e `(T1, T2, ..., Tn)`.
    #[rua_model_derive]
    pub struct RuaTuple {
        /// Represents the types of the tuple.
        pub tys: Vec<RuaType>,
    }

    impl From<RuaTuple> for RuaType {
        fn from(value: RuaTuple) -> Self {
            RuaType::Tuple(value)
        }
    }

    pub use syn_convert::*;
    mod syn_convert {
        use syn::{spanned::Spanned, TypeTuple};

        use super::*;

        impl TryFrom<&TypeTuple> for RuaTuple {
            type Error = ConversionError;

            fn try_from(value: &TypeTuple) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::TypeTuple")
                        .target_type("RuaTuple")
                        .build()
                };
                let tys = value
                    .elems
                    .iter()
                    .map(|ty| ty.try_into().map_err(error_mapper))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(RuaTuple { tys })
            }
        }
    }
}

pub use rua_struct::*;
mod rua_struct {
    use super::*;

    /// Represents a struct in Rust.
    #[rua_model_derive]
    pub enum RuaStruct {
        /// Represents a named struct.
        Named(RuaNamedStruct),
        /// Represents a tuple struct.
        Tuple(RuaTupleStruct),
        /// Represents a unit struct.
        Unit(RuaUnitStruct),
    }

    /// Represents a named struct in Rust.
    #[rua_model_derive]
    pub struct RuaNamedStruct {
        /// Represents the name of the struct.
        pub name: RuaName,
        /// Represents the fields of the struct.
        pub fields: Vec<RuaVar>,
    }

    /// Represents a tuple struct in Rust.
    #[rua_model_derive]
    pub struct RuaTupleStruct {
        /// Represents the name of the struct.
        pub name: RuaName,
        /// Represents the types of the struct.
        pub tys: Vec<RuaType>,
    }

    /// Represents a unit struct in Rust.
    #[rua_model_derive]
    pub struct RuaUnitStruct {
        /// Represents the name of the struct.
        pub name: RuaName,
    }

    impl RuaNamed for RuaNamedStruct {
        fn name(&self) -> &RuaName {
            &self.name
        }
    }

    impl RuaNamed for RuaTupleStruct {
        fn name(&self) -> &RuaName {
            &self.name
        }
    }

    impl RuaNamed for RuaUnitStruct {
        fn name(&self) -> &RuaName {
            &self.name
        }
    }

    impl RuaNamed for RuaStruct {
        fn name(&self) -> &RuaName {
            match self {
                RuaStruct::Named(named) => named.name(),
                RuaStruct::Tuple(tuple) => tuple.name(),
                RuaStruct::Unit(unit) => unit.name(),
            }
        }
    }

    impl From<RuaNamedStruct> for RuaStruct {
        fn from(value: RuaNamedStruct) -> Self {
            RuaStruct::Named(value)
        }
    }

    impl From<RuaTupleStruct> for RuaStruct {
        fn from(value: RuaTupleStruct) -> Self {
            RuaStruct::Tuple(value)
        }
    }

    impl From<RuaUnitStruct> for RuaStruct {
        fn from(value: RuaUnitStruct) -> Self {
            RuaStruct::Unit(value)
        }
    }

    impl From<RuaStruct> for RuaType {
        fn from(value: RuaStruct) -> Self {
            RuaType::Struct(value)
        }
    }

    pub use syn_convert::*;
    mod syn_convert {
        use proc_macro2::Ident;
        use syn::{
            spanned::Spanned, Fields, FieldsNamed, FieldsUnnamed, ItemStruct,
            Variant,
        };

        use super::*;

        fn convert_named_fields(
            fields: &FieldsNamed,
            error_mapper: &impl Fn(ConversionError) -> ConversionError,
        ) -> Result<Vec<RuaVar>, ConversionError> {
            let fields = fields
                .named
                .iter()
                .map(|field| field.try_into().map_err(error_mapper))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(fields)
        }

        fn convert_unnamed_fields(
            fields: &FieldsUnnamed,
            error_mapper: &impl Fn(ConversionError) -> ConversionError,
        ) -> Result<Vec<RuaType>, ConversionError> {
            let fields = fields
                .unnamed
                .iter()
                .map(|field| (&field.ty).try_into().map_err(error_mapper))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(fields)
        }

        fn convert_fields(
            name: &Ident,
            fields: &Fields,
            error_mapper: &impl Fn(ConversionError) -> ConversionError,
        ) -> Result<RuaStruct, ConversionError> {
            match fields {
                syn::Fields::Named(named) => {
                    let fields = convert_named_fields(named, error_mapper)?;
                    Ok(RuaStruct::Named(RuaNamedStruct {
                        name: name.try_into().map_err(error_mapper)?,
                        fields,
                    }))
                }
                syn::Fields::Unnamed(unnamed) => {
                    let tys = convert_unnamed_fields(unnamed, error_mapper)?;
                    Ok(RuaStruct::Tuple(RuaTupleStruct {
                        name: name.try_into().map_err(error_mapper)?,
                        tys,
                    }))
                }
                syn::Fields::Unit => Ok(RuaStruct::Unit(RuaUnitStruct {
                    name: name.try_into().map_err(error_mapper)?,
                })),
            }
        }

        impl TryFrom<&ItemStruct> for RuaStruct {
            type Error = ConversionError;

            fn try_from(value: &ItemStruct) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::ItemStruct")
                        .target_type("RuaStruct")
                        .build()
                };
                convert_fields(&value.ident, &value.fields, &error_mapper)
            }
        }

        impl TryFrom<&Variant> for RuaStruct {
            type Error = ConversionError;

            fn try_from(value: &Variant) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::Variant")
                        .target_type("RuaStruct")
                        .build()
                };
                convert_fields(&value.ident, &value.fields, &error_mapper)
            }
        }
    }
}

pub use rua_enum::*;

mod rua_enum {
    use super::*;

    /// Represents an enum in Rust.
    #[rua_model_derive]
    pub struct RuaEnum {
        /// Represents the name of the enum.
        pub name: RuaName,
        /// Represents the variants of the enum.
        pub variants: Vec<RuaStruct>,
    }

    impl RuaNamed for RuaEnum {
        fn name(&self) -> &RuaName {
            &self.name
        }
    }

    impl From<RuaEnum> for RuaType {
        fn from(value: RuaEnum) -> Self {
            RuaType::Enum(value)
        }
    }

    pub use syn_convert::*;

    mod syn_convert {
        use super::*;
        use syn::{spanned::Spanned, ItemEnum};

        impl TryFrom<ItemEnum> for RuaEnum {
            type Error = ConversionError;

            fn try_from(value: ItemEnum) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::ItemEnum")
                        .target_type("RuaEnum")
                        .build()
                };
                let variants = value
                    .variants
                    .iter()
                    .map(|variant| variant.try_into().map_err(error_mapper))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(RuaEnum {
                    name: (&value.ident).try_into().map_err(error_mapper)?,
                    variants,
                })
            }
        }
    }
}

pub use rua_fn::*;
mod rua_fn {
    use super::*;

    /// Represents a function in Rust.
    #[rua_model_derive]
    pub enum RuaFn {
        /// Represents a bare function in Rust.
        Bare(RuaBareFn),
        /// Represents a function in Rust.
        Fn(RuaSigFn),
    }

    /// Represents a bare function in Rust. A bare function is a function
    /// without a name.
    #[rua_model_derive]
    pub struct RuaBareFn {
        /// Represents the parameters of the bare function.
        pub params: Vec<RuaType>,
        /// Represents the return type of the bare function.
        pub ret: Box<RuaType>,
    }

    /// Represents a function in Rust.
    #[rua_model_derive]
    pub struct RuaSigFn {
        /// Represents the name of the function.
        pub name: RuaName,
        /// Represents the parameters of the function.
        pub params: Vec<RuaVar>,
        /// Represents the return type of the function.
        pub ret: Box<RuaType>,
    }

    impl RuaNamed for RuaSigFn {
        fn name(&self) -> &RuaName {
            &self.name
        }
    }

    impl From<RuaFn> for RuaType {
        fn from(value: RuaFn) -> Self {
            RuaType::Fn(value)
        }
    }

    impl From<RuaBareFn> for RuaFn {
        fn from(value: RuaBareFn) -> Self {
            RuaFn::Bare(value)
        }
    }

    impl From<RuaSigFn> for RuaFn {
        fn from(value: RuaSigFn) -> Self {
            RuaFn::Fn(value)
        }
    }

    pub use syn_convert::*;

    mod syn_convert {
        use syn::{spanned::Spanned, ItemFn, TypeBareFn};

        use super::*;

        impl TryFrom<&ItemFn> for RuaSigFn {
            type Error = ConversionError;

            fn try_from(value: &ItemFn) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::ItemFn")
                        .target_type("RuaFn")
                        .build()
                };
                let params = value
                    .sig
                    .inputs
                    .iter()
                    .map(|param| param.try_into().map_err(error_mapper))
                    .collect::<Result<Vec<_>, _>>()?;
                let ret =
                    (&value.sig.output).try_into().map_err(error_mapper)?;
                Ok(RuaSigFn {
                    name: (&value.sig.ident)
                        .try_into()
                        .map_err(error_mapper)?,
                    params,
                    ret: Box::new(ret),
                })
            }
        }

        impl TryFrom<&TypeBareFn> for RuaBareFn {
            type Error = ConversionError;

            fn try_from(value: &TypeBareFn) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::TypeBareFn")
                        .target_type("RuaFn")
                        .build()
                };
                let params = value
                    .inputs
                    .iter()
                    .map(|param| param.try_into().map_err(error_mapper))
                    .collect::<Result<Vec<_>, _>>()?;
                let ret = (&value.output).try_into().map_err(error_mapper)?;
                Ok(RuaBareFn {
                    params,
                    ret: Box::new(ret),
                })
            }
        }
    }
}

pub use rua_pointer::*;

mod rua_pointer {
    use super::*;

    /// Represents a pointer in Rust.
    #[rua_model_derive]
    pub struct RuaPointer {
        /// Represents if the pointer is a constant pointer. If it is not constant,
        /// it is a mutable pointer.
        pub is_const: bool,
        /// Represents the type of the value the pointer points to.
        pub ty: Box<RuaType>,
    }

    impl From<RuaPointer> for RuaType {
        fn from(value: RuaPointer) -> Self {
            RuaType::Pointer(value)
        }
    }

    pub use syn_convert::*;

    mod syn_convert {
        use syn::{spanned::Spanned, TypePtr};

        use super::*;

        impl TryFrom<&TypePtr> for RuaPointer {
            type Error = ConversionError;

            fn try_from(value: &TypePtr) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::TypePtr")
                        .target_type("RuaPointer")
                        .build()
                };
                Ok(RuaPointer {
                    is_const: value.const_token.is_some(),
                    ty: Box::new(
                        (value.elem.as_ref())
                            .try_into()
                            .map_err(error_mapper)?,
                    ),
                })
            }
        }
    }
}

pub use rua_ref::*;

mod rua_ref {
    use super::*;

    /// Represents a reference in Rust.
    #[rua_model_derive]
    pub struct RuaReference {
        /// Represents if the reference is a mutable reference. If it is not
        /// mutable, it is an immutable reference.
        pub is_mut: bool,
        /// Represents the type of the value the reference points to.
        pub ty: Box<RuaType>,
    }

    impl From<RuaReference> for RuaType {
        fn from(value: RuaReference) -> Self {
            RuaType::Reference(value)
        }
    }

    pub use syn_convert::*;

    mod syn_convert {
        use syn::{spanned::Spanned, TypeReference};

        use super::*;

        impl TryFrom<&TypeReference> for RuaReference {
            type Error = ConversionError;

            fn try_from(value: &TypeReference) -> Result<Self, Self::Error> {
                let error_mapper = |err: ConversionError| {
                    err.builder_for_next()
                        .span(&value.span())
                        .source_type("syn::TypeReference")
                        .target_type("RuaReference")
                        .build()
                };
                Ok(RuaReference {
                    is_mut: value.mutability.is_some(),
                    ty: Box::new(
                        (value.elem.as_ref())
                            .try_into()
                            .map_err(error_mapper)?,
                    ),
                })
            }
        }
    }
}
