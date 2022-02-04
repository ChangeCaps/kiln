use std::borrow::Cow;

use crate::{GpuInstance, GpuResourceDescriptor, Id, RawShaderModuleDescriptor, ShaderModule};

pub type ShaderModuleId = Id<ShaderModule>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShaderModuleDescriptor {
    pub label: Option<String>,
    pub source: Cow<'static, str>,
}

impl GpuResourceDescriptor for ShaderModuleDescriptor {
    type Resource = ShaderModule;
}

impl ShaderModuleDescriptor {
    pub fn create_shader_module(&self, instance: &GpuInstance) -> ShaderModule {
        let desc = RawShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(self.source.clone()),
        };

        instance.device.raw_device().create_shader_module(&desc)
    }
}
