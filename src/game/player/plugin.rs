use bevy::prelude::*;
use super::systems::{player_movement , handle_weapon_swap_input,
    attach_pending_equipment , auto_fit_player_collider };
//use super::spawn::spawn_player;
use super::components::{Player, PlayerState, PlayerMovementConfig};
use crate::game::player::spawn::spawn_player;
use crate::game::player::systems::update_guard_intent;
use crate::game::ik::systems::update_shield_ik_target;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMovementConfig>() // Config sisteme kaydedildi
           .add_systems(Startup, spawn_player)
           .add_systems(Update, (player_movement, handle_weapon_swap_input,
                attach_pending_equipment , update_guard_intent , auto_fit_player_collider))
           .add_systems(Update, (update_guard_intent, update_shield_ik_target));
    }
}

