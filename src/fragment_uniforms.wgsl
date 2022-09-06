struct Uniforms {
	view: mat4x4<f32>,
	aspect: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;
