use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::{
    PendingColliderFit, Player, PlayerInventory, PlayerState,
};
use crate::game::animation::components::{AnimationIntent, CharacterBones};
use crate::game::combat::components::{
    AttackIntent, CombatInput, CombatState, GuardIntent, LeftHand, RightHand,
};
use crate::game::health::components::Health;
use crate::game::ik::components::IkTarget;
use crate::game::shield::components::BlockingState;
use crate::game::shield::data::ShieldType;
use crate::game::stamina::components::Stamina;
use crate::game::weapon::components::WeaponType;
use crate::game::weapon::data::get_unarmed_config;

use crate::game::common::components::{HurtboxProfile, HUMAN_PROXIES};

#[derive(Component)]
pub struct SpawnPointPlayer;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_point_query: Query<&Transform, With<SpawnPointPlayer>>,
) {
    let player_entity = commands.spawn_empty().id();
    let character_scene = asset_server.load("models/proxyfull.glb#Scene0");

    let spawn_transform = if let Ok(sp_transform) = spawn_point_query.get_single() {
        Transform::from_translation(sp_transform.translation + Vec3::new(0.0, 50.0, 0.0))
    } else {
        Transform::from_xyz(0.0, 50.0, 0.0)
    };

    commands
        .entity(player_entity)
        .insert((
            Player,
            PendingColliderFit,
            SpatialBundle {
                transform: spawn_transform,
                ..default()
            },
            PlayerInventory {
                weapons: vec![None, Some(WeaponType::Spear), Some(WeaponType::VampireBlade)],
                shields: vec![None, Some(ShieldType::Buckler), Some(ShieldType::KiteShield)],
            },
            PlayerState::default(),
            Health::new(100.0),
            Stamina::default(),
            HurtboxProfile { proxies: HUMAN_PROXIES },
        ))
        .insert((
            CombatInput::default(),
            CombatState::default(),
            AttackIntent::default(),
            GuardIntent::default(),
            IkTarget { position: Vec3::ZERO, weight: 0.0 },
            BlockingState::Idle,
        ))
        .insert((
            AnimationIntent::default(),
            CharacterBones::default(),
        ))
        .insert((
            RigidBody::KinematicPositionBased,
            // Daha ince ve uzun bir kapsül (Yarım-uzunluk: 0.65, Yarıçap: 0.25)
            // Kollar ve silahlar animasyon sırasında bu ince kapsülün dışına rahatça taşabilir.
            Collider::capsule_y(0.65, 0.00025),
            KinematicCharacterController::default(),
            get_unarmed_config(),
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: character_scene,
                transform: Transform::from_xyz(0.0, -1.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                ..default()
            });

            parent.spawn((
                RightHand,
                SpatialBundle {
                    transform: Transform::from_xyz(0.4, 1.0, 0.4),
                    ..default()
                },
            ));

            parent.spawn((
                LeftHand,
                SpatialBundle {
                    transform: Transform::from_xyz(-0.4, 1.0, 0.4),
                    ..default()
                },
            ));
        });
}
