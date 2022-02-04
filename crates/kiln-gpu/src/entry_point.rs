use crate::{
    BindGroupDescriptor, BindGroupId, BindGroupLayoutDescriptor, BindGroupLayoutId,
    ComputePassCommands, ComputePipelineDescriptor, ComputePipelineId, GpuDevice, GpuInstance,
    GpuResources, PipelineLayoutDescriptor, PipelineLayoutId, ShaderModuleDescriptor,
};

pub trait EntryPoint {
    fn source(&self) -> &'static str;
    fn entry_point(&self) -> &'static str;

    fn bind_group_layout_descriptors(&self) -> Vec<BindGroupLayoutDescriptor>;
    fn bind_group_descriptors(
        &self,
        bind_group_layouts: &[BindGroupLayoutId],
    ) -> Vec<BindGroupDescriptor>;

    fn bind_group_layouts(&self, instance: &GpuInstance) -> Vec<BindGroupLayoutId> {
        self.bind_group_layout_descriptors()
            .into_iter()
            .map(|desc| instance.create_bind_group_layout(desc))
            .collect()
    }

    fn pipeline_layout(
        &self,
        instance: &GpuInstance,
        bind_group_layouts: Vec<BindGroupLayoutId>,
    ) -> PipelineLayoutId {
        let desc = PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts,
            push_constant_ranges: Vec::new(),
        };

        instance.create_pipeline_layout(desc)
    }

    fn bind_groups(
        &self,
        instance: &GpuInstance,
        bind_group_layouts: &[BindGroupLayoutId],
    ) -> Vec<BindGroupId> {
        self.bind_group_descriptors(bind_group_layouts)
            .into_iter()
            .map(|desc| instance.create_bind_group(desc))
            .collect()
    }
}

pub trait RenderEntryPoint: EntryPoint {
    type ColorTargets;
    type DepthTargets;
}

pub struct Dispatch {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub trait ComputeEntryPoint: EntryPoint {
    fn compute_pipeline(
        &self,
        instance: &GpuInstance,
        pipeline_layout: PipelineLayoutId,
    ) -> ComputePipelineId {
        let module = instance.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: self.source().into(),
        });

        instance.create_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(pipeline_layout),
            module,
            entry_point: String::from(self.entry_point()),
        })
    }

    fn pass(self, instance: &GpuInstance) -> ComputePass<Self>
    where
        Self: Sized,
    {
        ComputePass::new(instance, self)
    }
}

pub struct ComputePass<E: ComputeEntryPoint> {
    instance: GpuInstance,
    entry_point: E,
    pipeline: ComputePipelineId,
    commands: ComputePassCommands,
}

impl<E: ComputeEntryPoint> ComputePass<E> {
    pub fn new(instance: &GpuInstance, entry_point: E) -> Self {
        let layouts = entry_point.bind_group_layouts(instance);
        let pipeline_layout = entry_point.pipeline_layout(instance, layouts.clone());
        let pipeline = entry_point.compute_pipeline(instance, pipeline_layout);

        let mut commands = ComputePassCommands::default();

        commands.set_pipeline(pipeline);

        for (index, bind_group) in entry_point
            .bind_groups(instance, &layouts)
            .into_iter()
            .enumerate()
        {
            commands.set_bind_group(index as u32, bind_group, Vec::new());
        }

        Self {
            instance: instance.clone(),
            entry_point,
            pipeline,
            commands,
        }
    }

    pub fn dispatch(&mut self, dispatch: Dispatch) {
        self.commands.dispatch(dispatch.x, dispatch.y, dispatch.z);
    }

    pub fn execute(&self) {
        let ref_commands = self.commands.to_ref_commands(&self.instance);

        let mut encoder = self
            .instance
            .device
            .raw_device()
            .create_command_encoder(&Default::default());

        let mut compute_pass = encoder.begin_compute_pass(&Default::default());

        ref_commands.execute(&mut compute_pass);
    }
}

impl<E: ComputeEntryPoint> Drop for ComputePass<E> {
    fn drop(&mut self) {
        self.execute();
    }
}
