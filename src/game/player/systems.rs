use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::common::components::MainCamera;
use crate::game::stamina::components::Stamina;
use crate::game::dodge::components::DodgeState;
use crate::game::combat::components::{CombatState, Weapon,
     AttackDefinition, AttackType, AttackIntent};
use crate::game::lock_on::components::LockOnState;

use bevy::utils::HashMap;

use super::components::{Player, PlayerState, PlayerMovementConfig}; 


pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<PlayerMovementConfig>, 
    lock_on: Res<LockOnState>,
    mut player_query: Query<(
        &mut Transform, 
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>, //Yere değme kontrolü
        &mut PlayerState, //  Dikey hız hafızası
        &mut Stamina,
    ), (With<Player>, Without<DodgeState>)>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    target_query: Query<&Transform, (Without<Player>, Without<MainCamera>)>, //  (Hedefin pozisyonunu almak için)
) {
    let Ok((mut player_transform, mut controller, output, mut state, mut stamina)) = player_query.get_single_mut() else { return };
    let Ok(camera_transform) = camera_query.get_single() else { return };

    let mut direction = Vec3::ZERO;

    let forward = camera_transform.forward();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = camera_transform.right();
    let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    if keys.pressed(KeyCode::KeyW) { direction += forward_flat; }
    if keys.pressed(KeyCode::KeyS) { direction -= forward_flat; }
    if keys.pressed(KeyCode::KeyD) { direction += right_flat; }
    if keys.pressed(KeyCode::KeyA) { direction -= right_flat; }

    // Sprint Mantığı: Shift'e basılıysa sprint hızını, değilse yürüme hızını al
    // --- SPRINT MANTIĞI GÜNCELLENDİ ---
    let mut speed = config.walk_speed;
    
    // Karakter hareket ediyorsa ve Shift'e basılıysa
    if keys.pressed(KeyCode::ShiftLeft) && direction != Vec3::ZERO {
        let cost = stamina.sprint_cost * time.delta_seconds();
        
        // Staminada yer varsa sprint yap ve staminayı tüket
        if stamina.try_drain(cost) {
            speed = config.sprint_speed;
        }
    }
    let mut movement = direction.normalize_or_zero() * speed * time.delta_seconds();

    // Zıplama ve Yerçekimi Mantığı
    // Rapier çıktısı henüz oluşmadıysa (ilk frame) false kabul et
    let is_grounded = match output {
        Some(out) => out.grounded,
        None => false,
    };

    if is_grounded {
        // Yerdeysek hafif eksi hız vererek zemine yapışık kalmasını sağla
        state.vertical_velocity = -0.1; 
        
        // Sadece yerdeyken Space tuşuna basıldığında zıpla
        if keys.just_pressed(KeyCode::Space) {
            let cost = stamina.jump_cost ;
            state.vertical_velocity = config.jump_velocity;
            stamina.try_drain(cost);
        }
    } else {
        // Havadaysak yerçekimini dikey hıza uygula (ivmelenme)
        state.vertical_velocity += config.gravity * time.delta_seconds();
    }
    // Dikey hızı (Y ekseni) asıl harekete dahil et
    movement.y = state.vertical_velocity * time.delta_seconds();
    controller.translation = Some(movement);

    // --- GÜNCELLENEN ROTASYON MANTIĞI (STRAFING) ---
    // Eğer lock-on aktifse ve hedef dünyadaysa
    if let Some(target_entity) = lock_on.target {
        if let Ok(target_transform) = target_query.get(target_entity) {
            // Karakterin Y (yukarı/aşağı) eğilmesini engellemek için sadece X ve Z eksenine bakıyoruz
            let target_pos = Vec3::new(
                target_transform.translation.x, 
                player_transform.translation.y, 
                target_transform.translation.z
            );
            player_transform.look_at(target_pos, Vec3::Y);
        }
    } else {
        // Lock-on yoksa serbest hareket (Gidilen yöne dön)
        if direction != Vec3::ZERO {
            let target_position = player_transform.translation + direction;
            player_transform.look_at(target_position, Vec3::Y);
        }
    }
}
