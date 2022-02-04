use crate::{Id, RawTexture, Sampled, Stored, Texture, TextureDimension, TextureFormat};

pub trait SampledTexture<const MS: bool> {
    type Dimension;
    type SampleType;

    fn raw_texture(&self) -> Id<RawTexture>;

    fn write(&mut self);
}

impl<F, D, const MS: bool> SampledTexture<MS> for Texture<F, D, MS>
where
    F: TextureFormat + Sampled,
    D: TextureDimension<F>,
{
    type Dimension = D;
    type SampleType = F::SampleType;

    fn raw_texture(&self) -> Id<RawTexture> {
        self.sync();

        self.raw_texture()
    }

    fn write(&mut self) {
        unsafe { self.mark_gpu() };
    }
}

pub trait StoredTexture {
    type Dimension;
    type TexelFormat;

    fn raw_texture(&self) -> Id<RawTexture>;

    fn write(&mut self);
}

impl<F, D, const MS: bool> StoredTexture for Texture<F, D, MS>
where
    F: TextureFormat + Stored,
    D: TextureDimension<F>,
{
    type Dimension = D;
    type TexelFormat = F::TexelFormat;

    fn raw_texture(&self) -> Id<RawTexture> {
        self.sync();

        self.raw_texture()
    }

    fn write(&mut self) {
        unsafe { self.mark_gpu() };
    }
}
