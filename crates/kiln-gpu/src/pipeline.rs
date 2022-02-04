use std::ops::Deref;

use wgpu::PushConstantRange;

use crate::{
    BindGroupLayoutId, GpuInstance, GpuResourceDescriptor, Id, PipelineLayout,
    RawPipelineLayoutDescriptor,
};

pub type PipelineLayoutId = Id<PipelineLayout>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PipelineLayoutDescriptor {
    pub label: Option<String>,
    pub bind_group_layouts: Vec<BindGroupLayoutId>,
    pub push_constant_ranges: Vec<PushConstantRange>,
}

impl GpuResourceDescriptor for PipelineLayoutDescriptor {
    type Resource = PipelineLayout;
}

impl PipelineLayoutDescriptor {
    pub fn create_pipeline_layout(&self, instance: &GpuInstance) -> PipelineLayout {
        let bind_group_layouts = self
            .bind_group_layouts
            .iter()
            .map(|id| instance.resources.bind_group_layouts.get_id(id).unwrap())
            .collect::<Vec<_>>();

        let bind_group_layouts = bind_group_layouts
            .iter()
            .map(Deref::deref)
            .collect::<Vec<_>>();

        let desc = RawPipelineLayoutDescriptor {
            label: self.label.as_ref().map(AsRef::as_ref),
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &self.push_constant_ranges,
        };

        instance.device.raw_device().create_pipeline_layout(&desc)
    }
}
