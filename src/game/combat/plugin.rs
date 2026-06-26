use bevy::prelude::*;
use super::systems::{  handle_attacks, process_attack_states, hit_detection, animate_weapon, update_attack_intent};
use super::components::StaggerEvent;
use crate::game::enemy::systems::handle_stagger_events ;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StaggerEvent>();
        app.add_systems(Update, (
            update_attack_intent, // YENİ: Fareden niyet okuma ilk sırada çalışmalı
            handle_attacks, 
            process_attack_states,
            hit_detection,
            animate_weapon,
            handle_stagger_events, 
            
        ).chain());
    }
}
