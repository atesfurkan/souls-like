use bevy::prelude::*;
use crate::game::poise::events::{ApplyPoiseDamageEvent, PoiseBrokenEvent};
use crate::game::poise::systems::handle_poise_damage;

pub struct PoisePlugin;

impl Plugin for PoisePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyPoiseDamageEvent>()
           .add_event::<PoiseBrokenEvent>()
           .add_systems(Update, handle_poise_damage);
    }
}
