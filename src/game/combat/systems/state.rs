use crate::game::combat::components::{CombatInput, CombatState, EquippedWeapon, Hitbox, RightHand, LeftHand, UnarmedConfig};
use crate::game::weapon::components::{Weapon, AttackType};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;
use crate::game::poise::events::PoiseBrokenEvent;
use crate::game::combat::components::StaggerEvent;

pub fn trigger_stagger_on_poise_break(
    mut poise_broken_events: EventReader<PoiseBrokenEvent>,
    mut stagger_events: EventWriter<StaggerEvent>,
) {
    for event in poise_broken_events.read() {
        stagger_events.send(StaggerEvent {
            target: event.target,
            force: 1.0,     
            duration: 0.5,  
        });
    }
}

// Hiyerarşide belirli bir marker componenti (T) taşıyan ve owner_entity'ye ait olan çocuk entity'i bulan yardımcı fonksiyon
fn find_child_with_marker<T: Component>(
    target_marker_query: &Query<(Entity, &Parent), With<T>>,
    parent_query: &Query<&Parent>,
    owner_entity: Entity,
) -> Option<Entity> {
    for (eq_ent, eq_parent) in target_marker_query.iter() {
        let mut current = eq_parent.get();
        loop {
            if current == owner_entity {
                return Some(eq_ent);
            }
            if let Ok(p) = parent_query.get(current) {
                current = p.get();
            } else {
                break;
            }
        }
    }
    None
}

pub fn process_attack_states(
    mut commands: Commands,
    time: Res<Time>,
    // YENİ: Hem Weapon hem de UnarmedConfig esnek bir şekilde kontrol ediliyor
    mut query: Query<(Entity, &mut CombatState, Option<&Weapon>, Option<&UnarmedConfig>, Option<&CombatInput>)>,
    weapon_query: Query<(Entity, &Parent), With<EquippedWeapon>>,
    right_hand_query: Query<(Entity, &Parent), With<RightHand>>,
    left_hand_query: Query<(Entity, &Parent), With<LeftHand>>,
    parent_query: Query<&Parent>,
) {
    for (entity, mut combat_state, weapon_opt, unarmed_opt, combat_input) in query.iter_mut() {
        let mut next_state = None;

        match &mut *combat_state {
            CombatState::Idle => {}

            CombatState::Windup {
                timer,
                charge_timer,
                attack_type,
            } => {
                let mut transition_to_active = false;
                timer.tick(time.delta());

                if timer.finished() {
                    let is_held = combat_input.map(|i| i.attack_held).unwrap_or(false);

                    if is_held && !charge_timer.finished() {
                        charge_timer.tick(time.delta());
                    } else {
                        transition_to_active = true;
                    }
                }

                if transition_to_active {
                    let mut attack_def_opt = None;
                    let mut target_spawn_entity = entity;

                    // 1. Silahlı Saldırı Tespiti
                    if let Some(weapon) = weapon_opt {
                        if let Some(def) = weapon.attacks.get(attack_type) {
                            attack_def_opt = Some(def.clone());
                            if let Some(found_wep) = find_child_with_marker(&weapon_query, &parent_query, entity) {
                                target_spawn_entity = found_wep;
                            }
                        }
                    }

                    // 2. Unarmed (Silahsız) Saldırı Tespiti
                    if attack_def_opt.is_none() {
                        if let Some(unarmed) = unarmed_opt {
                            if let Some(def) = unarmed.attacks.get(attack_type) {
                                attack_def_opt = Some(def.clone());
                                
                                // Doğru eli bularak hitbox'ı oraya konumlandırma kararı
                                if *attack_type == AttackType::UnarmedRight {
                                    if let Some(rh) = find_child_with_marker(&right_hand_query, &parent_query, entity) {
                                        target_spawn_entity = rh;
                                    }
                                } else if *attack_type == AttackType::UnarmedLeft {
                                    if let Some(lh) = find_child_with_marker(&left_hand_query, &parent_query, entity) {
                                        target_spawn_entity = lh;
                                    }
                                }
                            }
                        }
                    }

                    // Saldırı tanımı bulunduysa hitbox oluşturulur
                    if let Some(attack_def) = attack_def_opt {
                        let charge_multiplier = 1.0 + charge_timer.fraction();

                        let hitbox_entity = commands
                            .spawn((
                                Hitbox {
                                    damage: attack_def.damage * charge_multiplier,
                                    poise_damage: attack_def.poise_damage * charge_multiplier,
                                    owner: entity,
                                    hit_entities: HashSet::new(),
                                    lifesteal: attack_def.lifesteal,
                                },
                                Collider::ball(attack_def.hitbox_radius),
                                Sensor,
                                ActiveEvents::COLLISION_EVENTS,
                                TransformBundle::from_transform(Transform::from_translation(
                                    attack_def.hitbox_offset,
                                )),
                            ))
                            .id();

                        commands
                            .entity(target_spawn_entity)
                            .add_child(hitbox_entity);

                        next_state = Some(CombatState::Active {
                            timer: Timer::from_seconds(attack_def.active_duration, TimerMode::Once),
                            hitbox_entity: Some(hitbox_entity),
                            attack_type: *attack_type,
                        });
                    } else {
                        // Geçersiz bir duruma düşülürse (Saldırı configte yoksa)
                        next_state = Some(CombatState::Idle);
                    }
                }
            }
            CombatState::Active {
                timer,
                hitbox_entity,
                attack_type,
            } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if let Some(h_entity) = hitbox_entity {
                        if let Some(mut entity_cmd) = commands.get_entity(*h_entity) {
                            entity_cmd.despawn_recursive();
                        }
                    }
                    
                    let mut rec_dur = 0.5; // Fallback
                    
                    if let Some(w) = weapon_opt {
                        if let Some(def) = w.attacks.get(attack_type) { rec_dur = def.recovery_duration; }
                    }
                    if let Some(u) = unarmed_opt {
                        if let Some(def) = u.attacks.get(attack_type) { rec_dur = def.recovery_duration; }
                    }

                    next_state = Some(CombatState::Recovery {
                        timer: Timer::from_seconds(rec_dur, TimerMode::Once),
                        attack_type: *attack_type,
                    });
                }
            }
            CombatState::Recovery { timer, .. } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    next_state = Some(CombatState::Idle);
                }
            }
            CombatState::Recoiled { timer } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    next_state = Some(CombatState::Idle);
                }
            }
        }

        if let Some(state) = next_state {
            *combat_state = state;
        }
    }
}