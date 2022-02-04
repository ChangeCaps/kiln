use crate::VertexFormat;

pub trait AsVertexAttribute {
    const FORMAT: VertexFormat;
}

macro_rules! impl_vert_attr {
    ($ty:path = $format:expr) => {
        impl AsVertexAttribute for $ty {
            const FORMAT: VertexFormat = $format;
        }
    };
}

impl_vert_attr!(i32 = VertexFormat::Sint32);
impl_vert_attr!(u32 = VertexFormat::Uint32);
impl_vert_attr!(f32 = VertexFormat::Float32);
