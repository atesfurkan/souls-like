use bevy::prelude::*;
use bevy::utils::HashMap;

// --- YENİ EKLENEN KEMİK SABİTLERİ ---

#[derive(Resource)]
pub struct PlayerMovementConfig {
    pub walk_speed: f32,
    pub sprint_speed: f32,
    pub jump_velocity: f32,
    pub gravity: f32,
}

impl Default for PlayerMovementConfig {
    fn default() -> Self {
        Self {
            walk_speed: 4.0,
            sprint_speed: 8.0,
            jump_velocity: 15.0,
            gravity: -15.0,
        }
    }
}

#[derive(Component, Default)]
pub struct PlayerState {
    pub vertical_velocity: f32,
}

#[derive(Component)]
pub struct Player;




#[derive(Component)]
pub struct PendingEquipment; // Silah ve kalkanın kemiklere bağlanmayı beklediğini belirtir