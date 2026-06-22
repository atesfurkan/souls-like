use bevy::prelude::*;
use super::systems::{dodge_input, dodge_movement};
use super::components::DodgeConfig;

pub struct DodgePlugin;

impl Plugin for DodgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DodgeConfig>()
           .add_systems(Update, (dodge_input, dodge_movement));
    }
}
