use bevy::prelude::*;
use super::systems::{spawn_debug_hud, update_debug_hud};

pub struct DebugHudPlugin;

impl Plugin for DebugHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_hud)
           .add_systems(Update, update_debug_hud);
    }
}
