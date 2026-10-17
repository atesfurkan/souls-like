use super::components::StaggerEvent;
use super::systems::{
    attack::{handle_attacks, update_attack_intent , update_combat_input},
    state::{process_attack_states, trigger_stagger_on_poise_break },
    collision::hit_detection,
    lifecycle::{cleanup_hitboxes, handle_recoil_events, spawn_hitbox_on_attack},
    animation::animate_weapon,
};
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StaggerEvent>();
        app.add_systems(
            Update,
            (
                update_attack_intent, // YENİ: Fareden niyet okuma ilk sırada çalışmalı
                update_combat_input,
                handle_attacks,
                process_attack_states,
                hit_detection,
                handle_recoil_events, // YENİ: Hit detection'dan hemen sonra 
                animate_weapon,
                trigger_stagger_on_poise_break,
                spawn_hitbox_on_attack,
                cleanup_hitboxes,
            )
                .chain(),
        );
    }
}
