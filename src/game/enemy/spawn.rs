use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::components::{Enemy, EnemyAI, EnemyAIState};
use crate::game::health::components::Health;
use crate::game::player::components::Player;
use crate::game::combat::components::{ StaggerEvent, CombatState,
      EquippedWeapon , 
      AttackIntent , Hurtbox };//burada neden eq. weapon var lan ,bi sorak
use crate::game::lock_on::components::Lockable;
use crate::game::enemy::components::StaggerState;

use crate::game::weapon::components::{Weapon ,
     AttackDefinition, AttackType};
use crate::game::weapon::data:: {get_weapon_data , WeaponType } ;
use bevy::utils::HashMap;



pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)  {
    // 1. Önce ID alıyoruz
    let enemy_entity = commands.spawn_empty().id();

    let enemy_material = materials.add(Color::srgb(0.8, 0.1, 0.1));
    let weapon_wood = materials.add(Color::srgb(0.4, 0.2, 0.1));
    let weapon_metal = materials.add(Color::srgb(0.8, 0.8, 0.8));
    

    commands.entity(enemy_entity).insert((
        // GÖRSEL (PbrBundle)
        PbrBundle {
            mesh: meshes.add(Capsule3d { radius: 0.6, half_length: 0.6 }),
            material: enemy_material,
            transform: Transform::from_xyz(5.0, 3.0, -5.0),
            ..default()
        },
        Enemy,
        Lockable,
        Health::new(100.0),
        Hurtbox { owner: enemy_entity }, // Düşmanın hasar alma kutusu
        
        // --- YENİ: AI ve Savaş Bileşenleri ---
        EnemyAIState::Idle,
        EnemyAI {
            detection_range: 12.0,
            attack_range: 2.0, // Mızrak menzili oyuncuya yetecek kadar uzun
            speed: 3.0,
            attack_cooldown: Timer::from_seconds(1.5, TimerMode::Once), // 1.5 saniyede bir vurabilir
        },
        CombatState::default(),
        AttackIntent::default(),
        get_weapon_data(WeaponType::Spear),

        // --- Fizik Bileşenleri ---
        RigidBody::Dynamic, 
        Velocity::default(), // Hareketi kontrol etmek için eklendi
        Collider::capsule_y(0.6, 0.6),
        LockedAxes::ROTATION_LOCKED, 
    ))
    .with_children(|parent| {
        // --- SİLAH (SPEAR) ---
        // Oyuncunun mızrağının birebir aynısı düşmanın eline de bağlanıyor
        parent.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.7, 0.0, -0.6)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
            ..default()
        },
        EquippedWeapon, // Animasyon sistemin bu markeri arıyor, onu koruyoruz
    ));
    });
}