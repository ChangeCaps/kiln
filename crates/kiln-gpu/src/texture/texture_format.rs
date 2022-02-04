use crate::{color::*, RawTextureFormat};

pub unsafe trait TextureData: Send + Sync + Copy {}

pub trait TextureFormat: Send + Sync {
    type Data: TextureData;

    fn format(&self) -> RawTextureFormat;
}

pub trait Sampled {
    type SampleType;
}

pub trait Stored {
    type TexelFormat;
}

pub struct Float;
pub struct Depth;
pub struct Sint;
pub struct Uint;

macro_rules! texture_format {
	($name:ident, $sample_type:ident $(<$filterable:literal>)?, $data:path $(, $texel_format:ident)?) => {
		#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct $name;

		impl Sampled for $name {
			type SampleType = $sample_type$(<$filterable>)?;
		}

		$(
			impl Stored for $name {
				type TexelFormat = $texel_format;
			}
		)?

		impl super::TextureFormat for $name {
			type Data = $data;

			fn format(&self) -> wgpu::TextureFormat {
				wgpu::TextureFormat::$name
			}
		}
	};
}

texture_format!(Rgba8UnormSrgb, Float, Rgba8U);
texture_format!(Rgba8Unorm, Float, Rgba8U, Rgba8Unorm);
texture_format!(Rgba8Snorm, Float, Rgba8I, Rgba8Snorm);
texture_format!(Rgba8Uint, Uint, Rgba8U, Rgba8Uint);
texture_format!(Rgba8Sint, Sint, Rgba8I, Rgba8Sint);
texture_format!(Rgba16Uint, Uint, Rgba16U, Rgba16Uint);
texture_format!(Rgba16Sint, Sint, Rgba16I, Rgba16Sint);
texture_format!(Rgba16Float, Float, Rgba16U, Rgba16Float);
texture_format!(R32Uint, Uint, R32U, R32Uint);
texture_format!(R32Sint, Sint, R32I, R32Sint);
texture_format!(R32Float, Float, R32, R32Float);
texture_format!(Rg32Uint, Uint, Rg32U, Rg32Uint);
texture_format!(Rg32Sint, Sint, Rg32I, Rg32Sint);
texture_format!(Rg32Float, Float, Rg32, Rg32Float);
texture_format!(Rgba32Uint, Uint, Rgba32U, Rgba32Uint);
texture_format!(Rgba32Sint, Sint, Rgba32I, Rgba32Sint);
texture_format!(Rgba32Float, Float, Rgba32, Rgba32Float);
