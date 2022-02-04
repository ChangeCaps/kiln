use naga::{valid::ModuleInfo, EntryPoint, Module, ShaderStage};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{
    bindings::{rust_shader_stages, EntryPointBindings},
    path::kiln_path,
};

pub struct EntryPointDesc {
    pub ident: Ident,
    pub name: String,
    pub shader_stage: ShaderStage,
    pub bindings: EntryPointBindings,
}

impl EntryPointDesc {
    pub fn new(module: &Module, module_info: &ModuleInfo, entry_point: &EntryPoint) -> Self {
        let ident = Ident::new(&entry_point.name, Span::call_site());
        let name = entry_point.name.clone();

        Self {
            ident,
            name,
            shader_stage: entry_point.stage,
            bindings: EntryPointBindings::from_entry_point(module, module_info, entry_point),
        }
    }

    pub fn single_binding(&self, source: &str, bindings: &EntryPointBindings) -> TokenStream {
        let name = &self.name;
        let ident = &self.ident;

        let fields = bindings.bindings.iter().map(|binding| {
            let ident = &binding.ident;
            let rust_ref = binding.access.rust_ref();
            let ty = binding.ty.rust_type();

            quote! {
                pub #ident: #rust_ref #ty,
            }
        });

        let lifetime = if self.bindings.bindings.is_empty() {
            None
        } else {
            Some(quote!(<'a>))
        };

        let kiln_path = kiln_path();

        let bind_group_layout_descriptors = bindings.bind_group_layout_descriptors();
        let bind_group_descriptors = bindings.bind_group_descriptors();

        let entry_point_impl = quote! {
            impl #lifetime #kiln_path::EntryPoint for #ident #lifetime {
                fn source(&self) -> &'static ::std::primitive::str {
                    #source
                }

                fn entry_point(&self) -> &'static ::std::primitive::str {
                    #name
                }

                fn bind_group_layout_descriptors(&self) -> ::std::vec::Vec<#kiln_path::BindGroupLayoutDescriptor> {
                    ::std::vec![
                        #(#bind_group_layout_descriptors)*
                    ]
                }

                fn bind_group_descriptors(
                    &self,
                    bind_group_layouts: &[#kiln_path::BindGroupLayoutId]
                ) -> ::std::vec::Vec<#kiln_path::BindGroupDescriptor> {
                    ::std::vec![
                        #(#bind_group_descriptors)*
                    ]
                }
            }
        };

        let specific_impl = match self.shader_stage {
            ShaderStage::Compute => quote! {
                impl #lifetime #kiln_path::ComputeEntryPoint for #ident #lifetime {}
            },
            _ => unreachable!(),
        };

        quote! {
            pub struct #ident #lifetime {
                #(#fields)*
            }

            #entry_point_impl

            #specific_impl
        }
    }

    pub fn vertex_fragment_binding(&self, other: &Self, source: &str) -> Option<TokenStream> {
        let ident = Ident::new(&format!("{}_{}", self.name, other.name), Span::call_site());

        let bindings = self.bindings.clone().combined(other.bindings.clone())?;

        Some(self.single_binding(source, &bindings))
    }
}

pub struct EntryPoints {
    pub entry_points: Vec<EntryPointDesc>,
}

impl EntryPoints {
    pub fn new(module: &Module, module_info: &ModuleInfo) -> Self {
        let mut entry_points = Vec::new();

        for entry_point in &module.entry_points {
            entry_points.push(EntryPointDesc::new(module, module_info, entry_point));
        }

        Self { entry_points }
    }

    pub fn bindings(&self, source: &str) -> TokenStream {
        let bindings =
            self.entry_points
                .iter()
                .map(|desc| match desc.shader_stage {
                    ShaderStage::Compute | ShaderStage::Vertex => {
                        desc.single_binding(source, &desc.bindings)
                    }
                    ShaderStage::Fragment => {
                        let bindings = self.entry_points.iter().filter_map(|other_desc| {
                            match other_desc.shader_stage {
                                ShaderStage::Vertex => {
                                    Some(other_desc.vertex_fragment_binding(desc, source))
                                }
                                _ => None,
                            }
                        });

                        quote! {
                            #(#bindings)*
                        }
                    }
                });

        quote! {
            #(#bindings)*
        }
    }
}
