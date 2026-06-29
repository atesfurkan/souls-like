use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::common::components::MainCamera;
use crate::game::stamina::components::Stamina;
use crate::game::health::components::Health;
use crate::game::dodge::components::DodgeState;
use crate::game::combat::components::{CombatState,  Hurtbox,
      AttackIntent , EquippedWeapon};
use crate::game::lock_on::components::LockOnState;

use crate::game::weapon::components::{Weapon ,
     AttackDefinition, AttackType};
use crate::game::weapon::data:: {get_weapon_data , WeaponType } ;

use bevy::utils::HashMap;

use super::components::{Player, PlayerState, PlayerMovementConfig}; 

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)  {
    let player_entity = commands.spawn_empty().id();
    // Materyalleri hazırlayalım
    let body_material = materials.add(Color::srgb(0.2, 0.5, 0.8));
    let skin_material = materials.add(Color::srgb(0.9, 0.7, 0.6));
    let weapon_wood = materials.add(Color::srgb(0.4, 0.2, 0.1));
    let weapon_metal = materials.add(Color::srgb(0.8, 0.8, 0.8));

    // Ana oyuncu kapsülü (Ebeveyn / Parent) 
    commands.entity(player_entity).insert((
        PbrBundle {
            mesh: meshes.add(Capsule3d { radius: 0.5, half_length: 0.5 }),
            material: body_material.clone(),
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            ..default()
        },
        Player,
        PlayerState::default(),
        CombatState::default(), // Başlangıçta Idle
        AttackIntent::default(), 
        // --- YENİ EKLENEN SİSTEMLER ---
        Health::new(100.0), // Oyuncunun canı
        Hurtbox { owner: player_entity }, // Oyuncunun hasar alma kutusu (sahibi kendisi)
        Stamina::default(),

        get_weapon_data(WeaponType::Spear),

        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.5, 0.5), 
        KinematicCharacterController::default(),
        
    ))
    .with_children(|parent| {
        // --- ANATOMİ ---
        
        // Kafa (Sphere)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Sphere { radius: 0.35 }), // DÜZELTİLDİ
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, 0.9, 0.0), // Kapsülün tam üstünde
            ..default()
        });

        // Sol Kol (Cylinder)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.12, half_height: 0.35 }), // DÜZELTİLDİ (half_height kullanır)
            material: skin_material.clone(),
            transform: Transform::from_xyz(-0.6, 0.2, 0.0),
            ..default()
        });

        // Sağ Kol (Cylinder)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.12, half_height: 0.35 }), // DÜZELTİLDİ
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.6, 0.2, 0.0),
            ..default()
        });

        // Sol Bacak (Cylinder)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.15, half_height: 0.4 }), // DÜZELTİLDİ
            material: body_material.clone(),
            transform: Transform::from_xyz(-0.25, -0.9, 0.0),
            ..default()
        });

        // Sağ Bacak (Cylinder)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.15, half_height: 0.4 }), // DÜZELTİLDİ
            material: body_material.clone(),
            transform: Transform::from_xyz(0.25, -0.9, 0.0),
            ..default()
        });

        parent.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.7, 0.0, -0.6)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
            ..default()
        },
        EquippedWeapon,
    ));
    });
}