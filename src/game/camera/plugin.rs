use bevy::prelude::*;
use super::systems::{spawn_camera, camera_controls, cursor_management};
//use crate::game::ui::equipment_menu::components::MenuState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
           .add_systems(Update, (
               camera_controls,
               cursor_management,
           ));
        // Menü açıkken kamera ve fare yönetimi durur!
    }
}