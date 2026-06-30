use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity, // Hasarı kim alıyor?
    pub amount: f32,    // Ne kadar hasar alıyor?
}

#[derive(Event)]
pub struct HealEvent {
    pub target: Entity, // Kim iyileşiyor?
    pub amount: f32,    // Ne kadar canı artıyor?
}

#[derive(Event)]
pub struct DeathEvent {
    pub target: Entity, // Hangi entity öldü?
}