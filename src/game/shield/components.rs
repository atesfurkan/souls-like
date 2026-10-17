use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum DefaultShieldShape {
    Box { width: f32, height: f32, depth: f32 },
    Cylinder { radius: f32, height: f32 },
}

#[derive(Component, Clone)]
pub enum ShieldVisualData {
    ProgrammerArt {
        shape: DefaultShieldShape,
        color: Color,
    },
}

#[derive(Component)]
pub struct ShieldVisualAnchor;

#[derive(Component)]
pub struct Shield {
    pub name: String,
    pub block_rate: f32,
    pub stamina_cost_multiplier: f32,
    pub parry_window: f32,
}

#[derive(Component)]
pub struct ShieldBox {
    pub owner: Entity,
}

// YENİ: Unarmed block için geçici olarak yaratılacak hitbox'ı bulmamızı sağlar
#[derive(Component)]
pub struct UnarmedShieldBox;

#[derive(Component)]
pub struct EquippedShield;

#[derive(Component, Default)]
pub enum BlockingState {
    #[default]
    Idle,
    Parrying { timer: Timer },
    Blocking,
    GuardBroken { timer: Timer },
}