use std::{hash::Hash, ops::Deref, sync::Arc};

use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};

use crate::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, ComputePipelineDescriptor, Id, IdSource,
    PipelineLayoutDescriptor, RawBuffer, RawTexture, RenderPipelineDescriptor,
    ShaderModuleDescriptor,
};

pub type IdRef<'a, T> = Ref<'a, Id<T>, T>;
pub type IdRefMut<'a, T> = RefMut<'a, Id<T>, T>;

pub trait GpuResourceDescriptor: Eq + Hash {
    type Resource;
}

pub struct GpuResourceDescriptorMap<T: GpuResourceDescriptor> {
    id_source: IdSource<T::Resource>,
    map: DashMap<Id<T::Resource>, T::Resource>,
    descriptors: DashMap<T, Id<T::Resource>>,
}

impl<T> Default for GpuResourceDescriptorMap<T>
where
    T: GpuResourceDescriptor,
{
    fn default() -> Self {
        Self {
            id_source: Default::default(),
            map: Default::default(),
            descriptors: Default::default(),
        }
    }
}

impl<T> GpuResourceDescriptorMap<T>
where
    T: GpuResourceDescriptor,
{
    pub fn generate(&self) -> Id<T::Resource> {
        self.id_source.generate()
    }

    pub fn insert(&self, id: Id<T::Resource>, resource: T::Resource) -> Option<T::Resource> {
        self.map.insert(id, resource)
    }

    pub fn push(&self, resource: T::Resource) -> Id<T::Resource> {
        let id = self.generate();
        self.map.insert(id, resource);
        id
    }

    pub fn insert_descriptor(&self, desc: T, id: Id<T::Resource>) {
        self.descriptors.insert(desc, id);
    }

    pub fn push_descriptor(&self, desc: T, resource: T::Resource) -> Id<T::Resource> {
        let id = self.push(resource);
        self.insert_descriptor(desc, id);
        id
    }

    pub fn get(&self, desc: &T) -> Option<IdRef<'_, T::Resource>> {
        let id = self.descriptors.get(desc)?;
        self.map.get(&id)
    }

    pub fn get_mut(&self, desc: &T) -> Option<IdRefMut<'_, T::Resource>> {
        let id = self.descriptors.get(desc)?;
        self.map.get_mut(&id)
    }

    pub fn get_id(&self, id: &Id<T::Resource>) -> Option<IdRef<'_, T::Resource>> {
        self.map.get(id)
    }

    pub fn get_mut_id(&self, id: &Id<T::Resource>) -> Option<IdRefMut<'_, T::Resource>> {
        self.map.get_mut(id)
    }

    pub fn get_desc(&self, desc: &T) -> Option<Id<T::Resource>> {
        self.descriptors.get(desc).map(|id| id.clone())
    }
}

pub struct GpuResourceMap<T> {
    id_source: IdSource<T>,
    map: DashMap<Id<T>, T>,
}

impl<T> Default for GpuResourceMap<T> {
    fn default() -> Self {
        Self {
            id_source: Default::default(),
            map: Default::default(),
        }
    }
}

impl<T> GpuResourceMap<T> {
    pub fn generate(&self) -> Id<T> {
        self.id_source.generate()
    }

    pub fn insert(&self, id: Id<T>, resource: T) -> Option<T> {
        self.map.insert(id, resource)
    }

    pub fn push(&self, resource: T) -> Id<T> {
        let id = self.generate();
        self.insert(id, resource);
        id
    }

    pub fn contains(&self, id: &Id<T>) -> bool {
        self.map.contains_key(id)
    }

    pub fn get(&self, id: &Id<T>) -> Option<IdRef<'_, T>> {
        self.map.get(id)
    }

    pub fn get_mut(&self, id: &Id<T>) -> Option<IdRefMut<'_, T>> {
        self.map.get_mut(id)
    }

    pub fn remove(&self, id: &Id<T>) {
        self.map.remove(id);
    }
}

#[derive(Default)]
pub struct GpuResourcesInner {
    pub buffers: GpuResourceMap<RawBuffer>,
    pub textures: GpuResourceMap<RawTexture>,
    pub bind_group_layouts: GpuResourceDescriptorMap<BindGroupLayoutDescriptor>,
    pub bind_groups: GpuResourceDescriptorMap<BindGroupDescriptor>,
    pub pipeline_layouts: GpuResourceDescriptorMap<PipelineLayoutDescriptor>,
    pub shader_modules: GpuResourceDescriptorMap<ShaderModuleDescriptor>,
    pub render_pipelines: GpuResourceDescriptorMap<RenderPipelineDescriptor>,
    pub compute_pipelines: GpuResourceDescriptorMap<ComputePipelineDescriptor>,
}

/// Clonable storage for gpu resources.
#[derive(Clone, Default)]
pub struct GpuResources {
    inner: Arc<GpuResourcesInner>,
}

impl Deref for GpuResources {
    type Target = GpuResourcesInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
