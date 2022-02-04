mod generic_buffer;
mod uniform_buffer;

use std::ops::{Bound, RangeBounds};

pub use generic_buffer::*;
pub use uniform_buffer::*;

use crate::{Id, RawBuffer};

pub type BufferId = Id<RawBuffer>;

impl BufferId {
    pub fn slice<S: RangeBounds<u64>>(self, bounds: S) -> BufferSlice {
        BufferSlice {
            buffer: self,
            start: bounds.start_bound().cloned(),
            end: bounds.end_bound().cloned(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferSlice {
    buffer: BufferId,
    start: Bound<u64>,
    end: Bound<u64>,
}

impl BufferSlice {
    pub fn buffer(&self) -> BufferId {
        self.buffer
    }

    pub fn start(&self) -> Bound<u64> {
        self.start
    }

    pub fn end(&self) -> Bound<u64> {
        self.end
    }
}
