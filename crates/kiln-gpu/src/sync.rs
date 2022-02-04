use std::sync::atomic::{AtomicBool, Ordering};

pub enum GpuSyncState {
    Cpu,
    Gpu,
}

impl GpuSyncState {
    pub const fn is_cpu(self) -> bool {
        match self {
            Self::Cpu => true,
            Self::Gpu => false,
        }
    }

    pub const fn is_gpu(self) -> bool {
        match self {
            Self::Cpu => false,
            Self::Gpu => true,
        }
    }
}

pub struct GpuSync {
    changed: AtomicBool,
    is_cpu: AtomicBool,
}

impl GpuSync {
    pub const fn new(state: Option<GpuSyncState>) -> Self {
        let changed = state.is_some();
        let is_cpu = match state {
            Some(state) => state.is_cpu(),
            None => false,
        };

        Self {
            changed: AtomicBool::new(changed),
            is_cpu: AtomicBool::new(is_cpu),
        }
    }

    pub fn mark_changed(&self, state: GpuSyncState) {
        self.changed.store(true, Ordering::Release);
        self.is_cpu.store(state.is_cpu(), Ordering::Release);
    }

    pub fn mark_cpu(&self) {
        self.mark_changed(GpuSyncState::Cpu);
    }

    pub fn mark_gpu(&self) {
        self.mark_changed(GpuSyncState::Gpu);
    }

    pub fn mark_unchanged(&self) {
        self.changed.store(false, Ordering::Release);
    }

    pub fn state(&self) -> Option<GpuSyncState> {
        if self.changed.load(Ordering::Acquire) {
            if self.is_cpu.load(Ordering::Acquire) {
                Some(GpuSyncState::Cpu)
            } else {
                Some(GpuSyncState::Gpu)
            }
        } else {
            None
        }
    }

    pub fn is_changed_cpu(&self) -> bool {
        self.changed.load(Ordering::Acquire) && self.is_cpu.load(Ordering::Acquire)
    }

    pub fn is_changed_gpu(&self) -> bool {
        self.changed.load(Ordering::Acquire) && !self.is_cpu.load(Ordering::Acquire)
    }
}
