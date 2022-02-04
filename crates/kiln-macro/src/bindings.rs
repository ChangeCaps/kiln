use naga::{
    valid::{GlobalUse, ModuleInfo, ShaderStages},
    EntryPoint, GlobalVariable, Handle, ImageClass, ImageDimension, Module, ResourceBinding,
    ScalarKind, ShaderStage, StorageAccess, StorageClass, StorageFormat, TypeInner,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Type};

use crate::{path::kiln_path, rust_type::rust_type, texture::rust_storage_format};

#[derive(Clone, PartialEq, Eq)]
pub enum BufferBindingType {
    Uniform,
    Storage { read_only: bool },
}

impl BufferBindingType {
    pub fn rust_buffer_binding_type(&self) -> TokenStream {
        let kiln_path = kiln_path();

        match self {
            Self::Uniform => quote!(#kiln_path::BufferBindingType::Uniform),
            Self::Storage { read_only } => {
                quote!(#kiln_path::BufferBindingType::Storage { read_only: #read_only })
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TextureSampleType {
    Float,
    Depth,
    Sint,
    Uint,
}

impl TextureSampleType {
    pub fn rust_type(&self) -> Type {
        let kiln_path = kiln_path();

        match self {
            Self::Float => parse_quote!(#kiln_path::Float),
            Self::Depth => parse_quote!(#kiln_path::Depth),
            Self::Sint => parse_quote!(#kiln_path::Sint),
            Self::Uint => parse_quote!(#kiln_path::Uint),
        }
    }

    pub fn rust_sample_type(&self) -> Type {
        let kiln_path = kiln_path();

        match self {
            Self::Float => parse_quote!(#kiln_path::TextureSampleType::Float { filterable: true }),
            Self::Depth => parse_quote!(#kiln_path::TextureSampleType::Depth),
            Self::Sint => parse_quote!(#kiln_path::TextureSampleType::Sint),
            Self::Uint => parse_quote!(#kiln_path::TextureSampleType::Uint),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum StorageTextureAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Clone, PartialEq, Eq)]
pub enum TextureViewDimension {
    D1,
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

impl TextureViewDimension {
    pub fn from_dim_arrayed(dim: ImageDimension, arrayed: bool) -> Self {
        match dim {
            ImageDimension::D1 => Self::D1,
            ImageDimension::D2 => Self::D2,
            ImageDimension::Cube => Self::Cube,
            ImageDimension::D3 => Self::D3,
        }
    }

    pub fn rust_dimension(&self) -> Type {
        let kiln_path = kiln_path();

        match self {
            Self::D1 => parse_quote!(#kiln_path::D1),
            Self::D2 => parse_quote!(#kiln_path::D2),
            Self::D2Array => parse_quote!(#kiln_path::D2Array),
            Self::Cube => parse_quote!(#kiln_path::Cube),
            Self::CubeArray => parse_quote!(#kiln_path::CubeArray),
            Self::D3 => parse_quote!(#kiln_path::D3),
        }
    }

    pub fn rust_view_dimension(&self) -> TokenStream {
        let kiln_path = kiln_path();

        match self {
            Self::D1 => parse_quote!(#kiln_path::TextureViewDimension::D1),
            Self::D2 => parse_quote!(#kiln_path::TextureViewDimension::D2),
            Self::D2Array => parse_quote!(#kiln_path::TextureViewDimension::D2Array),
            Self::Cube => parse_quote!(#kiln_path::TextureViewDimension::Cube),
            Self::CubeArray => parse_quote!(#kiln_path::TextureViewDimension::CubeArray),
            Self::D3 => parse_quote!(#kiln_path::TextureViewDimension::D3),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum BindingType {
    Buffer {
        rust_ty: Type,
        ty: BufferBindingType,
    },
    Sampler {
        comparison: bool,
    },
    Texture {
        sample_type: TextureSampleType,
        view_dimension: TextureViewDimension,
        multisampled: bool,
    },
    StorageTexture {
        access: StorageTextureAccess,
        format: StorageFormat,
        view_dimension: TextureViewDimension,
    },
}

impl BindingType {
    pub fn rust_type(&self) -> Type {
        let kiln_path = kiln_path();

        match self {
            Self::Buffer { rust_ty, .. } => parse_quote!(#kiln_path::UniformBuffer<#rust_ty>),
            Self::Sampler { comparison: false } => parse_quote!(#kiln_path::Sampler),
            Self::Sampler { comparison: true } => parse_quote!(#kiln_path::SamplerComparison),
            Self::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => {
                let rust_dimension = view_dimension.rust_dimension();
                let sample_type = sample_type.rust_type();

                parse_quote!(dyn #kiln_path::SampledTexture<
					#multisampled,
					Dimension = #rust_dimension, SampleType = #sample_type,
				>)
            }
            Self::StorageTexture {
                format,
                view_dimension,
                ..
            } => {
                let rust_dimension = view_dimension.rust_dimension();
                let format = rust_storage_format(format);

                parse_quote!(dyn #kiln_path::StoredTexture<
					Dimension = #rust_dimension, TexelFormat = #format,
				>)
            }
        }
    }

    pub fn rust_binding_type(&self) -> TokenStream {
        let kiln_path = kiln_path();

        match self {
            Self::Buffer { ty, .. } => {
                let ty = ty.rust_buffer_binding_type();

                quote! {
                    #kiln_path::BindingType::Buffer {
                        ty: #ty,
                        has_dynamic_offset: false,
                        min_binding_size: ::std::option::Option::None,
                    }
                }
            }
            Self::Sampler { comparison } => {
                quote! {
                    #kiln_path::BindingType::Sampler(#kiln_path::SamplerBindingType::Filtering)
                }
            }
            Self::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => {
                let sample_type = sample_type.rust_sample_type();
                let view_dimension = view_dimension.rust_view_dimension();

                quote! {
                    #kiln_path::BindingType::Texture {
                        sample_type: #sample_type,
                        view_dimension: #view_dimension,
                        multisampled: #multisampled,
                    }
                }
            }
            Self::StorageTexture {
                access,
                format,
                view_dimension,
            } => {
                let view_dimension = view_dimension.rust_view_dimension();

                quote! {
                    #kiln_path::BindingType::Texture {
                        view_dimension: #view_dimension,
                    }
                }
            }
        }
    }
}

pub fn rust_shader_stages(stages: ShaderStages) -> TokenStream {
    let kiln_path = kiln_path();

    if stages.contains(ShaderStages::COMPUTE) {
        quote!(#kiln_path::ShaderStages::COMPUTE)
    } else if stages.contains(ShaderStages::VERTEX) && stages.contains(ShaderStages::FRAGMENT) {
        quote!(#kiln_path::ShaderStages::VERTEX | #kiln_path::ShaderStages::FRAGMENT)
    } else if stages.contains(ShaderStages::VERTEX) {
        quote!(#kiln_path::ShaderStages::VERTEX)
    } else if stages.contains(ShaderStages::FRAGMENT) {
        quote!(#kiln_path::ShaderStages::FRAGMENT)
    } else {
        unreachable!("invalid shader stage access")
    }
}

#[derive(Clone)]
pub enum BindingAccess {
    Read,
    Write,
}

impl BindingAccess {
    pub fn rust_ref(&self) -> TokenStream {
        match self {
            Self::Read => quote!(&'a),
            Self::Write => quote!(&'a mut),
        }
    }
}

#[derive(Clone)]
pub struct EntryPointBinding {
    pub ident: Ident,
    pub name: String,
    pub ty: BindingType,
    pub access: BindingAccess,
    pub stages: ShaderStages,
    pub binding: ResourceBinding,
    pub write: bool,
}

impl EntryPointBinding {
    pub fn new(
        module: &Module,
        global_variable: &GlobalVariable,
        global_use: GlobalUse,
        stages: ShaderStages,
    ) -> Option<Self> {
        let name = global_variable.name.clone()?;
        let ident = Ident::new(&name, Span::call_site());
        let binding = global_variable.binding.clone()?;

        let ty = &module.types[global_variable.ty];
        let ty = match ty.inner {
            TypeInner::Image {
                dim,
                arrayed,
                class,
            } => match class {
                ImageClass::Depth { multi } => BindingType::Texture {
                    sample_type: TextureSampleType::Depth,
                    multisampled: multi,
                    view_dimension: TextureViewDimension::from_dim_arrayed(dim, arrayed),
                },
                ImageClass::Sampled { kind, multi } => {
                    let sample_type = match kind {
                        ScalarKind::Float => TextureSampleType::Float,
                        ScalarKind::Sint => TextureSampleType::Sint,
                        ScalarKind::Uint => TextureSampleType::Uint,
                        _ => unreachable!(),
                    };

                    BindingType::Texture {
                        sample_type,
                        multisampled: multi,
                        view_dimension: TextureViewDimension::from_dim_arrayed(dim, arrayed),
                    }
                }
                ImageClass::Storage { format, access } => {
                    let access = match access {
                        StorageAccess::LOAD => StorageTextureAccess::ReadOnly,
                        StorageAccess::STORE => StorageTextureAccess::WriteOnly,
                        _ => StorageTextureAccess::ReadWrite,
                    };

                    BindingType::StorageTexture {
                        format,
                        access,
                        view_dimension: TextureViewDimension::from_dim_arrayed(dim, arrayed),
                    }
                }
            },
            TypeInner::Sampler { comparison } => BindingType::Sampler { comparison },
            _ => {
                let rust_ty = rust_type(module, ty).unwrap();

                let ty = match global_variable.class {
                    StorageClass::Storage {
                        access: StorageAccess::LOAD,
                    } => BufferBindingType::Storage { read_only: true },
                    StorageClass::Storage { .. } => BufferBindingType::Storage { read_only: false },
                    StorageClass::Uniform => BufferBindingType::Uniform,
                    _ => unreachable!(),
                };

                BindingType::Buffer { rust_ty, ty }
            }
        };

        let access = if global_use.contains(GlobalUse::WRITE) {
            BindingAccess::Read
        } else if global_use.contains(GlobalUse::READ) {
            BindingAccess::Write
        } else {
            return None;
        };

        Some(Self {
            ident,
            name,
            ty,
            access,
            stages,
            binding,
            write: global_use.contains(GlobalUse::WRITE),
        })
    }

    pub fn rust_binding_resource(&self) -> TokenStream {
        let kiln_path = kiln_path();

        match self.ty {
            BindingType::Buffer { .. } => {
                let ident = &self.ident;

                quote! {
                    #kiln_path::BindingResource::Buffer(#kiln_path::BufferBinding {
                        buffer: self.#ident.raw_buffer(),
                        offset: 0u64,
                        size: ::std::option::Option::None,
                    })
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct EntryPointBindings {
    pub bindings: Vec<EntryPointBinding>,
}

impl EntryPointBindings {
    pub fn from_entry_point(
        module: &Module,
        module_info: &ModuleInfo,
        entry_point: &EntryPoint,
    ) -> Self {
        let index = module
            .entry_points
            .iter()
            .position(|ep| ep.name == entry_point.name)
            .expect("entry point not found");

        let function_info = module_info.get_entry_point(index);

        let mut bindings = Vec::new();

        for (global_variable_handle, global_variable) in module.global_variables.iter() {
            let global_use = &function_info[global_variable_handle];

            let stages = match entry_point.stage {
                ShaderStage::Compute => ShaderStages::COMPUTE,
                ShaderStage::Fragment => ShaderStages::FRAGMENT,
                ShaderStage::Vertex => ShaderStages::VERTEX,
            };

            if let Some(binding) =
                EntryPointBinding::new(module, global_variable, *global_use, stages)
            {
                bindings.push(binding);
            }
        }

        Self { bindings }
    }

    pub fn binding_compatible(&mut self, new_binding: EntryPointBinding) -> bool {
        for binding in &mut self.bindings {
            if binding.name == new_binding.name && binding.binding != new_binding.binding {
                return false;
            }

            if binding.binding == new_binding.binding {
                if binding.name != new_binding.name || binding.ty != new_binding.ty {
                    return false;
                }

                binding.write |= new_binding.write;
                binding.stages |= new_binding.stages;

                return true;
            }
        }

        self.bindings.push(new_binding);

        true
    }

    pub fn combined(mut self, other: Self) -> Option<Self> {
        for binding in other.bindings {
            if !self.binding_compatible(binding) {
                return None;
            }
        }

        Some(self)
    }

    pub fn bind_group_layout_descriptors(&self) -> impl Iterator<Item = TokenStream> + '_ {
        let mut groups = Vec::new();

        for binding in &self.bindings {
            let group = binding.binding.group as usize;
            groups.resize_with(groups.len().max(group + 1), Vec::new);

            groups[group].push(binding.clone());
        }

        let kiln_path = kiln_path();
        groups.into_iter().map(move |bindings| {
            let bindings = bindings.iter().map(|binding| {
                let _binding = binding.binding.binding;
                let shader_stages = rust_shader_stages(binding.stages);
                let binding_type = binding.ty.rust_binding_type();

                quote! {
                    #kiln_path::BindGroupLayoutEntry {
                        binding: #_binding,
                        visibility: #shader_stages,
                        ty: #binding_type,
                        count: ::std::option::Option::None,
                    },
                }
            });

            quote! {
                #kiln_path::BindGroupLayoutDescriptor {
                    label: None,
                    entries: ::std::vec![
                        #(#bindings)*
                    ],
                }
            }
        })
    }

    pub fn bind_group_descriptors(&self) -> impl Iterator<Item = TokenStream> + '_ {
        let mut groups = Vec::new();

        for binding in &self.bindings {
            let group = binding.binding.group as usize;
            groups.resize_with(groups.len().max(group + 1), Vec::new);

            groups[group].push(binding.clone());
        }

        let kiln_path = kiln_path();
        groups
            .into_iter()
            .enumerate()
            .map(move |(group, bindings)| {
                let bindings = bindings.iter().map(|binding| {
                    let _binding = binding.binding.binding;
                    let resource = binding.rust_binding_resource();

                    quote! {
                        #kiln_path::BindGroupEntry {
                            binding: #_binding,
                            resource: #resource,
                        },
                    }
                });

                quote! {
                    #kiln_path::BindGroupDescriptor {
                        label: None,
                        layout: bind_group_layouts[#group],
                        entries: ::std::vec![
                            #(#bindings)*
                        ],
                    }
                }
            })
    }
}
