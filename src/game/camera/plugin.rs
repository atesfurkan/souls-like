use bevy::prelude::*;
use super::systems::{camera_controls, cursor_management, spawn_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
           .add_systems(Update, (camera_controls, cursor_management));
    }
}
