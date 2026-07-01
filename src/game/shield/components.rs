use bevy::prelude::*;



// Kalkanın RPG istatistikleri
#[derive(Component)]
pub struct Shield {
    pub block_rate: f32,                // Engellenecek hasar oranı (örn: %100 için 1.0, %80 için 0.8)
    pub stamina_cost_multiplier: f32,   // Hasar başına gidecek dayanıklılık çarpanı
    pub parry_window: f32,              // Parry penceresi (saniye)
}

// Fiziksel çarpışmayı algılayacak işaretçi
#[derive(Component)]
pub struct ShieldBox {
    pub owner: Entity, // Kalkanın sahibi (Player veya Enemy)
}

// Görsel mesh'lerin ekleneceği child entity işaretçisi
#[derive(Component)]
pub struct EquippedShield;

// Savunma durum makinesi (FSM)
#[derive(Component, Default)]
pub enum BlockingState {
    #[default]
    Idle,
    Parrying { timer: Timer },
    Blocking,
    GuardBroken { timer: Timer },
}


