use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use crate::game::weapon::components::{Weapon, AttackDefinition, AttackType}; 


#[derive(Component)]
pub struct Hurtbox {
    pub owner: Entity, // Hasarın uygulanacağı ana entity (Health buradadır)
}

#[derive(Event)]
pub struct StaggerEvent {
    pub target: Entity,
    pub force: f32, // İleride Poise ile karşılaştırılacak
    pub duration: f32,
}


//attacktype vardı lan burda

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

//attackdef vardı 
//weapon vardı
#[derive(Component, Default , PartialEq , Eq)]
pub enum CombatState {
    #[default]
    Idle,
    // Artık her durum, hangi saldırının yapıldığını biliyor
    Windup { timer: Timer, attack_type: AttackType },
    Active { timer: Timer, hitbox_entity: Option<Entity>, attack_type: AttackType },
    Recovery { timer: Timer, attack_type: AttackType },
    Recoiled { timer: Timer },
}

#[derive(Component)]
pub struct Hitbox {
    pub damage: f32,
    pub owner: Entity,
    pub hit_entities: HashSet<Entity>,
    pub lifesteal: f32,
}

#[derive(Component)]
pub struct EquippedWeapon;
