use crate::game::combat::components::{
    AttackIntent, CombatInput, CombatState, Hitbox, LeftHand, RightHand, UnarmedConfig,
};
use crate::game::shield::events::RecoilEvent;
use crate::game::weapon::components::{AttackDefinition, AttackType, Weapon};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn get_attack_definition(
    attack_type: &AttackType,
    weapon_opt: Option<&Weapon>,
    unarmed_opt: Option<&UnarmedConfig>,
) -> Option<AttackDefinition> {
    if let Some(weapon) = weapon_opt {
        if let Some(def) = weapon.attacks.get(attack_type) {
            return Some(def.clone());
        }
    }
    if let Some(unarmed) = unarmed_opt {
        if let Some(def) = unarmed.attacks.get(attack_type) {
            return Some(def.clone());
        }
    }
    None
}

pub fn update_combat_state(
    time: Res<Time>,
    mut query: Query<(
        &mut CombatState,
        &CombatInput,
        Option<&Weapon>,
        Option<&UnarmedConfig>,
    )>,
) {
    for (mut state, input, weapon_opt, unarmed_opt) in query.iter_mut() {
        match *state {
            CombatState::Idle => {}
            CombatState::Windup {
                ref mut timer,
                ref mut charge_timer,
                attack_type,
            } => {
                timer.tick(time.delta());
                charge_timer.tick(time.delta());

                let can_transition =
                    timer.finished() && (!input.attack_held || charge_timer.finished());

                if can_transition {
                    if let Some(def) = get_attack_definition(&attack_type, weapon_opt, unarmed_opt)
                    {
                        *state = CombatState::Active {
                            timer: Timer::from_seconds(def.active_duration, TimerMode::Once),
                            hitbox_entity: None,
                            attack_type,
                        };
                    } else {
                        *state = CombatState::Idle;
                    }
                }
            }
            CombatState::Active {
                ref mut timer,
                hitbox_entity: _,
                attack_type,
            } => {
                timer.tick(time.delta());
                if timer.finished() {
                    if let Some(def) = get_attack_definition(&attack_type, weapon_opt, unarmed_opt)
                    {
                        *state = CombatState::Recovery {
                            timer: Timer::from_seconds(def.recovery_duration, TimerMode::Once),
                            attack_type,
                        };
                    } else {
                        *state = CombatState::Idle;
                    }
                }
            }
            CombatState::Recovery {
                ref mut timer,
                attack_type: _,
            } => {
                timer.tick(time.delta());
                if timer.finished() {
                    *state = CombatState::Idle;
                }
            }
            CombatState::Recoiled { ref mut timer } => {
                timer.tick(time.delta());
                if timer.finished() {
                    *state = CombatState::Idle;
                }
            }
        }
    }
}

pub fn spawn_hitbox_on_attack(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut CombatState,
        Option<&Weapon>,
        Option<&UnarmedConfig>,
    )>,
    children_query: Query<&Children>,
    right_hand_query: Query<Entity, With<RightHand>>,
    left_hand_query: Query<Entity, With<LeftHand>>,
) {
    for (entity, mut state, weapon_opt, unarmed_opt) in query.iter_mut() {
        if let CombatState::Active {
            ref mut hitbox_entity,
            attack_type,
            ..
        } = *state
        {
            if hitbox_entity.is_some() {
                continue;
            }

            if let Some(def) = get_attack_definition(&attack_type, weapon_opt, unarmed_opt) {
                let mut spawn_parent = entity;

                if attack_type == AttackType::UnarmedRight {
                    for child in children_query.iter_descendants(entity) {
                        if right_hand_query.get(child).is_ok() {
                            spawn_parent = child;
                            break;
                        }
                    }
                } else if attack_type == AttackType::UnarmedLeft {
                    for child in children_query.iter_descendants(entity) {
                        if left_hand_query.get(child).is_ok() {
                            spawn_parent = child;
                            break;
                        }
                    }
                }

                let spawned_hitbox = commands
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_translation(def.hitbox_offset),
                            ..default()
                        },
                        Collider::ball(def.hitbox_radius),
                        Sensor,
                        ActiveEvents::COLLISION_EVENTS,
                        Hitbox {
                            damage: def.damage,
                            poise_damage: def.poise_damage,
                            owner: entity,
                            hit_entities: Default::default(),
                            lifesteal: def.lifesteal,
                        },
                    ))
                    .id();

                commands.entity(spawn_parent).add_child(spawned_hitbox);
                *hitbox_entity = Some(spawned_hitbox);
            }
        }
    }
}

pub fn cleanup_hitboxes(
    mut commands: Commands,
    mut query: Query<&mut CombatState>,
    hitbox_query: Query<Entity, With<Hitbox>>,
) {
    for mut state in query.iter_mut() {
        match *state {
            CombatState::Recovery { .. } | CombatState::Idle | CombatState::Recoiled { .. } => {
                if let CombatState::Active {
                    ref mut hitbox_entity,
                    ..
                } = *state
                {
                    if let Some(hb) = hitbox_entity.take() {
                        if hitbox_query.get(hb).is_ok() {
                            commands.entity(hb).despawn_recursive();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn handle_recoil_events(
    mut recoil_events: EventReader<RecoilEvent>,
    mut combat_query: Query<&mut CombatState>,
) {
    for event in recoil_events.read() {
        if let Ok(mut combat_state) = combat_query.get_mut(event.target) {
            *combat_state = CombatState::Recoiled {
                timer: Timer::from_seconds(0.8, TimerMode::Once),
            };
        }
    }
}