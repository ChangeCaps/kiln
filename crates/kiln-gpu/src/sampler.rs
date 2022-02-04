use std::num::NonZeroU8;

use ordered_float::OrderedFloat;

use crate::{
    AddressMode, CompareFunction, FilterMode, Id, RawSampler, RawSamplerDescriptor,
    SamplerBorderColor,
};

pub type SamplerId = Id<RawSampler>;

#[derive(Clone, Debug)]
pub struct Sampler {
    pub label: Option<String>,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub anisotropy_clamp: Option<NonZeroU8>,
    pub border_color: Option<SamplerBorderColor>,
}

impl Default for Sampler {
    fn default() -> Self {
        Self {
            label: None,
            address_mode_u: AddressMode::default(),
            address_mode_v: AddressMode::default(),
            address_mode_w: AddressMode::default(),
            mag_filter: FilterMode::default(),
            min_filter: FilterMode::default(),
            mipmap_filter: FilterMode::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 1.0,
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}

impl Sampler {}

#[derive(Clone, Debug)]
pub struct SamplerComparison {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SamplerDescriptor {
    pub label: Option<String>,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: OrderedFloat<f32>,
    pub lod_max_clamp: OrderedFloat<f32>,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: Option<NonZeroU8>,
    pub border_color: Option<SamplerBorderColor>,
}

impl SamplerDescriptor {
    pub fn as_raw_desc<'a>(&'a self) -> RawSamplerDescriptor<'a> {
        RawSamplerDescriptor {
            label: self.label.as_ref().map(AsRef::as_ref),
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: self.address_mode_w,
            mag_filter: self.mag_filter,
            min_filter: self.min_filter,
            mipmap_filter: self.mipmap_filter,
            lod_min_clamp: self.lod_min_clamp.into(),
            lod_max_clamp: self.lod_max_clamp.into(),
            compare: self.compare,
            anisotropy_clamp: self.anisotropy_clamp,
            border_color: self.border_color,
        }
    }
}
