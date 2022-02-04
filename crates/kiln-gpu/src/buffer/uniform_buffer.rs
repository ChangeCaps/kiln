use wgpu::util::{DeviceExt, DownloadBuffer};

use crate::{BufferId, GpuDevice, GpuInstance, GpuResources, GpuSync, GpuSyncState, Id, RawBuffer};

use std::{
    cell::UnsafeCell,
    mem,
    ops::{Deref, DerefMut},
    ptr, slice,
};

/// Signifies that an object can safely be used in a [`UniformBuffer`].
///
/// # Safety
/// * must be save to transmute from and to it's byte representation
/// * must not have any invalid bit patterns eg. no [`bool`]
pub unsafe trait BufferData: Sized + Copy {
    fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const _ as *const u8, mem::size_of::<Self>()) }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), mem::size_of::<Self>());
        unsafe { ptr::read_unaligned(bytes.as_ptr() as *const Self) }
    }
}

pub struct UniformBuffer<T: BufferData> {
    device: GpuDevice,
    resources: GpuResources,
    sync: GpuSync,
    raw_buffer: Id<RawBuffer>,
    uniform: UnsafeCell<T>,
}

impl<T: BufferData> UniformBuffer<T> {
    pub fn new(instance: &GpuInstance, uniform: T) -> Self {
        let contents = uniform.as_bytes();

        let raw_buffer =
            instance
                .device
                .raw_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("kiln_uniform_buffer")),
                    contents,
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                });

        let raw_buffer = instance.resources.buffers.push(raw_buffer);

        Self {
            device: instance.device.clone(),
            resources: instance.resources.clone(),
            sync: GpuSync::new(None),
            raw_buffer,
            uniform: UnsafeCell::new(uniform),
        }
    }

    pub unsafe fn mark_gpu_changed(&self) {
        self.sync.mark_gpu();
    }

    pub fn sync(&self) {
        match self.sync.state() {
            Some(GpuSyncState::Cpu) => self.sync_cpu(),
            Some(GpuSyncState::Gpu) => self.sync_gpu(),
            None => {}
        }

        self.sync.mark_unchanged();
    }

    fn sync_cpu(&self) {
        let raw_buffer = self.resources.buffers.get(&self.raw_buffer).unwrap();

        self.device
            .raw_queue()
            .write_buffer(&raw_buffer, 0, self.bytes());
    }

    fn sync_gpu(&self) {
        let raw_buffer = self.resources.buffers.get(&self.raw_buffer).unwrap();

        let gpu_data = pollster::block_on(DownloadBuffer::read_buffer(
            self.device.raw_device(),
            self.device.raw_queue(),
            &raw_buffer.slice(..),
        ))
        .unwrap();

        unsafe { *self.uniform.get() = T::from_bytes(&gpu_data) };
    }

    pub fn raw_buffer(&self) -> BufferId {
        self.raw_buffer
    }

    pub fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<T: BufferData> Deref for UniformBuffer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.sync.is_changed_gpu() {
            self.sync_gpu();
            self.sync.mark_unchanged();
        }

        unsafe { &*self.uniform.get() }
    }
}

impl<T: BufferData> DerefMut for UniformBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.sync.is_changed_gpu() {
            self.sync_gpu();
            self.sync.mark_unchanged();
        }

        self.sync.mark_cpu();

        self.uniform.get_mut()
    }
}

impl<T: BufferData> Drop for UniformBuffer<T> {
    fn drop(&mut self) {
        self.resources.buffers.remove(&self.raw_buffer);
    }
}
