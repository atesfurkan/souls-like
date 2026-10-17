use bevy::prelude::*;
use super::data::ShieldType;

#[derive(Event)]
pub struct ShieldSwapRequestEvent {
    pub entity: Entity,
    pub target_shield: Option<ShieldType>, // YENİ: Option eklendi
}

#[derive(Event)]
pub struct RecoilEvent {
    pub target: Entity,
}

#[derive(Event)]
pub struct BlockEvent {
    pub defender: Entity,
    pub attacker: Entity,
    pub is_parry: bool,
}