use super::components::StaggerEvent;
use super::systems::{
    attack::{handle_attacks, update_attack_intent},
    state::process_attack_states,
    collision::hit_detection,
    lifecycle::{cleanup_hitboxes, handle_recoil_events, spawn_hitbox_on_attack},
    animation::animate_weapon,
};
use crate::game::enemy::systems::{handle_stagger_events, process_stagger};
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StaggerEvent>();
        app.add_systems(
            Update,
            (
                update_attack_intent, // YENİ: Fareden niyet okuma ilk sırada çalışmalı
                handle_attacks,
                process_attack_states,
                hit_detection,
                handle_recoil_events, // YENİ: Hit detection'dan hemen sonra 
                animate_weapon,
                handle_stagger_events,
                process_stagger,
                spawn_hitbox_on_attack,
                cleanup_hitboxes,
            )
                .chain(),
        );
    }
}
