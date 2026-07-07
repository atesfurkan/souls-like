use bevy::prelude::*;

/// Karakterin belirli bir uzvu için ulaşılması hedeflenen noktayı temsil eder.
/// Bu bileşen genellikle oyundaki mantık (Combat/Shield) tarafından güncellenir.
#[derive(Component, Debug, Reflect)]
pub struct IkTarget {
    pub position: Vec3, // Hedefin dünya veya lokal koordinattaki pozisyonu
    /// 0.0 (Tamamen FK/Animasyon) ile 1.0 (Tamamen IK) arası geçiş değeri.
    /// Smooth interpolation (slerp) için kullanılır.
    pub weight: f32, 
}

/// 2-Bone IK algoritmasının ihtiyaç duyduğu kemik zinciri.
/// (Örn: UpperArm -> Forearm -> Hand)
#[derive(Component, Debug, Reflect)]
pub struct TwoBoneIkChain {
    pub root: Entity, // Kök kemik (Örn: Omuz / LeftArm)
    pub mid: Entity,  // Orta kemik (Örn: Dirsek / LeftForearm)
    pub tip: Entity,  // Uç kemik (Örn: El / LeftHand)
    pub pole_vector: Vec3, // Dirseğin veya dizin bükülme yönünü belirleyen vektör
}
