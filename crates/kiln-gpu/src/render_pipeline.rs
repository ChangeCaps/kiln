use crate::{GpuResourceDescriptor, Id, RenderPipeline, VertexAttribute, VertexStepMode};

pub type RenderPipelineId = Id<RenderPipeline>;

pub struct VertexBufferLayout {
    pub array_stride: u64,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RenderPipelineDescriptor {}

impl GpuResourceDescriptor for RenderPipelineDescriptor {
    type Resource = RenderPipeline;
}
