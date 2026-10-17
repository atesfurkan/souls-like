pub mod gizmos;

use bevy::prelude::*;
use gizmos::{draw_reference_grid, draw_layout_graph,
    };

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_reference_grid, draw_layout_graph));
    }
}