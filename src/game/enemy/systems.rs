use super::components::{Enemy, EnemyAI, EnemyAIState};
use crate::game::animation::components::AnimationIntent;
use crate::game::combat::components::{
    AttackIntent, CombatInput, CombatState, GuardIntent, UnarmedConfig,
};
use crate::game::health::components::Health;
use crate::game::player::components::Player;
use crate::game::shield::components::BlockingState;
use crate::game::weapon::components::{AttackType, Weapon};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::world::generation::chunk_tracker::ChunkUnloadEvent;
use crate::game::placement::components::{Placement, PlacementState, ChunkEntityLink, LinkedPlacement};
// --- YENİ BİLEŞENLER: Bölge Aidiyeti ve Kalıcı Veri Bağı ---


// --- YENİ SİSTEM: Chunk Unload Olduğunda Düşmanları Yönet ---

pub fn handle_enemy_despawn(
    mut commands: Commands,
    mut unload_events: EventReader<ChunkUnloadEvent>,
    enemy_query: Query<(Entity, &ChunkEntityLink, &LinkedPlacement, &Transform, &Health), With<Enemy>>,
    mut placement_query: Query<(&mut Placement, &mut Transform), Without<Enemy>>,
) {
    for unload_event in unload_events.read() {
        let unloaded_chunk = unload_event.0;

        for (enemy_entity, link, linked_placement, enemy_transform, health) in enemy_query.iter() {
            if link.0 == unloaded_chunk {
                if health.current <= 0.0 {
                    // Düşman ölmüşse kalıcı (Placement) verisini dünyadan tamamen sil ki bir daha doğmasın
                    if commands.get_entity(linked_placement.0).is_some() {
                        commands.entity(linked_placement.0).despawn_recursive();
                    }
                } else {
                    // Düşman yaşıyorsa mevcut durumunu (Transform) kaydet ve tekrar doğabilmesi için Active yap
                    if let Ok((mut placement, mut placement_transform)) = placement_query.get_mut(linked_placement.0) {
                        *placement_transform = *enemy_transform;
                        placement.state = PlacementState::Active;
                    }
                }
                
                // Çalışma zamanı entity'sini dünyadan güvenlice temizle (aşağı düşme bug'ı çözümü)
                commands.entity(enemy_entity).despawn_recursive();
            }
        }
    }
}

// --- GÜNCELLENEN SİSTEM: CPU Optimizasyonlu Yapay Zeka ---

