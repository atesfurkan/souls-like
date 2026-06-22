pub mod camera;
pub mod common;
pub mod player;
pub mod stamina;
pub mod world;
pub mod dodge;
pub mod enemy;
pub mod health;
pub mod combat;
pub mod lock_on;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*; 

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(), // FİZİK MOTORU
            world::plugin::WorldPlugin,
            camera::plugin::CameraPlugin,
            player::plugin::PlayerPlugin,
            stamina::plugin::StaminaPlugin,
            dodge::plugin::DodgePlugin,
            health::plugin::HealthPlugin,
            enemy::plugin::EnemyPlugin,
            combat::plugin::CombatPlugin,
            lock_on::plugin::LockOnPlugin,
            
        ));
    }
}
