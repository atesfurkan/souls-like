use bevy::prelude::*;

/// YENİ: Noktayı belirli bir oranda sıkıştırarak (Scale) Basık Küre/Elipsoid oluşturur
pub fn sdf_ellipsoid(p: Vec3, center: Vec3, radius: f32, scale: Vec3) -> f32 {
    // Uzayı scale ile çarparak büküyoruz. (Örn: Y eksenini 2.0 ile çarparsanız, şekil Y'de basıklaşır)
    let local_p = (p - center) * scale;
    local_p.length() - radius
}

// ODALAR İÇİN (Sizin kodunuzda adı muhtemelen sdf_boxy olarak kayıtlı)
pub fn sdf_boxy(p: Vec3, center: Vec3, radius: f32, scale: Vec3) -> f32 {
    let local_p = (p - center) * scale;
    local_p.abs().max_element() - radius
}

// KORİDORLAR İÇİN (Bu eksik kalmış veya yanlış çağrılmış olabilir)
pub fn sdf_boxy_capsule(p: Vec3, a: Vec3, b: Vec3, r: f32) -> f32 {
    let pa = p - a;
    let ba = b - a;
    
    let h = (pa.dot(ba) / ba.dot(ba)).clamp(0.0, 1.0);
    let closest_point = a + ba * h;
    let d = p - closest_point;
    
    d.abs().max_element() - r
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

pub fn smooth_min(a: f32, b: f32, k: f32) -> f32 {
    let h = (0.5 + 0.5 * (b - a) / k).clamp(0.0, 1.0);
    lerp(b, a, h) - k * h * (1.0 - h)
}

pub fn sdf_sphere(point: Vec3, center: Vec3, radius: f32) -> f32 {
    point.distance(center) - radius
}

/// YENİ: İki nokta arasında uzanan Kapsül (Tünel) için SDF
/// p: Uzaydaki geçerli nokta
/// a: Tünelin başlangıç noktası
/// b: Tünelin bitiş noktası
/// r: Tünelin yarıçapı
pub fn sdf_capsule(p: Vec3, a: Vec3, b: Vec3, r: f32) -> f32 {
    let pa = p - a;
    let ba = b - a;
    
    // Noktanın doğru parçası (A-B) üzerindeki izdüşüm oranını (0 ile 1 arası) buluyoruz
    let h = (pa.dot(ba) / ba.dot(ba)).clamp(0.0, 1.0);
    
    // İzdüşüm noktasına olan mesafeden yarıçapı çıkarıyoruz
    (pa - ba * h).length() - r
}