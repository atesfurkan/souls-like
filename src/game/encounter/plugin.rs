use bevy::prelude::*;
use super::components::ZoneEncounter;
use super::definition::EncounterConfig;
use super::systems::{encounter_planner_system, debug_encounter_state_system};

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ZoneEncounter>();
        app.init_resource::<EncounterConfig>();
        
        // YENİ: Race condition'ı önlemek için planner sistemini analizden önceye (PreUpdate) alıyoruz
        app.add_systems(PreUpdate, encounter_planner_system);
        app.add_systems(Update, debug_encounter_state_system);
    }
}