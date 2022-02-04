use naga::{
    proc::TypeResolution, ArraySize, Constant, ConstantInner, GlobalVariable, Handle, Module,
    ScalarKind, ScalarValue, Type, TypeInner, VectorSize,
};
use proc_macro2::{Ident, Spacing, Span, TokenStream};
use quote::quote;
use syn::parse_quote;

use crate::path::kiln_path;

pub fn rust_constant_decl(
    module: &Module,
    constant_handle: &Handle<Constant>,
) -> Option<TokenStream> {
    let constant = &module.constants[*constant_handle];

    let name = constant.name.as_ref()?;
    let ident = Ident::new(name, Span::call_site());
    let ty = match constant.inner.resolve_type() {
        TypeResolution::Handle(ty) => rust_type(module, &module.types[ty]),
        TypeResolution::Value(inner) => rust_type_inner(module, &inner),
    };
    let value = rust_constant_inner(module, &constant.inner);

    Some(quote! {
        pub const #ident: #ty = #value;
    })
}

pub fn rust_constant(module: &Module, constant: &Handle<Constant>) -> TokenStream {
    let constant = &module.constants[*constant];

    match constant.name {
        Some(ref name) => {
            let ident = Ident::new(name, Span::call_site());

            quote!(self::#ident)
        }
        None => rust_constant_inner(module, &constant.inner),
    }
}

pub fn rust_constant_inner(module: &Module, inner: &ConstantInner) -> TokenStream {
    match *inner {
        ConstantInner::Scalar { width, value } => rust_scalar_value(value, width),
        _ => unreachable!(),
    }
}

pub fn rust_scalar_value(value: ScalarValue, width: u8) -> TokenStream {
    match value {
        ScalarValue::Bool(val) => parse_quote!(#val as ::std::primitive::bool),
        ScalarValue::Float(val) => match width {
            4 => parse_quote!(#val as ::std::primitive::f32),
            8 => parse_quote!(#val as ::std::primitive::f64),
            _ => unreachable!(),
        },
        ScalarValue::Sint(val) => match width {
            1 => parse_quote!(#val as ::std::primitive::i8),
            2 => parse_quote!(#val as ::std::primitive::i16),
            4 => parse_quote!(#val as ::std::primitive::i32),
            8 => parse_quote!(#val as ::std::primitive::i64),
            _ => unreachable!(),
        },
        ScalarValue::Uint(val) => match width {
            1 => parse_quote!(#val as ::std::primitive::u8),
            2 => parse_quote!(#val as ::std::primitive::u16),
            4 => parse_quote!(#val as ::std::primitive::u32),
            8 => parse_quote!(#val as ::std::primitive::u64),
            _ => unreachable!(),
        },
    }
}

pub fn rust_type_decl(module: &Module, ty: &Type) -> Option<TokenStream> {
    let kiln_path = kiln_path();
    let name = ty.name.as_ref()?;
    let ident = Ident::new(name, Span::call_site());

    match ty.inner {
        TypeInner::Struct { ref members, .. } => {
            let decl_members = members.iter().map(|member| {
                let ident = Ident::new(member.name.as_ref().unwrap(), Span::call_site());
                let ty = rust_type(module, &module.types[member.ty]);

                quote! {
                    pub #ident: #ty,
                }
            });

            let new_args = members.iter().map(|member| {
                let ident = Ident::new(member.name.as_ref().unwrap(), Span::call_site());
                let ty = rust_type(module, &module.types[member.ty]);

                quote! {
                    #ident: #ty
                }
            });

            let new_members = members.iter().map(|member| {
                let ident = Ident::new(member.name.as_ref().unwrap(), Span::call_site());

                quote! {
                    #ident
                }
            });

            Some(quote! {
                #[repr(C)]
                #[derive(
                    ::std::clone::Clone,
                    ::std::marker::Copy,
                    ::std::fmt::Debug,
                    ::std::default::Default,
                )]
                pub struct #ident {
                    #(#decl_members)*
                }

                impl #ident {
                    pub const fn new(#(#new_args),*) -> Self {
                        Self { #(#new_members),* }
                    }
                }

                unsafe impl #kiln_path::BufferData for #ident {}
            })
        }
        _ => None,
    }
}

pub fn rust_type(module: &Module, ty: &Type) -> Option<syn::Type> {
    match ty.inner {
        TypeInner::Struct { .. } => {
            let ident = Ident::new(ty.name.as_ref().unwrap(), Span::call_site());

            Some(parse_quote!(self::#ident))
        }
        _ => rust_type_inner(module, &ty.inner),
    }
}

pub fn rust_type_inner(module: &Module, ty: &TypeInner) -> Option<syn::Type> {
    let kiln_path = kiln_path();

    Some(match *ty {
        TypeInner::Scalar { kind, width } => rust_type_scalar(kind, width),
        TypeInner::Matrix {
            columns,
            rows,
            width,
        } => {
            let base_type = rust_type_scalar(ScalarKind::Float, width);

            let columns = vector_size(columns);
            let rows = vector_size(rows);

            let ident = Ident::new(&format!("Mat{}x{}", columns, rows), Span::call_site());

            parse_quote!(#kiln_path::#ident<#base_type>)
        }
        TypeInner::Vector { kind, size, width } => {
            let base_type = rust_type_scalar(kind, width);

            match size {
                VectorSize::Bi => parse_quote!(#kiln_path::Vec2<#base_type>),
                VectorSize::Tri => parse_quote!(#kiln_path::Vec3<#base_type>),
                VectorSize::Quad => parse_quote!(#kiln_path::Vec4<#base_type>),
            }
        }
        TypeInner::Array { base, size, .. } => {
            let base_ty = &module.types[base];
            let base_ty = rust_type(module, base_ty);

            match size {
                ArraySize::Constant(constant) => {
                    let size = rust_constant(module, &constant);

                    parse_quote!([#base_ty; #size])
                }
                ArraySize::Dynamic => parse_quote!([#base_ty]),
            }
        }
        _ => return None,
    })
}

pub fn vector_size(size: VectorSize) -> u8 {
    match size {
        VectorSize::Bi => 2,
        VectorSize::Tri => 3,
        VectorSize::Quad => 4,
    }
}

pub fn rust_type_scalar(kind: ScalarKind, width: u8) -> syn::Type {
    match kind {
        ScalarKind::Bool => parse_quote!(::std::primitive::bool),
        ScalarKind::Float => match width {
            4 => parse_quote!(::std::primitive::f32),
            8 => parse_quote!(::std::primitive::f64),
            _ => unreachable!(),
        },
        ScalarKind::Sint => match width {
            1 => parse_quote!(::std::primitive::i8),
            2 => parse_quote!(::std::primitive::i16),
            4 => parse_quote!(::std::primitive::i32),
            8 => parse_quote!(::std::primitive::i64),
            _ => unreachable!(),
        },
        ScalarKind::Uint => match width {
            1 => parse_quote!(::std::primitive::u8),
            2 => parse_quote!(::std::primitive::u16),
            4 => parse_quote!(::std::primitive::u32),
            8 => parse_quote!(::std::primitive::u64),
            _ => unreachable!(),
        },
    }
}
