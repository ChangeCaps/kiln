use std::ops::Deref;

use crate::{
    ComputePipeline, GpuInstance, GpuResourceDescriptor, Id, PipelineLayoutId,
    RawComputePipelineDescriptor, ShaderModuleId,
};

pub type ComputePipelineId = Id<ComputePipeline>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComputePipelineDescriptor {
    pub label: Option<String>,
    pub layout: Option<PipelineLayoutId>,
    pub module: ShaderModuleId,
    pub entry_point: String,
}

impl GpuResourceDescriptor for ComputePipelineDescriptor {
    type Resource = ComputePipeline;
}

impl ComputePipelineDescriptor {
    pub fn create_compute_pipeline(&self, instance: &GpuInstance) -> ComputePipeline {
        let layout = self
            .layout
            .map(|id| instance.resources.pipeline_layouts.get_id(&id).unwrap());

        let module = instance
            .resources
            .shader_modules
            .get_id(&self.module)
            .unwrap();

        let desc = RawComputePipelineDescriptor {
            label: self.label.as_ref().map(AsRef::as_ref),
            layout: layout.as_ref().map(Deref::deref),
            module: module.deref(),
            entry_point: &self.entry_point,
        };

        instance.device.raw_device().create_compute_pipeline(&desc)
    }
}
