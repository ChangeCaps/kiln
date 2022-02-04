use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

pub fn kiln_path() -> TokenStream {
    if let Ok(found) = crate_name("kiln") {
        crate_path(found)
    } else if let Ok(found) = crate_name("kiln-gpu") {
        crate_path(found)
    } else {
        panic!("crate 'kiln' not found in cargo.toml")
    }
}

pub fn crate_path(found: FoundCrate) -> TokenStream {
    match found {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
    }
}
