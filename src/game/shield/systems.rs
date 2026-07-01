use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use super::components::{BlockingState, EquippedShield, Shield};

// 1. Kalkan Durum Makinesi (State Machine)
// Sağ tıka basıldığında Parry penceresini başlatır, süre dolunca normal Block durumuna geçer.
pub fn handle_shield_input(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut BlockingState, &Shield)>,
) {
    for (mut state, shield) in query.iter_mut() {
        if mouse.just_pressed(MouseButton::Right) {
            // Sadece Idle durumundayken kalkan kaldırılabilir
            if matches!(*state, BlockingState::Idle) {
                *state = BlockingState::Parrying {
                    timer: Timer::from_seconds(shield.parry_window, TimerMode::Once),
                };
            }
        } else if mouse.pressed(MouseButton::Right) {
            // Sağ tıka basılı tutuluyorsa Parry zamanlayıcısını ilerlet
            if let BlockingState::Parrying { timer } = &mut *state {
                timer.tick(time.delta());
                
                // Parry süresi dolduysa normal bloklamaya geç
                if timer.just_finished() {
                    *state = BlockingState::Blocking;
                    println!("Parry süresi bitti, normal Block durumuna geçildi.");
                }
            }
        } else if mouse.just_released(MouseButton::Right) {
            // Sağ tık bırakıldığında kalkanı indir (GuardBroken değilse)
            if matches!(*state, BlockingState::Parrying { .. } | BlockingState::Blocking) {
                *state = BlockingState::Idle;
            }
        }
    }
}


