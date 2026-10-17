use bevy::prelude::*;
use crate::game::weapon::components::WeaponType;
use crate::game::shield::data::ShieldType;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PendingEquipment;

#[derive(Component)]
pub struct PendingColliderFit;

#[derive(Component, Default)]
pub struct PlayerInventory {
    // YENİ: Unarmed için Option kullanıyoruz
    pub weapons: Vec<Option<WeaponType>>, 
    pub shields: Vec<Option<ShieldType>>,
}

#[derive(Component, Default)]
pub struct PlayerState {
    pub vertical_velocity: f32,
}

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
            walk_speed: 5.0,
            sprint_speed: 84.0,
            jump_velocity: 15.0,
            gravity: -10.0,
        }
    }
}

#[derive(Resource)]
pub struct PlayerCombatConfig {
    pub guard_sensitivity: f32,
    pub guard_max_offset: f32,
}

impl Default for PlayerCombatConfig {
    fn default() -> Self {
        Self {
            guard_sensitivity: 0.005,
            guard_max_offset: 0.5,
        }
    }
}