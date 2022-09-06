use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

use bytemuck::{Pod, Zeroable};

use wgpu::ShaderModule;

use crate::{
    error::{Error, Result},
    shader_processor::ShaderProcessor,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct ShaderUniforms {
    pub view: [[f32; 4]; 4],
    pub aspect: f32,
    pub time: f32,
}

#[derive(Debug)]
pub struct Shader {
    pub vertex_path: Option<PathBuf>,
    pub fragment_path: PathBuf,
    pub last_modified: SystemTime,
    pub uniforms_group: wgpu::BindGroup,
    pub uniforms_buffer: wgpu::Buffer,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub pipeline: wgpu::RenderPipeline,
}

impl Shader {
    pub fn new(
        device: &wgpu::Device,
        processor: &mut ShaderProcessor,
        vertex_path: Option<PathBuf>,
        fragment_path: PathBuf,
    ) -> Result<Self> {
        if !fragment_path.exists() {
            return Err(Error::InvalidPath(fragment_path));
        }
        let meta = fragment_path.metadata()?;

        let uniforms_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("kiln-uniforms-layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("kiln-pipeline-layout"),
            bind_group_layouts: &[&uniforms_layout],
            push_constant_ranges: &[],
        });

        let uniforms_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("kiln-uniforms-buffer"),
            size: 256,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        let uniforms_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("kiln-uniforms-group"),
            layout: &uniforms_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms_buffer.as_entire_binding(),
            }],
        });

        let (vertex_module, fragment_module) =
            Self::load_shaders(device, processor, vertex_path.as_deref(), &fragment_path)?;
        let pipeline =
            Self::create_pipeline(&vertex_module, &fragment_module, device, &pipeline_layout);

        Ok(Self {
            vertex_path,
            fragment_path,
            last_modified: meta.modified()?,
            uniforms_group,
            uniforms_buffer,
            pipeline_layout,
            pipeline,
        })
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        processor: &mut ShaderProcessor,
    ) -> Result<bool> {
        let modified = self.fragment_path.metadata()?.modified()?;

        if modified > self.last_modified {
            self.last_modified = modified;

            processor.invalidate_locals();
            let (vertex_module, fragment_module) = Self::load_shaders(
                device,
                processor,
                self.vertex_path.as_deref(),
                &self.fragment_path,
            )?;

            self.pipeline = Self::create_pipeline(
                &vertex_module,
                &fragment_module,
                device,
                &self.pipeline_layout,
            );

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn write_uniforms(&self, queue: &wgpu::Queue, uniforms: &ShaderUniforms) {
        let bytes = bytemuck::bytes_of(uniforms);
        queue.write_buffer(&self.uniforms_buffer, 0, bytes);
    }

    fn load_shaders(
        device: &wgpu::Device,
        processor: &mut ShaderProcessor,
        vertex_path: Option<&Path>,
        fragment_path: &Path,
    ) -> Result<(ShaderModule, ShaderModule)> {
        let fragment_source = processor.process(&fragment_path)?;
        let fragment_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("kiln-shader"),
            source: wgpu::ShaderSource::Wgsl(fragment_source.into()),
        });

        let vertex_module = if let Some(vertex_path) = vertex_path {
            let vertex_source = processor.process(&vertex_path)?;

            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("kiln-shader"),
                source: wgpu::ShaderSource::Wgsl(vertex_source.into()),
            })
        } else {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("kiln-shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("include/default_vertex.wgsl").into(),
                ),
            })
        };

        Ok((vertex_module, fragment_module))
    }

    fn create_pipeline(
        vertex_module: &wgpu::ShaderModule,
        fragment_module: &wgpu::ShaderModule,
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("kiln-shader-pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: vertex_module,
                entry_point: "vert",
                buffers: &[],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: fragment_module,
                entry_point: "frag",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        })
    }
}
