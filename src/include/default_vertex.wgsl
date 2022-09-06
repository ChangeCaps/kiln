struct Uniforms {
	view: mat4x4<f32>,
	aspect: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) uv: vec2<f32>,
	@location(1) coord: vec2<f32>,
};

@vertex
fn vert(@builtin(vertex_index) index: u32) -> VertexOutput {
	var verts = array<vec2<f32>, 6>(
		vec2<f32>(-1.0, -1.0),
		vec2<f32>(1.0, -1.0),
		vec2<f32>(-1.0, 1.0),
		vec2<f32>(-1.0, 1.0),
		vec2<f32>(1.0, -1.0),
		vec2<f32>(1.0, 1.0),
	);

	var out: VertexOutput;

	out.position = vec4<f32>(verts[index], 0.0, 1.0);
	out.uv = out.position.xy * vec2(0.5, -0.5) + vec2(0.5, 0.5);
	out.coord = out.position.xy;

	return out;
}
