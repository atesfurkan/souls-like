use bevy::prelude::*;
use super::components::WeaponType;

#[derive(Event)]
pub struct WeaponSwapRequestEvent {
    pub entity: Entity, 
    pub target_weapon: Option<WeaponType>, // YENİ: Option ile destekleniyor
}