use quote::quote;
use syn::{parse_macro_input, Item};

extern crate proc_macro;

#[proc_macro_attribute]
pub fn rua_model_derive(
    _attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);
    match item {
        Item::Enum(_) => {}
        Item::Struct(_) => {}
        _ => panic!("Only enum and struct are supported"),
    };
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #item
    }
    .into()
}

