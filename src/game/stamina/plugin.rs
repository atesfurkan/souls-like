use bevy::prelude::*;
use super::systems::{spawn_stamina_ui, stamina_regen, update_stamina_ui};

pub struct StaminaPlugin;

impl Plugin for StaminaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stamina_ui)
           .add_systems(Update, (stamina_regen, update_stamina_ui));
    }
}
