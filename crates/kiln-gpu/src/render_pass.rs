use wgpu::{Color, Operations};

use crate::TextureViewId;

pub struct RenderPassColorAttachment {
    pub view: TextureViewId,
    pub resolve_target: Option<TextureViewId>,
    pub operations: Operations<Color>,
}

pub struct RenderPassDepthStencilAttachment {
    pub view: TextureViewId,
    pub depth_ops: Option<Operations<f32>>,
    pub stencil_ops: Option<Operations<u32>>,
}

pub struct RenderPassDescriptor {
    pub label: Option<String>,
    pub color_attachments: Vec<RenderPassColorAttachment>,
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment>,
}
