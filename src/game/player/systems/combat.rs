use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use crate::game::player::components::{Player, PlayerCombatConfig, PlayerInventory};
use crate::game::combat::components::{CombatState, GuardIntent};
use crate::game::lock_on::components::LockOnState;
use crate::game::shield::components::BlockingState;
use crate::game::shield::data::ShieldType;
use crate::game::shield::events::ShieldSwapRequestEvent;
use crate::game::weapon::components::Weapon;
use crate::game::weapon::events::WeaponSwapRequestEvent;

pub fn update_guard_intent(
    mut mouse_motion: EventReader<MouseMotion>,
    lock_on: Res<LockOnState>,
    config: Res<PlayerCombatConfig>,
    mut query: Query<(&mut GuardIntent, &BlockingState), With<Player>>,
) {
    if lock_on.target.is_none() {
        return;
    }

    for (mut intent, state) in query.iter_mut() {
        if matches!(state, BlockingState::Blocking) {
            for motion in mouse_motion.read() {
                intent.offset += motion.delta * config.guard_sensitivity;
                intent.offset = intent.offset.clamp(
                    Vec2::splat(-config.guard_max_offset),
                    Vec2::splat(config.guard_max_offset),
                );
            }
        } else {
            intent.offset = intent.offset.lerp(Vec2::ZERO, 0.1);
        }
    }
}

pub fn handle_weapon_swap_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_events: EventWriter<WeaponSwapRequestEvent>,
    player_query: Query<(Entity, &CombatState, Option<&Weapon>, &PlayerInventory), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        if let Ok((entity, combat_state, current_weapon_opt, inventory)) = player_query.get_single() {
            if *combat_state == CombatState::Idle && !inventory.weapons.is_empty() {
                // O anki silahı (varsa) tespit et
                let current_type = current_weapon_opt.map(|w| w.weapon_type);
                
                let current_idx = inventory
                    .weapons
                    .iter()
                    .position(|w| *w == current_type)
                    .unwrap_or(0);
                
                // Bir sonrakine geç (None yani silahsız duruma da geçer)
                let next_idx = (current_idx + 1) % inventory.weapons.len();
                let next_weapon = inventory.weapons[next_idx];

                swap_events.send(WeaponSwapRequestEvent {
                    entity,
                    target_weapon: next_weapon,
                });
            }
        }
    }
}

pub fn handle_shield_swap_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_events: EventWriter<ShieldSwapRequestEvent>,
    player_query: Query<(Entity, &CombatState, Option<&ShieldType>, &PlayerInventory), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        if let Ok((entity, combat_state, current_shield_type_opt, inventory)) = player_query.get_single() {
            if *combat_state == CombatState::Idle && !inventory.shields.is_empty() {
                let current_type = current_shield_type_opt.copied();
                
                let current_idx = inventory
                    .shields
                    .iter()
                    .position(|s| *s == current_type)
                    .unwrap_or(0);
                
                // Bir sonrakine geç (None yani kalkansız duruma da geçer)
                let next_idx = (current_idx + 1) % inventory.shields.len();
                let next_shield = inventory.shields[next_idx];

                swap_events.send(ShieldSwapRequestEvent {
                    entity,
                    target_shield: next_shield,
                });
            }
        }
    }
}