use crate::{VertexAttribute, VertexBufferLayout, VertexStepMode};

pub trait Vertex {
    fn array_stride() -> u64;

    fn attributes() -> Vec<VertexAttribute>;

    fn buffer_layout(step_mode: VertexStepMode) -> VertexBufferLayout {
        VertexBufferLayout {
            array_stride: Self::array_stride(),
            step_mode,
            attributes: Self::attributes(),
        }
    }
}
