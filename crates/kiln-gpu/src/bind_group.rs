use std::{num::NonZeroU64, ops::Deref};

use crate::{
    BindGroup, BindGroupLayout, BindGroupLayoutEntry, BufferId, GpuInstance, GpuResourceDescriptor,
    GpuResources, Id, IdRef, RawBuffer,
};

pub type BindGroupLayoutId = Id<BindGroupLayout>;
pub type BindGroupId = Id<BindGroup>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub label: Option<String>,
    pub entries: Vec<BindGroupLayoutEntry>,
}

impl GpuResourceDescriptor for BindGroupLayoutDescriptor {
    type Resource = BindGroupLayout;
}

impl BindGroupLayoutDescriptor {
    pub fn create_bind_group_layout(&self, instance: &GpuInstance) -> BindGroupLayout {
        let desc = wgpu::BindGroupLayoutDescriptor {
            label: self.label.as_ref().map(AsRef::as_ref),
            entries: &self.entries,
        };

        instance.device.raw_device().create_bind_group_layout(&desc)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BufferBinding {
    pub buffer: BufferId,
    pub offset: u64,
    pub size: Option<NonZeroU64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BindingResource {
    Buffer(BufferBinding),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource: BindingResource,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BindGroupDescriptor {
    pub label: Option<String>,
    pub layout: BindGroupLayoutId,
    pub entries: Vec<BindGroupEntry>,
}

impl GpuResourceDescriptor for BindGroupDescriptor {
    type Resource = BindGroup;
}

struct RefBufferBinding<'a> {
    pub buffer: IdRef<'a, RawBuffer>,
    pub offset: u64,
    pub size: Option<NonZeroU64>,
}

enum RefBindingResource<'a> {
    Buffer(RefBufferBinding<'a>),
}

impl<'a> RefBindingResource<'a> {
    fn from_binding_resource(resource: &BindingResource, resources: &'a GpuResources) -> Self {
        match resource {
            BindingResource::Buffer(buffer_binding) => Self::Buffer(RefBufferBinding {
                buffer: resources.buffers.get(&buffer_binding.buffer).unwrap(),
                offset: buffer_binding.offset,
                size: buffer_binding.size,
            }),
        }
    }

    fn to_raw_binding_resource(&'a self) -> wgpu::BindingResource<'a> {
        match self {
            Self::Buffer(buffer_binding) => wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: buffer_binding.buffer.deref(),
                offset: buffer_binding.offset,
                size: buffer_binding.size,
            }),
        }
    }
}

struct RefEntry<'a> {
    pub binding: u32,
    pub resource: RefBindingResource<'a>,
}

impl<'a> RefEntry<'a> {
    fn from_entry(entry: &BindGroupEntry, resources: &'a GpuResources) -> Self {
        Self {
            binding: entry.binding,
            resource: RefBindingResource::from_binding_resource(&entry.resource, resources),
        }
    }

    fn to_raw_entry(&'a self) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self.resource.to_raw_binding_resource(),
        }
    }
}

impl BindGroupDescriptor {
    pub fn create_bind_group(&self, instance: &GpuInstance) -> BindGroup {
        let entries = self
            .entries
            .iter()
            .map(|entry| RefEntry::from_entry(entry, &instance.resources))
            .collect::<Vec<_>>();

        let entries = entries
            .iter()
            .map(RefEntry::to_raw_entry)
            .collect::<Vec<_>>();

        let layout = &instance
            .resources
            .bind_group_layouts
            .get_id(&self.layout)
            .unwrap();

        let desc = wgpu::BindGroupDescriptor {
            label: self.label.as_ref().map(AsRef::as_ref),
            layout,
            entries: &entries,
        };

        instance.device.raw_device().create_bind_group(&desc)
    }
}
