use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity, // Hasarı kim alıyor?
    pub amount: f32,    // Ne kadar hasar alıyor?
}
