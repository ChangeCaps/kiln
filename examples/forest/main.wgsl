#include <kiln/input>
#include <kiln/camera>
#include <kiln/ray>
#include <kiln/post>

#include "noise.wgsl"

fn map_terrain(p: vec3<f32>) -> f32 {
	var e = fbm2(p.xz / 2000.0 + vec2<f32>(1.0, -2.0));
	e = 600.0 * e + 600.0;

	e += 90.0 * smoothstep(552.0, 594.0, e);

	return e;
}

fn map(p: vec3<f32>) -> f32 {
	let terrain = map_terrain(p);
	return (p.y - terrain) * 0.6;
}

fn intersect(ray: Ray) -> RayHit {
	var hit = new_hit();

	for (var i = 0; i < 256; i += 1) {
		let p = ray_end(ray, hit.len);
		let d = map(p);

		if d < 0.01 {
			hit.hit = true;
			break;
		}

		hit.len += d;

		if hit.len > 5000.0 {
			hit.hit = false;
			break;
		}
	}

	return hit;
}

fn normal(p: vec3<f32>) -> vec3<f32> {
    let e = vec2<f32>(0.1, 0.0);
	let d = map(p);

	let n = d - vec3<f32>(
		map(p - e.xyy),
		map(p - e.yxy),
		map(p - e.yyx),
	);

    return normalize(n);
}

@fragment
fn frag(input: Input) -> @location(0) vec4<f32> {
	let ray = camera_ray(input); 
	let hit = intersect(ray);

	var color = vec3<f32>(1.0);

	if hit.hit {
		var light = vec3<f32>(0.0);

		let normal = normal(ray_end(ray, hit.len));

		let sun_dir = normalize(vec3<f32>(1.0, 1.0, 0.2));

		let sun_diffuse = max(0.0, dot(normal, sun_dir));

		light += vec3<f32>(5.0, 3.0, 2.0) * sun_diffuse;

		color *= light;
	} else {
		color = vec3<f32>(0.1, 0.2, 0.3);
	}

	return vec4<f32>(tonemap_aces(color), 1.0);
}
