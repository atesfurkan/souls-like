use bevy::prelude::*;

#[derive(Event)]
pub struct ApplyPoiseDamageEvent {
    pub target: Entity, // Hasarı alan entity
    pub amount: f32,    // Alınan poise hasarı miktarı
}

#[derive(Event)]
pub struct PoiseBrokenEvent {
    pub target: Entity, // Poise değeri sıfırlanan entity
}
