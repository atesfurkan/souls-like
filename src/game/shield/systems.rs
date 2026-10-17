use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy_rapier3d::prelude::*;

use super::components::{BlockingState, EquippedShield, Shield, ShieldBox, UnarmedShieldBox};
use super::data::{build_shield_hierarchy, ShieldType};
use super::events::ShieldSwapRequestEvent;

use crate::game::combat::components::GuardIntent;
use crate::game::ik::components::IkTarget;
use crate::game::player::components::PendingEquipment;

pub fn process_shield_swap(
    mut commands: Commands,
    mut events: EventReader<ShieldSwapRequestEvent>,
    anchor_query: Query<(&Parent, &ShieldBox)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        // 1. ESKİ KALKANI SİL (Hiyerarşideki ShieldBox'ı ve Parent'ını bulup sil)
        for (parent, shield_box) in anchor_query.iter() {
            if shield_box.owner == event.entity {
                commands.entity(parent.get()).despawn_recursive();
            }
        }

        // Kök entity üzerindeki eski Shield statlarını sil
        commands.entity(event.entity).remove::<Shield>();
        commands.entity(event.entity).remove::<ShieldType>();

        // 2. YENİ BİR KALKAN VARSA ONU OLUŞTUR VE EKLE
        if let Some(target_shield) = event.target_shield {
            commands.entity(event.entity).with_children(|parent| {
                parent
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_rotation(Quat::from_rotation_z(
                                std::f32::consts::PI,
                            )),
                            ..default()
                        },
                        EquippedShield,
                        GuardIntent::default(),
                        IkTarget { position: Vec3::ZERO, weight: 0.0 },
                        BlockingState::Idle,
                        PendingEquipment, 
                        target_shield,
                    ))
                    .with_children(|shield_root| {
                        build_shield_hierarchy(
                            shield_root,
                            target_shield,
                            event.entity,
                            &mut meshes,
                            &mut materials,
                        );
                    });
            });

            let (new_stats, _) = crate::game::shield::data::get_shield_stats_bundle(target_shield);
            
            commands.entity(event.entity).insert((
                new_stats,
                target_shield, 
            ));

            println!("✅ Yeni kalkan başarıyla kuşandı: {:?}", target_shield);
        } else {
            println!("✅ Kalkan çıkarıldı (Sol El: Unarmed).");
        }
    }
}

pub fn handle_shield_input(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut BlockingState, Option<&Shield>)>,
) {
    for (mut state, shield) in query.iter_mut() {
        let parry_window = shield.map(|s| s.parry_window).unwrap_or(0.2);

        if mouse.just_pressed(MouseButton::Right) {
            if matches!(*state, BlockingState::Idle) {
                *state = BlockingState::Parrying {
                    timer: Timer::from_seconds(parry_window, TimerMode::Once),
                };
            }
        } else if mouse.pressed(MouseButton::Right) {
            if let BlockingState::Parrying { timer } = &mut *state {
                timer.tick(time.delta());
                if timer.just_finished() {
                    *state = BlockingState::Blocking;
                }
            }
        } else if mouse.just_released(MouseButton::Right) {
            if matches!(
                *state,
                BlockingState::Parrying { .. } | BlockingState::Blocking
            ) {
                *state = BlockingState::Idle;
            }
        }
    }
}

pub fn handle_unarmed_block_box(
    mut commands: Commands,
    query: Query<(Entity, &BlockingState, Option<&ShieldType>), Changed<BlockingState>>,
    unarmed_boxes: Query<(Entity, &ShieldBox), With<UnarmedShieldBox>>,
) {
    for (player_entity, state, shield_type) in query.iter() {
        if shield_type.is_some() {
            continue;
        }

        let is_blocking = matches!(*state, BlockingState::Blocking | BlockingState::Parrying { .. });

        let mut existing_box = None;
        for (box_ent, shield_box) in unarmed_boxes.iter() {
            if shield_box.owner == player_entity {
                existing_box = Some(box_ent);
                break;
            }
        }

        if is_blocking && existing_box.is_none() {
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn((
                    UnarmedShieldBox,
                    ShieldBox { owner: player_entity },
                    Collider::cuboid(0.3, 0.4, 0.2),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.35, -0.4),
                        ..default()
                    },
                ));
            });
        } else if !is_blocking && existing_box.is_some() {
            commands.entity(existing_box.unwrap()).despawn_recursive();
        }
    }
}