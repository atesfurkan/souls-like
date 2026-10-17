use bevy::prelude::*;
use super::spawn::{spawn_objects, handle_object_despawn};
use super::components::{Chest, Stalactite, Tree};

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Chest>()
           .register_type::<Stalactite>()
           .register_type::<Tree>()
           .add_systems(Update, (spawn_objects, handle_object_despawn));
    }
}