#include <kiln/uniforms>
#include <kiln/ray>
#include <kiln/input>

fn camera_ray(input: Input) -> Ray {
	let org = uniforms.view.w;
	let dir = uniforms.view * vec4<f32>(-input.coord.x * uniforms.aspect, input.coord.y, -1.0, 0.0);

	return Ray(org.xyz, normalize(dir.xyz));
}
