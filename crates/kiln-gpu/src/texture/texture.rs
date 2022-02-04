use std::ops::Index;

use crate::{
    GpuDevice, GpuInstance, GpuResources, GpuSync, GpuSyncState, HasId, Id, RawTexture,
    RawTextureFormat, TextureDimension, TextureFormat, TextureStorage, TextureStorageD2, D1, D2,
};

pub type TextureId = Id<RawTexture>;

pub struct Texture<F, D, const MS: bool>
where
    F: TextureFormat,
    D: TextureDimension<F>,
{
    device: GpuDevice,
    resources: GpuResources,
    format: F,
    storage: D::Storage,
    samples: u32,
    id: TextureId,
    sync: GpuSync,
}

impl<F, D, const MS: bool> HasId<RawTexture> for Texture<F, D, MS>
where
    F: TextureFormat,
    D: TextureDimension<F>,
{
    fn id(&self) -> Id<RawTexture> {
        self.id
    }
}

impl<F, D, const MS: bool> Texture<F, D, MS>
where
    F: TextureFormat,
    D: TextureDimension<F>,
{
    pub fn sync(&self) {
        match self.sync.state() {
            Some(GpuSyncState::Cpu) => self.sync_cpu(),
            Some(GpuSyncState::Gpu) => self.sync_gpu(),
            None => {}
        }

        self.sync.mark_unchanged();
    }

    fn sync_cpu(&self) {
        let size = self.storage.size();

        if size == 0 {
            return;
        }

        let texture = self.resources.textures.get(&self.id).unwrap();

        self.device.raw_queue().write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            self.storage.bytes(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: self.storage.bytes_per_row(),
                rows_per_image: None,
            },
            self.storage.extent(),
        );
    }

    fn sync_gpu(&self) {
        let size = self.storage.size();

        if size == 0 {
            return;
        }
    }

    pub unsafe fn mark_gpu(&self) {
        self.sync.mark_gpu();
    }

    pub fn raw_texture(&self) -> Id<RawTexture> {
        self.id
    }

    pub fn raw_format(&self) -> RawTextureFormat {
        self.format.format()
    }

    pub fn samples(&self) -> u32 {
        if MS {
            self.samples
        } else {
            1
        }
    }
}

impl<F, D, const MS: bool> Index<u32> for Texture<F, D, MS>
where
    F: TextureFormat,
    D: TextureDimension<F>,
    D::Storage: TextureStorage<Data = F::Data>,
{
    type Output = F::Data;

    fn index(&self, index: u32) -> &Self::Output {
        if self.sync.is_changed_gpu() {
            self.sync_gpu();
        }

        unsafe { &*self.storage.index(index, 0, 0) }
    }
}

impl<F, D, const MS: bool> Index<(u32, u32)> for Texture<F, D, MS>
where
    F: TextureFormat,
    D: TextureDimension<F>,
    D::Storage: TextureStorage<Data = F::Data>,
{
    type Output = F::Data;

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        if self.sync.is_changed_gpu() {
            self.sync_gpu();
        }

        unsafe { &*self.storage.index(x, y, 0) }
    }
}

impl<F, D, const MS: bool> Index<(u32, u32, u32)> for Texture<F, D, MS>
where
    F: TextureFormat,
    D: TextureDimension<F>,
    D::Storage: TextureStorage<Data = F::Data>,
{
    type Output = F::Data;

    fn index(&self, (x, y, z): (u32, u32, u32)) -> &Self::Output {
        if self.sync.is_changed_gpu() {
            self.sync_gpu();
        }

        unsafe { &*self.storage.index(x, y, z) }
    }
}

pub type Texture1d<F> = Texture<F, D1, false>;
pub type Texture2d<F> = Texture<F, D2, false>;

pub type Texture1dMs<F> = Texture<F, D1, true>;
pub type Texture2dMs<F> = Texture<F, D2, true>;

impl<F: TextureFormat + Default> Texture2d<F> {
    pub fn new(instance: &GpuInstance, width: u32, height: u32) -> Self {
        let format = F::default();

        let storage = TextureStorageD2::new(width, height);

        let texture = instance
            .device
            .raw_device()
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("kiln_texture_2d"),
                size: storage.extent(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: format.format(),
                usage: wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
            });

        let id = instance.resources.textures.push(texture);

        Self {
            device: instance.device.clone(),
            resources: instance.resources.clone(),
            format,
            storage,
            samples: 1,
            id,
            sync: GpuSync::new(None),
        }
    }
}
