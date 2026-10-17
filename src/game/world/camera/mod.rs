pub mod freecam;

use bevy::prelude::*;
use freecam::{camera_look, camera_movement, setup_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
           .add_systems(Update, (camera_movement, camera_look));
    }
}
