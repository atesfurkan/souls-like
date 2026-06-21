use bevy::prelude::*;
use super::systems::{player_movement, spawn_player};

use super::components::{Player, PlayerState, PlayerMovementConfig};



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMovementConfig>() // Config sisteme kaydedildi
           .add_systems(Startup, spawn_player)
           .add_systems(Update, player_movement);
    }
}
