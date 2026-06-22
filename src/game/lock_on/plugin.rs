use bevy::prelude::*;
use super::components::LockOnState;
use super::systems::{toggle_lock_on, validate_lock_on, lock_on_camera_focus};

pub struct LockOnPlugin;

impl Plugin for LockOnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LockOnState>()
           .add_systems(Update, (
               toggle_lock_on, 
               validate_lock_on,
               // Kamera odaklanması hareketten hemen sonra yapılmalı
               lock_on_camera_focus 
           ).chain());
    }
}
