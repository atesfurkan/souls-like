// Dosya: src/game/combat/systems/attack.rs

use crate::game::combat::components::{AttackIntent, CombatInput, CombatState, UnarmedConfig};
use crate::game::lock_on::components::LockOnState;
use crate::game::player::components::Player;
use crate::game::stamina::components::Stamina;
use crate::game::weapon::components::{AttackType, Weapon};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub fn update_combat_input(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut CombatInput, With<Player>>,
) {
    if let Ok(mut input) = query.get_single_mut() {
        // Sağ tık blok/kalkan için ayrıldığından, sadece sol tık saldırı niyetini tutar.
        input.attack_held = mouse.pressed(MouseButton::Left);
    }
}

pub fn player_attack_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    lock_on: Res<LockOnState>,
    mut query: Query<(&mut CombatState, Option<&Weapon>, Option<&UnarmedConfig>, &mut AttackIntent), With<Player>>,
) {
    for (mut state, weapon, unarmed, mut intent) in query.iter_mut() {
        if let CombatState::Idle = *state {
            let mut triggered = false;

            if mouse_input.just_pressed(MouseButton::Left) {
                if weapon.is_some() {
                    triggered = true;
                } else if unarmed.is_some() {
                    if lock_on.target.is_none() {
                        // Lock-On yokken yumruklar sırayla değişir (Alternating)
                        if intent.current == AttackType::UnarmedRight {
                            intent.current = AttackType::UnarmedLeft;
                        } else {
                            intent.current = AttackType::UnarmedRight;
                        }
                    } else {
                        // Lock-On varken varsayılan davranış
                        intent.current = AttackType::UnarmedLeft;
                    }
                    triggered = true;
                }
            }

            if triggered {
                let mut def_opt = None;
                if let Some(w) = weapon { 
                    def_opt = w.attacks.get(&intent.current); 
                }
                if def_opt.is_none() {
                    if let Some(u) = unarmed { 
                        def_opt = u.attacks.get(&intent.current); 
                    }
                }

                if let Some(def) = def_opt {
                    *state = CombatState::Windup {
                        timer: Timer::from_seconds(def.windup_duration, TimerMode::Once),
                        charge_timer: Timer::from_seconds(def.max_charge_duration, TimerMode::Once),
                        attack_type: intent.current,
                    };
                }
            }
        }
    }
}

pub fn update_attack_intent(
    mut motion_evr: EventReader<MouseMotion>,
    lock_on: Res<LockOnState>,
    mut query: Query<(&mut AttackIntent, Option<&Weapon>), With<Player>>,
) {
    let Ok((mut intent, weapon)) = query.get_single_mut() else {
        return;
    };

    // Silahsızsa motion intent tetiklemesini atlıyoruz, doğrudan sol input ile belirlenecek.
    if weapon.is_none() {
        for _ in motion_evr.read() {}
        return;
    }

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
    lock_on: Res<LockOnState>,
    mut query: Query<(&mut CombatState, Option<&Weapon>, Option<&UnarmedConfig>, &mut Stamina, &mut AttackIntent), With<Player>>,
) {
    let Ok((mut combat_state, weapon, unarmed, mut stamina, mut intent)) = query.get_single_mut() else {
        return;
    };

    if matches!(*combat_state, CombatState::Idle) {
        let mut triggered = false;

        if mouse.just_pressed(MouseButton::Left) {
            if weapon.is_some() {
                triggered = true;
            } else if unarmed.is_some() {
                if lock_on.target.is_none() {
                    // Lock-On yokken yumruklar sırayla değişir (Alternating)
                    if intent.current == AttackType::UnarmedRight {
                        intent.current = AttackType::UnarmedLeft;
                    } else {
                        intent.current = AttackType::UnarmedRight;
                    }
                } else {
                    // Lock-On varken varsayılan davranış
                    intent.current = AttackType::UnarmedLeft;
                }
                triggered = true;
            }
        }

        if triggered {
            let mut attack_def = None;
            if let Some(w) = weapon { 
                attack_def = w.attacks.get(&intent.current); 
            }
            if attack_def.is_none() {
                if let Some(u) = unarmed { 
                    attack_def = u.attacks.get(&intent.current); 
                }
            }

            if let Some(def) = attack_def {
                if stamina.try_drain(def.stamina_cost) {
                    *combat_state = CombatState::Windup {
                        timer: Timer::from_seconds(def.windup_duration, TimerMode::Once),
                        charge_timer: Timer::from_seconds(def.max_charge_duration, TimerMode::Once),
                        attack_type: intent.current,
                    };
                }
            }
        }
    }
}