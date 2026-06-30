use bevy::prelude::*;

#[derive(Event)]
pub struct WeaponSwapRequestEvent {
    pub entity: Entity, // Silahı değiştiren varlık (Örn: Player)
}
