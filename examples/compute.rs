pub use kiln::*;

#[repr(C)]
#[derive(Vertex)]
struct Vert {
    #[vertex(location = 0)]
    x: i32,
}

wgsl! {
    struct Uniforms {
        x: f32;
    };

    [[group(0), binding(0)]]
    var<storage, read_write> uniforms: Uniforms;

    [[stage(compute), workgroup_size(1)]]
    fn comp() {
        uniforms.x = 2.0;
    }
}

fn main() {
    let instance = GpuInstance::new_headless();

    let mut uniforms = UniformBuffer::new(&instance, Uniforms::new(0.0));

    comp {
        uniforms: &mut uniforms,
    }
    .pass(&instance);
}