pub fn enemy_ai_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (
            Entity,
            &mut Transform,
            &mut Velocity,
            &mut EnemyAI,
            &mut EnemyAIState,
            &mut CombatState,
            &mut CombatInput,
            Option<&mut BlockingState>,
            Option<&mut GuardIntent>,
            Option<&Weapon>,
            Option<&UnarmedConfig>,
            &Health,
            &mut AnimationIntent,
            &mut AttackIntent,
        ),
        (With<Enemy>, Without<Player>),
    >,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let player_pos = player_transform.translation;

    for (
        entity,
        mut transform,
        mut velocity,
        mut ai,
        mut ai_state,
        mut combat_state,
        mut combat_input,
        mut opt_blocking_state,
        mut opt_guard_intent,
        weapon_opt,
        unarmed_opt,
        health,
        mut animation_intent,
        mut attack_intent,
    ) in enemy_query.iter_mut()
    {
        ai.attack_cooldown.tick(time.delta());

        if health.current <= 0.0 {
            *ai_state = EnemyAIState::Dead;
            velocity.linvel.x = 0.0;
            velocity.linvel.z = 0.0;
            animation_intent.is_moving = false;
            animation_intent.is_attacking = false;
            continue;
        }

        if matches!(*combat_state, CombatState::Recoiled { .. }) {
            velocity.linvel.x = 0.0;
            velocity.linvel.z = 0.0;
            if let Some(mut blocking_state) = opt_blocking_state.as_deref_mut() {
                *blocking_state = BlockingState::Idle;
            }
            animation_intent.is_moving = false;
            animation_intent.is_attacking = false;
            continue;
        }

        let enemy_pos = transform.translation;
        let distance = enemy_pos.distance(player_pos);

        // YENİ: CPU Optimizasyonu - Mesafeye Göre Early Out
        // Düşman 60 metreden uzaksa ağır yapay zeka, yön ve animasyon kodlarını atla.
        if distance > 60.0 {
            velocity.linvel.x = 0.0;
            velocity.linvel.z = 0.0;
            animation_intent.is_moving = false;
            animation_intent.is_attacking = false;
            if let Some(mut blocking_state) = opt_blocking_state.as_deref_mut() {
                *blocking_state = BlockingState::Idle;
            }
            continue;
        }

        let mut direction = player_pos - enemy_pos;
        direction.y = 0.0;
        let direction_flat = direction.normalize_or_zero();

        let random_hash =
            ((time.elapsed_seconds() * 10.0) as usize + entity.index() as usize) % 100;
        let t = time.elapsed_seconds() + entity.index() as f32;

        if let (Some(blocking_state), Some(guard_intent)) = (
            opt_blocking_state.as_deref_mut(),
            opt_guard_intent.as_deref_mut(),
        ) {
            let shield_x_offset = (t * 2.0).sin() * 0.4;
            let shield_y_offset = (t * 3.0).cos() * 0.4;

            if matches!(*blocking_state, BlockingState::Blocking) {
                guard_intent.offset = Vec2::new(shield_x_offset, shield_y_offset);
            } else {
                guard_intent.offset = guard_intent.offset.lerp(Vec2::ZERO, 0.1);
            }
        }

        match *ai_state {
            EnemyAIState::Idle => {
                velocity.linvel.x = 0.0;
                velocity.linvel.z = 0.0;
                if distance <= ai.detection_range {
                    *ai_state = EnemyAIState::Chase;
                }
            }
            EnemyAIState::Chase => {
                if let Some(mut blocking_state) = opt_blocking_state.as_deref_mut() {
                    if random_hash < 40 {
                        *blocking_state = BlockingState::Blocking;
                    } else {
                        *blocking_state = BlockingState::Idle;
                    }
                }

                if distance > ai.detection_range {
                    *ai_state = EnemyAIState::Idle;
                    velocity.linvel.x = 0.0;
                    velocity.linvel.z = 0.0;
                } else if distance <= ai.attack_range {
                    *ai_state = EnemyAIState::Attack;
                    velocity.linvel.x = 0.0;
                    velocity.linvel.z = 0.0;
                } else {
                    let movement_vel = direction_flat * ai.speed;
                    velocity.linvel.x = movement_vel.x;
                    velocity.linvel.z = movement_vel.z;
                    
                    let target_look = Vec3::new(player_pos.x, enemy_pos.y, player_pos.z);
                    transform.look_at(target_look, Vec3::Y);
                }
            }
            EnemyAIState::Attack => {
                velocity.linvel.x = 0.0;
                velocity.linvel.z = 0.0;

                if let Some(mut blocking_state) = opt_blocking_state.as_deref_mut() {
                    *blocking_state = BlockingState::Idle;
                }

                let target_look = Vec3::new(player_pos.x, enemy_pos.y, player_pos.z);
                transform.look_at(target_look, Vec3::Y);

                if distance > ai.attack_range && matches!(*combat_state, CombatState::Idle) {
                    *ai_state = EnemyAIState::Chase;
                } else if matches!(*combat_state, CombatState::Idle)
                    && ai.attack_cooldown.finished()
                {
                    if let Some(weapon) = weapon_opt {
                        let attacks: Vec<_> = weapon.attacks.keys().collect();
                        if !attacks.is_empty() {
                            let rand_index = random_hash % attacks.len();
                            let chosen_attack = *attacks[rand_index];

                            if let Some(attack_def) = weapon.attacks.get(&chosen_attack) {
                                combat_input.attack_held = random_hash % 2 == 0;
                                *combat_state = CombatState::Windup {
                                    timer: Timer::from_seconds(
                                        attack_def.windup_duration,
                                        TimerMode::Once,
                                    ),
                                    charge_timer: Timer::from_seconds(
                                        attack_def.max_charge_duration,
                                        TimerMode::Once,
                                    ),
                                    attack_type: chosen_attack,
                                };
                                attack_intent.current = chosen_attack;
                                ai.attack_cooldown.reset();
                            }
                        }
                    } else if let Some(unarmed) = unarmed_opt {
                        let chosen_attack = if random_hash % 2 == 0 {
                            AttackType::UnarmedRight
                        } else {
                            AttackType::UnarmedLeft
                        };

                        if let Some(attack_def) = unarmed.attacks.get(&chosen_attack) {
                            combat_input.attack_held = false;
                            *combat_state = CombatState::Windup {
                                timer: Timer::from_seconds(
                                    attack_def.windup_duration,
                                    TimerMode::Once,
                                ),
                                charge_timer: Timer::from_seconds(
                                    attack_def.max_charge_duration,
                                    TimerMode::Once,
                                ),
                                attack_type: chosen_attack,
                            };
                            attack_intent.current = chosen_attack;
                            ai.attack_cooldown.reset();
                        }
                    }
                } else if let CombatState::Windup { .. } = *combat_state {
                    if combat_input.attack_held && random_hash < 5 {
                        combat_input.attack_held = false;
                    }
                }
            }
            EnemyAIState::Dead => {
                velocity.linvel.x = 0.0;
                velocity.linvel.z = 0.0;
                if let Some(mut blocking_state) = opt_blocking_state.as_deref_mut() {
                    *blocking_state = BlockingState::Idle;
                }
            }
        }

        animation_intent.is_moving = velocity.linvel.length_squared() > 0.1;
        animation_intent.is_sprinting = false;
        animation_intent.is_attacking = !matches!(*combat_state, CombatState::Idle);

        if let Some(blocking_state) = opt_blocking_state.as_deref() {
            animation_intent.is_blocking = matches!(*blocking_state, BlockingState::Blocking);
        } else {
            animation_intent.is_blocking = false;
        }
    }
}