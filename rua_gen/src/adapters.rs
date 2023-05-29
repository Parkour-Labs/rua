//! The adapters that implement the traits in [rua_gen::rua] on the [syn] crate.

use crate::models::*;

pub use rua_fn::*;

impl RuaStrTyped for Item {
    fn ty(&self) -> &str {
        match self {
            Item::Const(_) => "const",
            Item::Enum(_) => "enum",
            Item::ExternCrate(_) => "extern crate",
            Item::Fn(_) => "fn",
            Item::ForeignMod(_) => "foreign mod",
            Item::Impl(_) => "impl",
            Item::Macro(_) => "macro",
            Item::Mod(_) => "mod",
            Item::Static(_) => "static",
            Item::Struct(_) => "struct",
            Item::Trait(_) => "trait",
            Item::TraitAlias(_) => "trait alias",
            Item::Type(_) => "type",
            Item::Union(_) => "union",
            Item::Use(_) => "use",
            Item::Verbatim(_) => "verbatim",
            _ => todo!(),
        }
    }
}

/// The adapter for [syn::ItemFn].
mod rua_fn {
    use syn::{ItemFn, Type};

    use super::*;

    impl RuaNamed for ItemFn {
        fn name(&self) -> String {
            self.sig.ident.to_string()
        }
    }

    impl RuaVisible for ItemFn {
        fn is_pub(&self) -> bool {
            match self.vis {
                syn::Visibility::Public(_) => true,
                _ => false,
            }
        }
    }

    impl RuaHasAttr for ItemFn {
        fn attrs(&self) -> Vec<&dyn RuaAttr> {
            let mut res: Vec<&dyn RuaAttr> = vec![];
            for attr in &self.attrs {
                res.push(attr);
            }
            res
        }
    }

    impl RuaFn for ItemFn {
        fn fn_args(&self) -> Vec<&dyn RuaVar> {
            let mut res: Vec<&dyn RuaVar> = vec![];
            for arg in &self.sig.inputs {
                res.push(arg);
            }
            res
        }

        fn ret_ty(&self) -> Option<&Type> {
            match &self.sig.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(ty),
            }
        }
    }
}

pub use rua_enum::*;

/// The adapter for [syn::ItemEnum].
mod rua_enum {
    use super::*;
    use syn::{ItemEnum, Variant};

    impl RuaNamed for ItemEnum {
        fn name(&self) -> String {
            self.ident.to_string()
        }
    }

    impl RuaVisible for ItemEnum {
        fn is_pub(&self) -> bool {
            match self.vis {
                syn::Visibility::Public(_) => true,
                _ => false,
            }
        }
    }

    impl RuaHasAttr for ItemEnum {
        fn attrs(&self) -> Vec<&dyn RuaAttr> {
            let mut res: Vec<&dyn RuaAttr> = vec![];
            for attr in &self.attrs {
                res.push(attr);
            }
            res
        }
    }

    impl RuaNamed for Variant {
        fn name(&self) -> String {
            self.ident.to_string()
        }
    }

    impl RuaWithFields for Variant {
        fn field(&self) -> Vec<&dyn RuaVar> {
            let mut res: Vec<&dyn RuaVar> = vec![];
            for field in &self.fields {
                res.push(field);
            }
            res
        }
    }

    impl RuaEnumVariant for Variant {
        fn is_unit(&self) -> bool {
            match &self.fields {
                syn::Fields::Unit => true,
                _ => false,
            }
        }
    }

    impl RuaEnum for ItemEnum {
        fn variants(&self) -> Vec<&dyn RuaEnumVariant> {
            let mut res: Vec<&dyn RuaEnumVariant> = vec![];
            for variant in &self.variants {
                res.push(variant);
            }
            res
        }
    }
}

pub use rua_struct::*;

/// The adapter for [syn::ItemStruct].
mod rua_struct {
    use super::*;
    use syn::ItemStruct;

    impl RuaNamed for ItemStruct {
        fn name(&self) -> String {
            self.ident.to_string()
        }
    }

    impl RuaVisible for ItemStruct {
        fn is_pub(&self) -> bool {
            match self.vis {
                syn::Visibility::Public(_) => true,
                _ => false,
            }
        }
    }

