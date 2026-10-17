pub mod test_scene;

use bevy::prelude::*;
use test_scene::{spawn_test_spheres, update_entity_light_probes, update_sphere_materials};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_test_spheres)
           .add_systems(Update, (update_entity_light_probes, update_sphere_materials));
    }
}