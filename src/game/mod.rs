pub mod animation;
pub mod camera;
pub mod combat;
pub mod common;
pub mod debug_hud;
pub mod dodge;
pub mod enemy;
pub mod health;
pub mod ik;
pub mod lock_on;
pub mod placement; // YENİ EKLENDİ
pub mod player;
pub mod poise;
pub mod shield;
pub mod stamina;
pub mod weapon;
pub mod world; 
//pub mod ui;
pub mod objects;
pub mod encounter;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(), 

            placement::plugin::PlacementPlugin, 
            objects::plugin::ObjectsPlugin, // YENİ PLUGİN
            world::plugin::WorldPlugin, 
            camera::plugin::CameraPlugin,
            player::plugin::PlayerPlugin,
            stamina::plugin::StaminaPlugin,
            dodge::plugin::DodgePlugin,
            health::plugin::HealthPlugin,
            enemy::plugin::EnemyPlugin,
            combat::plugin::CombatPlugin,
            lock_on::plugin::LockOnPlugin,
            weapon::plugin::WeaponPlugin,
            debug_hud::plugin::DebugHudPlugin,
            //ui::plugin::UiPlugin,
        ))
        .add_plugins((
            //RapierDebugRenderPlugin::default(), // <--- BUNU EKLE
            shield::plugin::ShieldPlugin,
            animation::plugin::AnimationPlugin,
            ik::plugin::IkPlugin,
            poise::plugin::PoisePlugin,
            encounter::plugin::EncounterPlugin,
        ));
    }
}
