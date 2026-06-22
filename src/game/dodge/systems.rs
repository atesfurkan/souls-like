use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::components::{DodgeConfig, DodgeState};
use crate::game::player::components::Player;
use crate::game::common::components::MainCamera;
use crate::game::stamina::components::Stamina;

pub fn dodge_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    config: Res<DodgeConfig>,
    // Karakter zaten dodge atıyorsa tekrar atamaması için Without<DodgeState> kullanıyoruz
    mut query: Query<(Entity, &mut Stamina, &Transform), (With<Player>, Without<DodgeState>)>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
    // Sol Alt tuşu ile dodge (İsteğe göre değiştirebilirsin)
    if !keys.just_pressed(KeyCode::AltLeft) { return; }

    let Ok((entity, mut stamina, player_transform)) = query.get_single_mut() else { return };
    let Ok(camera_transform) = camera_query.get_single() else { return };

    // Yeterli stamina varsa kes ve işleme başla
    if stamina.try_drain(config.stamina_cost) {
        let mut direction = Vec3::ZERO;

        let forward = camera_transform.forward();
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let right = camera_transform.right();
        let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

        if keys.pressed(KeyCode::KeyW) { direction += forward_flat; }
        if keys.pressed(KeyCode::KeyS) { direction -= forward_flat; }
        if keys.pressed(KeyCode::KeyD) { direction += right_flat; }
        if keys.pressed(KeyCode::KeyA) { direction -= right_flat; }

        // Eğer hiçbir tuşa basılmıyorsa, karakterin mevcut baktığı yöne (ileri) atıl
        if direction == Vec3::ZERO {
            let player_forward = player_transform.forward();
            direction = Vec3::new(player_forward.x, 0.0, player_forward.z).normalize_or_zero();
        } else {
            direction = direction.normalize_or_zero();
        }

        // Oyuncuya Dodge State'i ekle (Dodge işlemini başlat)
        commands.entity(entity).insert(DodgeState {
            direction,
            timer: Timer::from_seconds(config.duration, TimerMode::Once),
        });
    }
}

pub fn dodge_movement(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<DodgeConfig>,
    mut query: Query<(Entity, &mut DodgeState, &mut KinematicCharacterController, &mut Transform)>,
) {
    for (entity, mut state, mut controller, mut transform) in query.iter_mut() {
        state.timer.tick(time.delta());

        if state.timer.finished() {
            // Dodge süresi doldu, State'i sil ve oyuncuyu serbest bırak
            commands.entity(entity).remove::<DodgeState>();
        } else {
            // Dodge süresince karakteri kilitli yöne doğru fırlat
            let mut movement = state.direction * config.speed * time.delta_seconds();
            movement.y = -9.8 * time.delta_seconds(); // Havaya uçmasın diye yerçekimi
            
            controller.translation = Some(movement);

            // Karakter dodge attığı yöne baksın
            let target_pos = transform.translation + state.direction;
            transform.look_at(target_pos, Vec3::Y);
        }
    }
}