    impl RuaWithFields for ItemStruct {
        fn field(&self) -> Vec<&dyn RuaVar> {
            let mut res: Vec<&dyn RuaVar> = vec![];
            for field in &self.fields {
                res.push(field);
            }
            res
        }
    }

    impl RuaHasAttr for ItemStruct {
        fn attrs(&self) -> Vec<&dyn RuaAttr> {
            let mut res: Vec<&dyn RuaAttr> = vec![];
            for attr in &self.attrs {
                res.push(attr);
            }
            res
        }
    }

    impl RuaStruct for ItemStruct {}
}

pub use rua_var::*;

/// The adapter for [syn::FnArg], [syn::PatType], [syn::Pat], [syn::Type],
/// [syn::Field]
mod rua_var {
    use super::*;
    use syn::{Pat, Type};

    impl RuaNamedOpt for Pat {
        fn name_opt(&self) -> Option<String> {
            match self {
                Pat::Ident(pat_ident) => pat_ident.ident.to_string().into(),
                _ => None,
            }
        }
    }

    pub use fn_arg::*;
    mod fn_arg {
        use super::*;
        use syn::FnArg;

        impl RuaNamedOpt for FnArg {
            fn name_opt(&self) -> Option<String> {
                match self {
                    FnArg::Receiver(_) => None,
                    FnArg::Typed(pat_type) => pat_type.pat.name_opt(),
                }
            }
        }

        impl RuaTyped for FnArg {
            fn ty(&self) -> &Type {
                match self {
                    FnArg::Receiver(_) => todo!(),
                    FnArg::Typed(pat_type) => &pat_type.ty,
                }
            }
        }

        impl RuaVar for FnArg {}
    }

    pub use pat_type::*;
    mod pat_type {
        use super::*;
        use syn::PatType;

        impl RuaNamedOpt for PatType {
            fn name_opt(&self) -> Option<String> {
                self.pat.name_opt()
            }
        }

        impl RuaTyped for PatType {
            fn ty(&self) -> &Type {
                &self.ty
            }
        }

        impl RuaVar for PatType {}
    }

    pub use type_::*;

    mod type_ {
        use super::*;

        impl RuaTyped for Type {
            fn ty(&self) -> &Type {
                self
            }
        }

        impl RuaNamedOpt for Type {
            fn name_opt(&self) -> Option<String> {
                None
            }
        }

        impl RuaVar for Type {}
    }

    pub use field::*;
    mod field {
        use super::*;
        use syn::Field;

        impl RuaNamedOpt for Field {
            fn name_opt(&self) -> Option<String> {
                self.ident.as_ref().map(|ident| ident.to_string())
            }
        }

        impl RuaTyped for Field {
            fn ty(&self) -> &Type {
                &self.ty
            }
        }

        impl RuaVar for Field {}
    }
}

pub use rua_attr::*;

/// The adapter for [syn::Attribute].
mod rua_attr {
    use super::*;
    use syn::{punctuated::Punctuated, token::PathSep, Attribute, PathSegment};

    impl RuaNamed for Punctuated<PathSegment, PathSep> {
        fn name(&self) -> String {
            let mut res = String::new();
            for segment in self {
                res.push_str(&segment.ident.to_string());
            }
            res
        }
    }

    impl RuaNamed for Attribute {
        fn name(&self) -> String {
            match &self.meta {
                syn::Meta::Path(path) => path.segments.name(),
                syn::Meta::List(list) => list.path.segments.name(),
                syn::Meta::NameValue(name) => name.path.segments.name(),
            }
        }
    }

    impl RuaAttr for Attribute {}
}

pub use rua_mod::*;
use syn::Item;

/// The adapter for [syn::ItemMod].
mod rua_mod {
    use super::*;
    use syn::ItemMod;

    impl RuaNamed for ItemMod {
        fn name(&self) -> String {
            self.ident.to_string()
        }
    }

    impl RuaVisible for ItemMod {
        fn is_pub(&self) -> bool {
            match self.vis {
                syn::Visibility::Public(_) => true,
                _ => false,
            }
        }
    }

    impl RuaHasAttr for ItemMod {
        fn attrs(&self) -> Vec<&dyn RuaAttr> {
            let mut res: Vec<&dyn RuaAttr> = vec![];
            for attr in &self.attrs {
                res.push(attr);
            }
            res
        }
    }

    impl RuaMod for ItemMod {}
}
