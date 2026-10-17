use bevy::prelude::*;

use super::components::{PlayerCombatConfig, PlayerMovementConfig};
use super::spawn::spawn_player;
use super::systems::{
    combat::{handle_shield_swap_input, handle_weapon_swap_input, update_guard_intent},
    equipment::{attach_pending_equipment, attach_unarmed_ik_chains, auto_fit_player_collider},
    movement::player_movement,
};
use crate::game::ik::systems::update_shield_ik_target;
use crate::game::common::systems::setup_anatomical_hurtboxes;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Input,
    Logic,
    Physics,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMovementConfig>()
            .init_resource::<PlayerCombatConfig>()
            .configure_sets(
                Update,
                (
                    PlayerSystemSet::Input,
                    PlayerSystemSet::Logic,
                    PlayerSystemSet::Physics,
                )
                    .chain(),
            )
            .add_systems(PostStartup, spawn_player)
            .add_systems(
                Update,
                (
                    handle_weapon_swap_input,
                    handle_shield_swap_input, 
                    update_guard_intent,
                )
                    .in_set(PlayerSystemSet::Input),
            )
            .add_systems(
                Update,
                (
                    player_movement,
                    update_shield_ik_target,
                    setup_anatomical_hurtboxes,
                )
                    .in_set(PlayerSystemSet::Logic),
            )
            .add_systems(
                Update,
                (
                    attach_pending_equipment,
                    attach_unarmed_ik_chains, 
                    auto_fit_player_collider,
                )
                    .in_set(PlayerSystemSet::Physics),
            );
    }
}