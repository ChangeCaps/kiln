use naga::{
    front::wgsl::parse_str,
    valid::{Capabilities, ValidationFlags, Validator},
};
use quote::quote;

use crate::{
    entry_points::EntryPoints,
    rust_type::{rust_constant_decl, rust_type_decl},
    span::{SpannedResult, SpannedSource},
};

pub fn wgsl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = SpannedSource::new(&input.into());
    let module = parse_str(&source.source).spanned_unwrap(&source);
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
    let module_info = validator.validate(&module).spanned_unwrap(&source);

    let entry_points = EntryPoints::new(&module, &module_info);

    let constants = module
        .constants
        .iter()
        .map(|(handle, _)| rust_constant_decl(&module, &handle));

    let types = module
        .types
        .iter()
        .map(|(_, ty)| rust_type_decl(&module, ty));

    let bindings = entry_points.bindings(&source.source);

    let expanded = quote! {
        #(#constants)*

        #(#types)*

        #bindings
    };

    println!("{}", expanded);

    expanded.into()
}
