use std::{
    alloc,
    alloc::Layout,
    marker::PhantomData,
    mem,
    num::NonZeroU32,
    ptr::{self, NonNull},
    slice,
    sync::atomic::{AtomicPtr, Ordering},
};

use crate::{TextureData, TextureFormat};

pub unsafe trait TextureStorage {
    type Data: TextureData;

    fn extent(&self) -> wgpu::Extent3d;

    fn bytes_per_row(&self) -> Option<NonZeroU32> {
        NonZeroU32::new(bytes_per_row::<Self::Data>(self.extent().width as usize) as u32)
    }

    fn size(&self) -> usize;

    fn ptr(&self) -> *mut u8;

    fn bytes(&self) -> &[u8];

    fn index(&self, x: u32, y: u32, z: u32) -> *mut Self::Data {
        let extent = self.extent();

        assert!(x < extent.width && y < extent.height && z < extent.depth_or_array_layers);

        let bytes_per_row = bytes_per_row::<Self::Data>(extent.width as usize);
        let rows_per_image = extent.height as usize;
        let bytes_per_image = rows_per_image * bytes_per_row;

        let image_ptr = unsafe { (self.ptr() as *mut u8).add(z as usize * bytes_per_image) };
        let row_ptr = unsafe { image_ptr.add(y as usize * bytes_per_row) as *mut Self::Data };
        let ptr = unsafe { row_ptr.add(x as usize) };

        ptr
    }
}

pub trait TextureDimension<Format: TextureFormat>: Send + Sync {
    type Storage: TextureStorage;
}

fn bytes_per_row<Data: TextureData>(width: usize) -> usize {
    let row_layout = Layout::array::<Data>(width)
        .unwrap()
        .align_to(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize)
        .unwrap()
        .pad_to_align();

    row_layout.size()
}

struct TextureStorageData<D> {
    data: AtomicPtr<D>,
    layout: Layout,
    marker: PhantomData<D>,
}

unsafe impl<D: Send> Send for TextureStorageData<D> {}
unsafe impl<D: Sync> Sync for TextureStorageData<D> {}

impl<D> TextureStorageData<D> {
    pub const unsafe fn new(layout: Layout) -> Self {
        Self {
            data: AtomicPtr::new(ptr::null_mut()),
            layout,
            marker: PhantomData,
        }
    }

    pub fn allocate(&self) {
        if !self.data.load(Ordering::Acquire).is_null() {
            return;
        }

        let ptr = unsafe { alloc::alloc_zeroed(self.layout) };

        let ptr = match NonNull::new(ptr) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(self.layout),
        };

        self.data.store(ptr.cast().as_ptr(), Ordering::Release);
    }

    pub fn size(&self) -> usize {
        self.layout.size()
    }

    pub fn ptr(&self) -> *mut D {
        self.allocate();

        self.data.load(Ordering::Acquire)
    }

    pub fn bytes(&self) -> &[u8] {
        self.allocate();

        unsafe { slice::from_raw_parts(self.ptr() as *const u8, self.layout.size()) }
    }
}

pub struct TextureStorageD1<D: TextureData> {
    width: u32,
    data: TextureStorageData<D>,
}

unsafe impl<D: TextureData> TextureStorage for TextureStorageD1<D> {
    type Data = D;

    fn extent(&self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.width,
            height: 1,
            depth_or_array_layers: 1,
        }
    }

    fn size(&self) -> usize {
        self.data.size()
    }

    fn ptr(&self) -> *mut u8 {
        self.data.ptr() as *mut u8
    }

    fn bytes(&self) -> &[u8] {
        self.data.bytes()
    }
}

pub struct D1;

impl<F: TextureFormat> TextureDimension<F> for D1 {
    type Storage = TextureStorageD1<F::Data>;
}

pub struct TextureStorageD2<D: TextureData> {
    width: u32,
    height: u32,
    data: TextureStorageData<D>,
}

impl<D: TextureData> TextureStorageD2<D> {
    pub fn new(width: u32, height: u32) -> Self {
        assert!(mem::size_of::<D>() <= wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize);

        let layout = Layout::from_size_align(
            bytes_per_row::<D>(width as usize) * height as usize,
            mem::align_of::<D>(),
        )
        .unwrap();

        Self {
            width,
            height,
            data: unsafe { TextureStorageData::new(layout) },
        }
    }
}

unsafe impl<D: TextureData> TextureStorage for TextureStorageD2<D> {
    type Data = D;

    fn extent(&self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        }
    }

    fn size(&self) -> usize {
        self.data.size()
    }

    fn ptr(&self) -> *mut u8 {
        self.data.ptr() as *mut u8
    }

    fn bytes(&self) -> &[u8] {
        self.data.bytes()
    }
}

pub struct D2;

impl<D: TextureFormat> TextureDimension<D> for D2 {
    type Storage = TextureStorageD2<D::Data>;
}
