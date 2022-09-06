fn hash2(p: vec2<f32>) -> f32 {
	let p = 50.0 * fract(p * 0.3183099);
	return fract(p.x * p.y * (p.x + p.y));
}

fn noised2(x: vec2<f32>) -> vec3<f32> {
	let p = floor(x);
	let w = fract(x);
	
	let u = w * w * w * (w * (w * 6.0 - 15.0) + 10.0);
	let du = 30.0 * w * w * (w * (w - 2.0) + 1.0);

	let a = hash2(p + vec2<f32>(0.0, 0.0));
	let b = hash2(p + vec2<f32>(1.0, 0.0));
	let c = hash2(p + vec2<f32>(0.0, 1.0));
	let d = hash2(p + vec2<f32>(1.0, 1.0));

	let k0 = a;
	let k1 = b - a;
	let k2 = c - a;
	let k4 = a - b - c + d;

	let h = k0 + k1 * u.x + k2 * u.y + k4 * u.x * u.y;
	let d = vec2<f32>(k1 + k4 * u.y, k2 + k4 * u.x);

	return vec3<f32>(h * 2.0 - 1.0, d * du * 2.0);
}

fn noise2(x: vec2<f32>) -> f32 {
	let p = floor(x);
	let w = fract(x);
	
	let u = w * w * w * (w * (w * 6.0 - 15.0) + 10.0);

	let a = hash2(p + vec2<f32>(0.0, 0.0));
	let b = hash2(p + vec2<f32>(1.0, 0.0));
	let c = hash2(p + vec2<f32>(0.0, 1.0));
	let d = hash2(p + vec2<f32>(1.0, 1.0));

	let k0 = a;
	let k1 = b - a;
	let k2 = c - a;
	let k4 = a - b - c + d;

	let h = k0 + k1 * u.x + k2 * u.y + k4 * u.x * u.y;

	return 	h * 2.0 - 1.0;
}

fn fbm2(x: vec2<f32>) -> f32 {
	let r = mat2x2<f32>(0.8, 0.6, -0.6, 0.8);

	var x = x;

	let f = 1.9;
	let s = 0.55;
	var a = 0.0;
	var b = 0.5;

	for (var i = 0; i < 9; i += 1) {
		let n = noise2(x);
		a += b * n;
		b *= s;
		x = f * r * x;
	}

	return a;
}

