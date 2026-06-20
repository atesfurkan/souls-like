pub mod camera;
pub mod common;
pub mod player;
pub mod world;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            world::plugin::WorldPlugin,
            camera::plugin::CameraPlugin,
            player::plugin::PlayerPlugin,
        ));
    }
}
