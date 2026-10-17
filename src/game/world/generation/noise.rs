/// 3D koordinatları karma (hash) işleminden geçirerek 0.0 ile 1.0 arası deterministik rastgele sayı üretir
fn hash3(x: i32, y: i32, z: i32, seed: i32) -> f32 {
    let mut h = seed.wrapping_add(x.wrapping_mul(374761393));
    h = h.wrapping_add(y.wrapping_mul(668265263));
    h = h.wrapping_add(z.wrapping_mul(786358481));
    h ^= h >> 13;
    h = h.wrapping_mul(236357905);
    h ^= h >> 15;
    
    (h as u32 as f32) / (u32::MAX as f32)
}

/// Lineer interpolasyon
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// Kenar yumaşatıcı interpolasyon (Smootherstep)
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// 3 Boyutlu temel gürültü fonksiyonu
pub fn value_noise_3d(x: f32, y: f32, z: f32, seed: i32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let z0 = z.floor() as i32;

    let x1 = x0.wrapping_add(1);
    let y1 = y0.wrapping_add(1);
    let z1 = z0.wrapping_add(1);

    let u = fade(x - x.floor());
    let v = fade(y - y.floor());
    let w = fade(z - z.floor());

    let c000 = hash3(x0, y0, z0, seed);
    let c100 = hash3(x1, y0, z0, seed);
    let c010 = hash3(x0, y1, z0, seed);
    let c110 = hash3(x1, y1, z0, seed);
    let c001 = hash3(x0, y0, z1, seed);
    let c101 = hash3(x1, y0, z1, seed);
    let c011 = hash3(x0, y1, z1, seed);
    let c111 = hash3(x1, y1, z1, seed);

    let nx00 = lerp(c000, c100, u);
    let nx01 = lerp(c001, c101, u);
    let nx10 = lerp(c010, c110, u);
    let nx11 = lerp(c011, c111, u);

    let nxy0 = lerp(nx00, nx10, v);
    let nxy1 = lerp(nx01, nx11, v);

    lerp(nxy0, nxy1, w)
}

/// Fractal Brownian Motion (FBM) ile katmanlı organik gürültü
pub fn fbm_3d(x: f32, y: f32, z: f32, octaves: usize, seed: i32) -> f32 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        total += value_noise_3d(x * frequency, y * frequency, z * frequency, seed) * amplitude;
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    total / max_value
}
