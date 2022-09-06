struct Ray {
	org: vec3<f32>,
	dir: vec3<f32>,
}

fn ray_end(ray: Ray, len: f32) -> vec3<f32> {
	return ray.org + ray.dir * len;
}

struct RayHit {
	hit: bool,
	len: f32,
}

fn new_hit() -> RayHit {
	return RayHit(false, 0.0);
}
