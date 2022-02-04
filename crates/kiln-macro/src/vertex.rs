use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{Diagnostic, Level};
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Attribute, Data, DeriveInput, Fields, LitInt, Token, Type,
};

use crate::path::kiln_path;

syn::custom_keyword!(C);
syn::custom_keyword!(location);

enum FieldAttr {
    Location(u32),
}

impl Parse for FieldAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.parse::<location>().is_ok() {
            input.parse::<Token![=]>()?;
            let int: LitInt = input.parse()?;

            Ok(FieldAttr::Location(int.base10_parse()?))
        } else {
            Err(syn::Error::new(input.span(), "invalid attribute"))
        }
    }
}

struct FieldAttrs {
    location: u32,
}

impl FieldAttrs {
    fn parse(span: Span, attrs: &[Attribute]) -> Self {
        let mut location = None;

        for arg in attrs {
            if arg.path.is_ident("vertex") {
                arg.parse_args_with(|parser: ParseStream| {
                    let attr = parser.parse::<FieldAttr>()?;

                    match attr {
                        FieldAttr::Location(loc) => {
                            if location.is_some() {
                                panic!("location defined twice");
                            }

                            location = Some(loc);
                        }
                    }

                    Ok(())
                })
                .unwrap();
            }
        }

        if location.is_none() {
            Diagnostic::spanned(span, Level::Error, String::from("location not specified")).abort();
        }

        Self {
            location: location.unwrap(),
        }
    }
}

pub fn derive_vertex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !is_repr_c(&input.attrs) {
        Diagnostic::spanned(
            input.span(),
            Level::Error,
            String::from("structs deriving Vertex must be #[repr(C)]"),
        )
        .abort();
    }

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let kiln_path = kiln_path();

    let attrs = repr_attrs(input.data);

    let array_stride = attrs.size();
    let attributes = attrs.attrs();

    let expanded = quote! {
        impl #impl_generics #kiln_path::Vertex for #name #ty_generics #where_clause {
            fn array_stride() -> ::std::primitive::u64 {
                (#array_stride) as ::std::primitive::u64
            }

            #[allow(unused)]
            fn attributes() -> ::std::vec::Vec<#kiln_path::VertexAttribute> {
                #attributes
            }
        }
    };

    expanded.into()
}

fn is_repr_c(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path.is_ident("repr") {
            if attr.parse_args::<C>().is_ok() {
                return true;
            }
        }
    }

    false
}

struct VertexAttr {
    pub ty: Type,
    pub attrs: FieldAttrs,
}

struct VertexAttrs {
    pub attrs: Vec<VertexAttr>,
}

impl VertexAttrs {
    pub fn size(&self) -> TokenStream {
        let kiln_path = kiln_path();

        let sizes = self.attrs.iter().map(|attr| {
            let ty = &attr.ty;

            quote_spanned! {ty.span()=>
                <#ty as #kiln_path::AsVertexAttribute>::FORMAT.size()
            }
        });

        quote! {
            0 #(+ #sizes)*
        }
    }

    pub fn attrs(&self) -> TokenStream {
        let kiln_path = kiln_path();

        let attrs = self.attrs.iter().map(|attr| {
            let ty = &attr.ty;
            let location = attr.attrs.location;

            quote_spanned! {ty.span()=>
                attrs.push(#kiln_path::VertexAttribute {
                    format: <#ty as #kiln_path::AsVertexAttribute>::FORMAT,
                    offset,
                    shader_location: #location,
                });

                offset += <#ty as #kiln_path::AsVertexAttribute>::FORMAT.size();
            }
        });

        quote! {
            let mut attrs = ::std::vec::Vec::new();

            let mut offset = 0u64;

            #(#attrs)*

            attrs
        }
    }
}

fn repr_attrs(data: Data) -> VertexAttrs {
    match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(named) => {
                let attrs = named
                    .named
                    .iter()
                    .map(|field| VertexAttr {
                        ty: field.ty.clone(),
                        attrs: FieldAttrs::parse(field.span(), &field.attrs),
                    })
                    .collect();

                VertexAttrs { attrs }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!("Vertex can only be derived for structs"),
    }
}
