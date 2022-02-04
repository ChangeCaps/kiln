use std::ops::Bound;

use crate::{
    BindGroup, BindGroupId, BufferSlice, ComputePass, ComputePipeline, ComputePipelineId,
    GpuInstance, IdRef, IndexFormat, RawBuffer, RawComputePass, RenderPass, RenderPassDescriptor,
    RenderPipeline, RenderPipelineId,
};

pub enum Command {
    BeginRenderPass {
        desc: RenderPassDescriptor,
        commands: RenderPassCommands,
    },
}

#[derive(Clone)]
pub enum ComputePassCommand {
    SetBindGroup {
        index: u32,
        bind_group: BindGroupId,
        offsets: Vec<u32>,
    },
    SetPipeline {
        pipeline: ComputePipelineId,
    },
    Dispatch {
        x: u32,
        y: u32,
        z: u32,
    },
}

impl ComputePassCommand {
    pub fn to_ref<'a>(&self, instance: &'a GpuInstance) -> RefComputePassCommand<'a> {
        match *self {
            Self::SetBindGroup {
                index,
                bind_group,
                ref offsets,
            } => RefComputePassCommand::SetBindGroup {
                index,
                bind_group: instance.resources.bind_groups.get_id(&bind_group).unwrap(),
                offsets: offsets.clone(),
            },
            Self::SetPipeline { pipeline } => RefComputePassCommand::SetPipeline {
                pipeline: instance
                    .resources
                    .compute_pipelines
                    .get_id(&pipeline)
                    .unwrap(),
            },
            Self::Dispatch { x, y, z } => RefComputePassCommand::Dispatch { x, y, z },
        }
    }
}

pub enum RefComputePassCommand<'a> {
    SetBindGroup {
        index: u32,
        bind_group: IdRef<'a, BindGroup>,
        offsets: Vec<u32>,
    },
    SetPipeline {
        pipeline: IdRef<'a, ComputePipeline>,
    },
    Dispatch {
        x: u32,
        y: u32,
        z: u32,
    },
}

impl<'a> RefComputePassCommand<'a> {
    pub fn execute(&'a self, compute_pass: &mut RawComputePass<'a>) {
        match *self {
            Self::SetBindGroup {
                index,
                ref bind_group,
                ref offsets,
            } => compute_pass.set_bind_group(index, &bind_group, &offsets),
            Self::SetPipeline { ref pipeline } => compute_pass.set_pipeline(&pipeline),
            Self::Dispatch { x, y, z } => compute_pass.dispatch(x, y, z),
        }
    }
}

#[derive(Clone, Default)]
pub struct ComputePassCommands {
    commands: Vec<ComputePassCommand>,
}

impl ComputePassCommands {
    pub fn to_ref_commands<'a>(&'a self, instance: &'a GpuInstance) -> RefComputePassCommands<'a> {
        let ref_commands = self
            .commands
            .iter()
            .map(|command| command.to_ref(instance))
            .collect::<Vec<_>>();

        RefComputePassCommands {
            commands: ref_commands,
        }
    }

    pub fn set_bind_group(&mut self, index: u32, bind_group: BindGroupId, offsets: Vec<u32>) {
        self.commands.push(ComputePassCommand::SetBindGroup {
            index,
            bind_group,
            offsets,
        });
    }

    pub fn set_pipeline(&mut self, pipeline: ComputePipelineId) {
        self.commands
            .push(ComputePassCommand::SetPipeline { pipeline });
    }

    pub fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        self.commands.push(ComputePassCommand::Dispatch { x, y, z });
    }
}

pub struct RefComputePassCommands<'a> {
    commands: Vec<RefComputePassCommand<'a>>,
}

impl<'a> RefComputePassCommands<'a> {
    pub fn execute(&'a self, compute_pass: &mut RawComputePass<'a>) {
        for command in &self.commands {
            command.execute(compute_pass);
        }
    }
}

