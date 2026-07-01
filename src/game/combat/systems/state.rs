use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;
use crate::game::combat::components::{CombatState, Hitbox};
use crate::game::weapon::components::Weapon;

pub fn combat_state_machine(time: Res<Time>, mut query: Query<(&mut CombatState, &Weapon)>) {
    for (mut state, weapon) in query.iter_mut() {
        let mut next_state = None;
        match *state {
            CombatState::Windup { ref mut timer, attack_type } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let def = weapon.attacks.get(&attack_type).unwrap();
                    next_state = Some(CombatState::Active {
                        timer: Timer::from_seconds(def.active_duration, TimerMode::Once),
                        hitbox_entity: None,
                        attack_type,
                    });
                }
            }
            CombatState::Active { ref mut timer, attack_type, .. } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let def = weapon.attacks.get(&attack_type).unwrap();
                    next_state = Some(CombatState::Recovery {
                        timer: Timer::from_seconds(def.recovery_duration, TimerMode::Once),
                        attack_type,
                    });
                }
            }
            CombatState::Recovery { ref mut timer, .. } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    next_state = Some(CombatState::Idle);
                }
            }
            CombatState::Idle => {}
            CombatState::Recoiled { ref mut timer } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    next_state = Some(CombatState::Idle);
                }
            }
        }

        if let Some(new_state) = next_state {
            *state = new_state;
        }
    }
}

pub fn process_attack_states(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CombatState, &Weapon)>,
) {
    for (entity, mut combat_state, weapon) in query.iter_mut() {
        let mut next_state = None;
        match &mut *combat_state {
            CombatState::Idle => {}
            CombatState::Windup { timer, attack_type } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let attack_def = weapon.attacks.get(attack_type).unwrap();
                    let hitbox_entity = commands
                        .spawn((
                            Hitbox {
                                damage: attack_def.damage,
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
                    commands.entity(entity).add_child(hitbox_entity);

                    next_state = Some(CombatState::Active {
                        timer: Timer::from_seconds(attack_def.active_duration, TimerMode::Once),
                        hitbox_entity: Some(hitbox_entity),
                        attack_type: *attack_type,
                    });
                }
            }
            CombatState::Active { timer, hitbox_entity, attack_type } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if let Some(h_entity) = hitbox_entity {
                        if let Some(mut entity_cmd) = commands.get_entity(*h_entity) {
                            entity_cmd.despawn_recursive();
                        }
                    }
                    let attack_def = weapon.attacks.get(attack_type).unwrap();
                    next_state = Some(CombatState::Recovery {
                        timer: Timer::from_seconds(attack_def.recovery_duration, TimerMode::Once),
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
