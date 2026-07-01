use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use crate::game::combat::components::{AttackIntent, CombatState};
use crate::game::lock_on::components::LockOnState;
use crate::game::player::components::Player;
use crate::game::stamina::components::Stamina;
use crate::game::weapon::components::{AttackType, Weapon};

pub fn player_attack_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut CombatState, &Weapon, &AttackIntent), With<Player>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (mut state, weapon, intent) in query.iter_mut() {
            if let CombatState::Idle = *state {
                if let Some(def) = weapon.attacks.get(&intent.current) {
                    *state = CombatState::Windup {
                        timer: Timer::from_seconds(def.windup_duration, TimerMode::Once),
                        attack_type: intent.current,
                    };
                    println!("Mızrak saldırısı tetiklendi: {:?}", intent.current);
                }
            }
        }
    }
}

pub fn update_attack_intent(
    mut motion_evr: EventReader<MouseMotion>,
    lock_on: Res<LockOnState>,
    mut query: Query<&mut AttackIntent, With<Player>>,
) {
    let Ok(mut intent) = query.get_single_mut() else {
        return;
    };

    if lock_on.target.is_none() {
        intent.current = AttackType::Thrust;
        for _ in motion_evr.read() {}
        return;
    }

    let mut total_delta = Vec2::ZERO;
    for ev in motion_evr.read() {
        total_delta += ev.delta;
    }

    let threshold = 5.0;
    if total_delta.length() > threshold {
        if total_delta.x.abs() > total_delta.y.abs() {
            if total_delta.x > 0.0 {
                intent.current = AttackType::LeftSwing;
            } else {
                intent.current = AttackType::RightSwing;
            }
        } else {
            if total_delta.y > 0.0 {
                intent.current = AttackType::Thrust;
            } else {
                intent.current = AttackType::Overhead;
            }
        }
    }
}

pub fn handle_attacks(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut CombatState, &Weapon, &mut Stamina, &AttackIntent), With<Player>>,
) {
    let Ok((mut combat_state, weapon, mut stamina, intent)) = query.get_single_mut() else {
        return;
    };

    if matches!(*combat_state, CombatState::Idle) && mouse.just_pressed(MouseButton::Left) {
        if let Some(attack_def) = weapon.attacks.get(&intent.current) {
            if stamina.try_drain(attack_def.stamina_cost) {
                *combat_state = CombatState::Windup {
                    timer: Timer::from_seconds(attack_def.windup_duration, TimerMode::Once),
                    attack_type: intent.current,
                };
            }
        }
    }
}