#[derive(Clone)]
pub enum RenderPassCommand {
    SetBindGroup {
        index: u32,
        bind_group: BindGroupId,
        offsets: Vec<u32>,
    },
    SetPipeline {
        pipeline: RenderPipelineId,
    },
    SetIndexBuffer {
        buffer_slice: BufferSlice,
        format: IndexFormat,
    },
    SetVertexBuffer {
        slot: u32,
        buffer_slice: BufferSlice,
    },
    SetScissorRect {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    SetViewport {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        min_depth: f32,
        max_depth: f32,
    },
}

impl RenderPassCommand {
    fn to_ref<'a>(&self, instance: &'a GpuInstance) -> RefRenderPassCommand<'a> {
        match *self {
            Self::SetBindGroup {
                index,
                bind_group,
                ref offsets,
            } => RefRenderPassCommand::SetBindGroup {
                index,
                bind_group: instance.resources.bind_groups.get_id(&bind_group).unwrap(),
                offsets: offsets.clone(),
            },
            Self::SetPipeline { pipeline } => RefRenderPassCommand::SetPipeline {
                pipeline: instance
                    .resources
                    .render_pipelines
                    .get_id(&pipeline)
                    .unwrap(),
            },
            Self::SetIndexBuffer {
                buffer_slice,
                format,
            } => RefRenderPassCommand::SetIndexBuffer {
                buffer: instance
                    .resources
                    .buffers
                    .get(&buffer_slice.buffer())
                    .unwrap(),
                start: buffer_slice.start(),
                end: buffer_slice.end(),
                format,
            },
            Self::SetVertexBuffer { slot, buffer_slice } => RefRenderPassCommand::SetVertexBuffer {
                slot,
                buffer: instance
                    .resources
                    .buffers
                    .get(&buffer_slice.buffer())
                    .unwrap(),
                start: buffer_slice.start(),
                end: buffer_slice.end(),
            },
            Self::SetScissorRect {
                x,
                y,
                width,
                height,
            } => RefRenderPassCommand::SetScissorRect {
                x,
                y,
                width,
                height,
            },
            Self::SetViewport {
                x,
                y,
                w,
                h,
                min_depth,
                max_depth,
            } => RefRenderPassCommand::SetViewport {
                x,
                y,
                w,
                h,
                min_depth,
                max_depth,
            },
        }
    }
}

pub enum RefRenderPassCommand<'a> {
    SetBindGroup {
        index: u32,
        bind_group: IdRef<'a, BindGroup>,
        offsets: Vec<u32>,
    },
    SetPipeline {
        pipeline: IdRef<'a, RenderPipeline>,
    },
    SetIndexBuffer {
        buffer: IdRef<'a, RawBuffer>,
        start: Bound<u64>,
        end: Bound<u64>,
        format: IndexFormat,
    },
    SetVertexBuffer {
        slot: u32,
        buffer: IdRef<'a, RawBuffer>,
        start: Bound<u64>,
        end: Bound<u64>,
    },
    SetScissorRect {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    SetViewport {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        min_depth: f32,
        max_depth: f32,
    },
}

impl<'a> RefRenderPassCommand<'a> {
    pub fn execute(&'a self, render_pass: &mut RenderPass<'a>) {
        match self {
            Self::SetBindGroup {
                index,
                bind_group,
                offsets,
            } => {
                render_pass.set_bind_group(*index, bind_group, offsets);
            }
            Self::SetPipeline { pipeline } => {
                render_pass.set_pipeline(pipeline);
            }
            Self::SetIndexBuffer {
                buffer,
                start,
                end,
                format,
            } => {
                render_pass.set_index_buffer(buffer.slice((*start, *end)), *format);
            }
            Self::SetVertexBuffer {
                slot,
                buffer,
                start,
                end,
            } => {
                render_pass.set_vertex_buffer(*slot, buffer.slice((*start, *end)));
            }
            &Self::SetScissorRect {
                x,
                y,
                width,
                height,
            } => {
                render_pass.set_scissor_rect(x, y, width, height);
            }
            &Self::SetViewport {
                x,
                y,
                w,
                h,
                min_depth,
                max_depth,
            } => {
                render_pass.set_viewport(x, y, w, h, min_depth, max_depth);
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct RenderPassCommands {
    commands: Vec<RenderPassCommand>,
}

impl RenderPassCommands {
    pub fn to_ref_commands<'a>(&'a self, instance: &'a GpuInstance) -> RefRenderPassCommands<'a> {
        let ref_commands = self
            .commands
            .iter()
            .map(|command| command.to_ref(instance))
            .collect::<Vec<_>>();

        RefRenderPassCommands {
            commands: ref_commands,
        }
    }

    pub fn set_bind_group(&mut self, index: u32, bind_group: BindGroupId, offsets: Vec<u32>) {
        self.commands.push(RenderPassCommand::SetBindGroup {
            index,
            bind_group,
            offsets,
        });
    }

    pub fn set_pipeline(&mut self, pipeline: RenderPipelineId) {
        self.commands
            .push(RenderPassCommand::SetPipeline { pipeline });
    }

    pub fn set_index_buffer(&mut self, buffer_slice: BufferSlice, format: IndexFormat) {
        self.commands.push(RenderPassCommand::SetIndexBuffer {
            buffer_slice,
            format,
        });
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice) {
        self.commands
            .push(RenderPassCommand::SetVertexBuffer { slot, buffer_slice });
    }
}

pub struct RefRenderPassCommands<'a> {
    commands: Vec<RefRenderPassCommand<'a>>,
}

impl<'a> RefRenderPassCommands<'a> {
    pub fn execute(&'a self, render_pass: &mut RenderPass<'a>) {
        for command in &self.commands {
            command.execute(render_pass);
        }
    }
}
