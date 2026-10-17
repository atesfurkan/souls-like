use bevy::prelude::*;

use super::events::{BlockEvent, RecoilEvent, ShieldSwapRequestEvent};
use super::systems::{handle_shield_input, process_shield_swap, handle_unarmed_block_box};

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecoilEvent>()
            .add_event::<BlockEvent>()
            .add_event::<ShieldSwapRequestEvent>()
            .add_systems(
                Update,
                (
                    handle_shield_input,
                    process_shield_swap,
                    handle_unarmed_block_box, // Unarmed guard collision/hitbox sistemi eklendi
                )
                    .chain(),
            );
    }
}