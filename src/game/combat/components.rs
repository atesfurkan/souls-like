use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};


#[derive(Event)]
pub struct StaggerEvent {
    pub target: Entity,
    pub force: f32, // İleride Poise ile karşılaştırılacak
    pub duration: f32,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackType {
    Thrust,
    Overhead,
    LeftSwing,
    RightSwing,
}

// O anki saldırı niyetini tutan bileşen
#[derive(Component)]
pub struct AttackIntent {
    pub current: AttackType,
}

impl Default for AttackIntent {
    fn default() -> Self {
        Self { current: AttackType::Thrust }
    }
}

// Tek bir saldırının tüm özellikleri
#[derive(Clone)]
pub struct AttackDefinition {
    pub windup_duration: f32,
    pub active_duration: f32,
    pub recovery_duration: f32,
    pub stamina_cost: f32,
    pub damage: f32,
    pub hitbox_radius: f32,
    pub hitbox_offset: Vec3,
}

// Silah artık bir saldırı sözlüğü tutuyor
#[derive(Component)]
pub struct Weapon {
    pub attacks: HashMap<AttackType, AttackDefinition>,
}

#[derive(Component, Default)]
pub enum CombatState {
    #[default]
    Idle,
    // Artık her durum, hangi saldırının yapıldığını biliyor
    Windup { timer: Timer, attack_type: AttackType },
    Active { timer: Timer, hitbox_entity: Option<Entity>, attack_type: AttackType },
    Recovery { timer: Timer, attack_type: AttackType },
}

#[derive(Component)]
pub struct Hitbox {
    pub damage: f32,
    pub owner: Entity,
    pub hit_entities: HashSet<Entity>,
}

#[derive(Component)]
pub struct EquippedWeapon;
