use super::components::{Enemy, EnemyAI, EnemyAIState};
use crate::game::animation::components::{AnimationIntent, CharacterBones};
use crate::game::combat::components::{
    AttackIntent, CombatInput, CombatState, EquippedWeapon, GuardIntent, LeftHand,
    RightHand,
};
use crate::game::stamina::components::Stamina;
use crate::game::health::components::Health;
use crate::game::ik::components::IkTarget;
use crate::game::lock_on::components::Lockable;
use crate::game::placement::components::{EnemySpawnVariant, Placement, PlacementKind, PlacementState, ChunkEntityLink, LinkedPlacement};
use crate::game::player::components::PendingEquipment;
use crate::game::poise::components::Poise;
use crate::game::shield::components::{BlockingState, EquippedShield};
use crate::game::shield::data::{build_shield_hierarchy, get_shield_stats_bundle, ShieldType};
use crate::game::weapon::components::WeaponType;
use crate::game::weapon::data::{get_unarmed_config, get_weapon_data};
use crate::game::common::components::{HurtboxProfile, HUMAN_PROXIES};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_test_enemies(mut commands: Commands) {
    let test_positions = [
        Vec3::new(4.0, 20.0, 4.0),
        Vec3::new(-4.0, 20.0, 4.0),
        Vec3::new(0.0, 20.0, 6.0),
    ];

    let variants = [
        EnemySpawnVariant::LightSpearman,
        EnemySpawnVariant::Vanguard,
        EnemySpawnVariant::UnarmedMonster,
    ];

    for (i, pos) in test_positions.iter().enumerate() {
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(*pos),
                ..default()
            },
            Placement {
                kind: PlacementKind::Enemy(variants[i % variants.len()].clone()),
                state: PlacementState::Active,
                ..default()
            },
        ));
    }
    
    info!("Test düşmanları oyuncunun (0, 20, 0) çevresinde Placement olarak oluşturuldu!");
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut placement_query: Query<(Entity, &GlobalTransform, &mut Placement)>,
) {
    let character_scene = asset_server.load("models/proxyfull.glb#Scene0");

    for (placement_entity, global_transform, mut placement) in placement_query.iter_mut() {
        if placement.state != PlacementState::Active {
            continue;
        }

        let transform = global_transform.compute_transform();
        
        info!(
            "Aktif Placement bulundu! Entity: {:?}, Dünya Pozisyonu: {:?}, Tür: {:?}", 
            placement_entity, transform.translation, placement.kind
        );

        let (variant, is_boss) = match &placement.kind {
            PlacementKind::Enemy(v) => {
                let is_boss = *v == EnemySpawnVariant::BossArena;
                (v.clone(), is_boss)
            }
            _ => {
                continue;
            }
        };

        placement.state = PlacementState::Consumed;

        let mut position = transform.translation;
        // Kritik Düzeltme: Y ofsetini 2.5'ten 1.0'a çektik.
        // Capsule yüksekliği ~1.6. Zemine (0.0) yerleştirdiğimizde 1.0 ofset kafayı 1.8 hizasında tutar. 
        // Böylece mağara tavanına girip physics patlamasına neden olmaz.
        position.y += 1.0;

        let enemy_entity = commands.spawn_empty().id();

        let (mut hp, mut poise, speed, detect, attack_range, opt_weapon_type, shield_type, cooldown) =
            match variant {
                EnemySpawnVariant::LightSpearman => {
                    (80.0, 20.0, 1.5, 15.0, 2.5, Some(WeaponType::Spear), None, 1.0)
                }
                EnemySpawnVariant::Vanguard => (
                    120.0,
                    50.0,
                    1.0,
                    12.0,
                    1.5,
                    Some(WeaponType::VampireBlade),
                    Some(ShieldType::KiteShield),
                    1.5,
                ),
                EnemySpawnVariant::HeavyDefender | EnemySpawnVariant::BossArena => (
                    200.0,
                    100.0,
                    0.6,
                    10.0,
                    1.5,
                    Some(WeaponType::VampireBlade),
                    Some(ShieldType::TowerShield),
                    2.5,
                ),
                EnemySpawnVariant::UnarmedMonster => {
                    (160.0, 80.0, 1.8, 14.0, 1.8, None, None, 1.2)
                }
            };

        let mut final_scale = transform.scale;
        if is_boss {
            hp *= 2.5;
            poise *= 2.0;
            final_scale *= 1.3;
        }

        let mut entity_commands = commands.entity(enemy_entity);

        entity_commands.insert((
            SpatialBundle {
                transform: Transform::from_translation(position).with_scale(final_scale),
                ..default()
            },
            Enemy,
            Lockable,
            Health::new(hp),
            Poise {
                current: poise,
                max: poise,
                ..default()
            },
            EnemyAIState::Idle,
            EnemyAI {
                detection_range: detect,
                attack_range: attack_range,
                speed,
                attack_cooldown: Timer::from_seconds(cooldown, TimerMode::Once),
            },
        ));

        entity_commands.insert(HurtboxProfile { proxies: HUMAN_PROXIES });

        entity_commands.insert((
            CombatState::default(),
            Stamina::default(),
            AttackIntent::default(),
            CombatInput::default(),
            GuardIntent::default(),
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::capsule_y(0.6, 0.2),
            LockedAxes::ROTATION_LOCKED,
            CharacterBones::default(),
            AnimationIntent::default(),
            IkTarget {
                position: Vec3::ZERO,
                weight: 0.0,
            },
            BlockingState::Idle,
        ));

        entity_commands.insert((
            ChunkEntityLink(placement.chunk_coord),
            LinkedPlacement(placement_entity),
        ));

        if let Some(w_type) = opt_weapon_type {
            entity_commands.insert(get_weapon_data(w_type));
        } else {
            entity_commands.insert(get_unarmed_config());
        }

        entity_commands.with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: character_scene.clone(),
                transform: Transform::from_xyz(0.0, -1.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                ..default()
            });

            if opt_weapon_type.is_some() {
                parent.spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0)
                            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
                        ..default()
                    },
                    EquippedWeapon,
                    IkTarget {
                        position: Vec3::ZERO,
                        weight: 0.0,
                    },
                    PendingEquipment,
                ));
            } else {
                parent.spawn((
                    RightHand,
                    SpatialBundle {
                        transform: Transform::from_xyz(0.4, 1.0, 0.4),
                        ..default()
                    },
                    IkTarget {
                        position: Vec3::ZERO,
                        weight: 0.0,
                    },
                ));

                parent.spawn((
                    LeftHand,
                    SpatialBundle {
                        transform: Transform::from_xyz(-0.4, 1.0, 0.4),
                        ..default()
                    },
                    IkTarget {
                        position: Vec3::ZERO,
                        weight: 0.0,
                    },
                ));
            }
        });

        if let Some(s_type) = shield_type {
            commands
                .entity(enemy_entity)
                .insert((get_shield_stats_bundle(s_type),))
                .with_children(|parent| {
                    parent
                        .spawn((
                            SpatialBundle {
                                transform: Transform::from_rotation(Quat::from_rotation_z(
                                    std::f32::consts::PI,
                                )),
                                ..default()
                            },
                            EquippedShield,
                            PendingEquipment,
                            s_type,
                        ))
                        .with_children(|shield_root| {
                            build_shield_hierarchy(
                                shield_root,
                                s_type,
                                enemy_entity,
                                &mut meshes,
                                &mut materials,
                            );
                        });
                });
        }
    }
}