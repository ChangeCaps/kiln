use crate::{
    BindGroupDescriptor, BindGroupId, BindGroupLayoutDescriptor, BindGroupLayoutId,
    ComputePipelineDescriptor, ComputePipelineId, GpuResources, PipelineLayoutDescriptor,
    PipelineLayoutId, RawDevice, RawQueue, ShaderModuleDescriptor, ShaderModuleId,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct GpuDevice {
    device: Arc<RawDevice>,
    queue: Arc<RawQueue>,
}

impl GpuDevice {
    pub fn from_raw(device: RawDevice, queue: RawQueue) -> Self {
        Self {
            device: device.into(),
            queue: queue.into(),
        }
    }

    pub fn raw_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn raw_queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

#[derive(Clone)]
pub struct GpuInstance {
    pub device: GpuDevice,
    pub resources: GpuResources,
}

impl GpuInstance {
    pub fn new_headless() -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("kiln_device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ))
        .unwrap();

        let gpu_device = GpuDevice::from_raw(device, queue);

        Self::from_device(gpu_device)
    }

    pub fn from_device(device: GpuDevice) -> Self {
        Self {
            device,
            resources: Default::default(),
        }
    }

    pub fn create_bind_group_layout(&self, desc: BindGroupLayoutDescriptor) -> BindGroupLayoutId {
        if let Some(id) = self.resources.bind_group_layouts.get_desc(&desc) {
            return id;
        }

        let bind_group = desc.create_bind_group_layout(self);

        self.resources
            .bind_group_layouts
            .push_descriptor(desc, bind_group)
    }

    pub fn create_bind_group(&self, desc: BindGroupDescriptor) -> BindGroupId {
        if let Some(id) = self.resources.bind_groups.get_desc(&desc) {
            return id;
        }

        let bind_group = desc.create_bind_group(self);

        self.resources.bind_groups.push_descriptor(desc, bind_group)
    }

    pub fn create_pipeline_layout(&self, desc: PipelineLayoutDescriptor) -> PipelineLayoutId {
        if let Some(id) = self.resources.pipeline_layouts.get_desc(&desc) {
            return id;
        }

        let bind_group = desc.create_pipeline_layout(self);

        self.resources
            .pipeline_layouts
            .push_descriptor(desc, bind_group)
    }

    pub fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> ShaderModuleId {
        if let Some(id) = self.resources.shader_modules.get_desc(&desc) {
            return id;
        }

        let bind_group = desc.create_shader_module(self);

        self.resources
            .shader_modules
            .push_descriptor(desc, bind_group)
    }

    pub fn create_compute_pipeline(&self, desc: ComputePipelineDescriptor) -> ComputePipelineId {
        if let Some(id) = self.resources.compute_pipelines.get_desc(&desc) {
            return id;
        }

        let bind_group = desc.create_compute_pipeline(self);

        self.resources
            .compute_pipelines
            .push_descriptor(desc, bind_group)
    }
}
