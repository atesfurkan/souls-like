pub mod components;
pub mod systems;
pub mod spawner;

use bevy::prelude::*;
use self::systems::{extract_region_analysis_system, debug_region_analysis_system, AnalysisTracker};

pub struct AnalysisPlugin;

impl Plugin for AnalysisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnalysisTracker>()
           .add_systems(Update, extract_region_analysis_system)
           .add_systems(Update, debug_region_analysis_system)
           // cave_spawner_system artık chunk_spawner_system oldu
           .add_systems(Update, crate::game::world::analysis::spawner::chunk_spawner_system);
    }
}