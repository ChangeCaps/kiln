mod bind_group;
mod buffer;
pub mod color;
mod compute_pipeline;
mod entry_point;
mod id;
mod instance;
mod math;
mod pipeline;
mod render_commands;
mod render_pass;
mod render_pipeline;
mod resources;
mod sampler;
mod shader;
mod sync;
mod texture;
mod vertex;
mod vertex_attribute;

pub use bind_group::*;
pub use buffer::*;
pub use compute_pipeline::*;
pub use entry_point::*;
pub use id::*;
pub use instance::*;
pub use math::*;
pub use pipeline::*;
pub use render_commands::*;
pub use render_pass::*;
pub use render_pipeline::*;
pub use resources::*;
pub use sampler::*;
pub use shader::*;
pub use sync::*;
pub use texture::*;
pub use vertex::*;
pub use vertex_attribute::*;

pub use ordered_float::*;
pub use wgpu::{
    AddressMode, BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, BufferBindingType,
    Color, CompareFunction, ComputePipeline, FilterMode, IndexFormat, Operations, PipelineLayout,
    PushConstantRange, RenderPass, RenderPipeline, SamplerBorderColor, ShaderModule, ShaderStages,
    VertexAttribute, VertexFormat, VertexStepMode,
};

pub type RawBuffer = wgpu::Buffer;
pub type RawBufferSlice<'a> = wgpu::BufferSlice<'a>;
pub type RawDevice = wgpu::Device;
pub type RawQueue = wgpu::Queue;
pub type RawTexture = wgpu::Texture;
pub type RawTextureView = wgpu::TextureView;
pub type RawSampler = wgpu::Sampler;
pub type RawSamplerDescriptor<'a> = wgpu::SamplerDescriptor<'a>;
pub type RawBindGroupLayoutDescriptor<'a> = wgpu::BindGroupLayoutDescriptor<'a>;
pub type RawPipelineLayoutDescriptor<'a> = wgpu::PipelineLayoutDescriptor<'a>;
pub type RawRenderPipelineDescriptor<'a> = wgpu::RenderPipelineDescriptor<'a>;
pub type RawComputePipelineDescriptor<'a> = wgpu::ComputePipelineDescriptor<'a>;
pub type RawShaderModuleDescriptor<'a> = wgpu::ShaderModuleDescriptor<'a>;
pub type RawRenderPass<'a> = wgpu::RenderPass<'a>;
pub type RawComputePass<'a> = wgpu::ComputePass<'a>;
pub type RawTextureFormat = wgpu::TextureFormat;

pub use wgpu;
