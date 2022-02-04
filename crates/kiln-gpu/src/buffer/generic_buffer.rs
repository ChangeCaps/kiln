use std::ops::{Deref, DerefMut};

use crate::{GpuDevice, GpuInstance, GpuResources, GpuSync, GpuSyncState, Id, RawBuffer};

pub struct GenericBuffer {
    device: GpuDevice,
    resources: GpuResources,
    data: Vec<u8>,
    sync: GpuSync,
    buffer_size: u64,
    raw_buffer: Option<Id<RawBuffer>>,
}

impl GenericBuffer {
    pub fn new(instance: &GpuInstance) -> Self {
        Self {
            device: instance.device.clone(),
            resources: instance.resources.clone(),
            data: Vec::new(),
            sync: GpuSync::new(None),
            buffer_size: 0,
            raw_buffer: None,
        }
    }

    fn sync(&self, state: GpuSyncState) {
        match state {
            GpuSyncState::Cpu => self.sync_cpu(),
            GpuSyncState::Gpu => self.sync_gpu(),
        }
    }

    // sync from cpu to gpu
    fn sync_cpu(&self) {}

    // sync from gpu to cpu
    fn sync_gpu(&self) {}
}

impl Deref for GenericBuffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for GenericBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.sync.mark_cpu();

        &mut self.data
    }
}
