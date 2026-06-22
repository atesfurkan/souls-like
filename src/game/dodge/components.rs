use bevy::prelude::*;

#[derive(Resource)]
pub struct DodgeConfig {
    pub speed: f32,
    pub duration: f32,
    pub stamina_cost: f32,
}

impl Default for DodgeConfig {
    fn default() -> Self {
        Self {
            speed: 12.0,       // Yürümeden çok daha hızlı
            duration: 0.3,     // 0.3 saniyelik kısa bir atılma (i-frame aralığı)
            stamina_cost: 20.0,
        }
    }
}

#[derive(Component)]
pub struct DodgeState {
    pub direction: Vec3,
    pub timer: Timer,
}
