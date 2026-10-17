use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::player::components::{Player, PlayerMovementConfig, PlayerState};
use crate::game::animation::components::AnimationIntent;
use crate::game::common::components::MainCamera;
use crate::game::dodge::components::DodgeState;
use crate::game::lock_on::components::LockOnState;
use crate::game::stamina::components::Stamina;

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<PlayerMovementConfig>,
    lock_on: Res<LockOnState>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
            &mut PlayerState,
            &mut Stamina,
            &mut AnimationIntent,
        ),
        (With<Player>, Without<DodgeState>),
    >,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    target_query: Query<&Transform, (Without<Player>, Without<MainCamera>)>,
    mut log_timer: Local<f32>, // YENİ: Log spam'ini önlemek için lokal zamanlayıcı eklendi
) {
    let Ok((mut player_transform, mut controller, output, mut state, mut stamina, mut intent)) =
        player_query.get_single_mut()
    else {
        return;
    };

    // YENİ: Saniyede 1 kez anlık oyuncu konumunu konsola yazdır
    *log_timer += time.delta_seconds();
    if *log_timer >= 1.0 {
        info!("Anlık Player Konumu: {:?}", player_transform.translation);
        *log_timer = 0.0;
    }

    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    let forward = camera_transform.forward();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = camera_transform.right();
    let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    if keys.pressed(KeyCode::KeyW) { direction += forward_flat; }
    if keys.pressed(KeyCode::KeyS) { direction -= forward_flat; }
    if keys.pressed(KeyCode::KeyD) { direction += right_flat; }
    if keys.pressed(KeyCode::KeyA) { direction -= right_flat; }

    let mut is_moving = false;
    let mut is_sprinting = false;
    let mut speed = config.walk_speed;

    if direction != Vec3::ZERO {
        is_moving = true;
        if keys.pressed(KeyCode::ShiftLeft) {
            let cost = stamina.sprint_cost * time.delta_seconds();
            if stamina.try_drain(cost) {
                speed = config.sprint_speed;
                is_sprinting = true;
            }
        }
    }

    let mut movement = direction.normalize_or_zero() * speed * time.delta_seconds();

    let is_grounded = match output {
        Some(out) => out.grounded,
        None => false,
    };

    intent.is_moving = is_moving;
    intent.is_sprinting = is_sprinting;
    intent.is_in_air = !is_grounded;

    if is_grounded {
        state.vertical_velocity = -0.1;
        if keys.just_pressed(KeyCode::Space) {
            let cost = stamina.jump_cost;
            if stamina.try_drain(cost) {
                state.vertical_velocity = config.jump_velocity;
            }
        }
    } else {
        state.vertical_velocity += config.gravity * time.delta_seconds();
    }

    movement.y = state.vertical_velocity * time.delta_seconds();
    controller.translation = Some(movement);

    if let Some(target_entity) = lock_on.target {
        if let Ok(target_transform) = target_query.get(target_entity) {
            let target_pos = Vec3::new(
                target_transform.translation.x,
                player_transform.translation.y,
                target_transform.translation.z,
            );
            player_transform.look_at(target_pos, Vec3::Y);
        }
    } else if direction != Vec3::ZERO {
        let target_position = player_transform.translation + direction;
        player_transform.look_at(target_position, Vec3::Y);
    }
}