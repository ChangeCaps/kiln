fn tonemap_aces(color: vec3<f32>) -> vec3<f32> {
	let a = 2.51;
	let b = 0.03;
	let c = 2.43;
	let d = 0.59;
	let e = 0.14;

	return clamp(
		(color * (a * color + b)) / (color * (c * color + d) + e), 
		vec3<f32>(0.0),
		vec3<f32>(1.0),
	);
}
