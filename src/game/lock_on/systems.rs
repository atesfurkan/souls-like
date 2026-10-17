use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::{LockOnState, Lockable};
use crate::game::player::components::Player;
use crate::game::health::components::Health;
use crate::game::common::components::MainCamera;

// MMB (Orta Fare Tuşu) ile kilitlenmeyi açıp kapatan sistem
pub fn toggle_lock_on(
    mouse: Res<ButtonInput<MouseButton>>,
    mut lock_on: ResMut<LockOnState>,
    player_query: Query<&Transform, With<Player>>,
    lockable_query: Query<(Entity, &Transform, &Health), With<Lockable>>,
) {
    if mouse.just_pressed(MouseButton::Middle) {
        if lock_on.target.is_some() {
            lock_on.target = None;
            println!("Lock-on Disabled");
        } else {
            let Ok(player_transform) = player_query.get_single() else { return };
            let player_pos = player_transform.translation;
            
            let mut closest_dist = f32::MAX;
            let mut closest_enemy = None;

            for (entity, transform, health) in lockable_query.iter() {
                if health.current > 0.0 {
                    let dist = player_pos.distance(transform.translation);
                    if dist < 20.0 && dist < closest_dist {
                        closest_dist = dist;
                        closest_enemy = Some(entity);
                    }
                }
            }
            
            lock_on.target = closest_enemy;
            if closest_enemy.is_some() {
                println!("Lock-on Enabled");
            }
        }
    }
}

// Hedef öldüğünde (veya dünyadan silindiğinde) kilidi otomatik kaldıran sistem
pub fn validate_lock_on(
    mut lock_on: ResMut<LockOnState>,
    query: Query<&Health>,
) {
    if let Some(target) = lock_on.target {
        if let Ok(health) = query.get(target) {
            if health.current <= 0.0 {
                lock_on.target = None;
                println!("Target Dead -> Lock-on Disabled");
            }
        } else {
            lock_on.target = None;
        }
    }
}

pub fn lock_on_camera_focus(
    lock_on: Res<LockOnState>,
    time: Res<Time>, 
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    target_query: Query<&Transform, (Without<MainCamera>, Without<Player>)>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<MainCamera>)>, 
    rapier_context: Res<RapierContext>, 
) {
    if let Some(target_entity) = lock_on.target {
        let Ok(target_transform) = target_query.get(target_entity) else { return };
        let Ok((player_entity, player_transform)) = player_query.get_single() else { return };
        let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };

        let player_pos = player_transform.translation;
        let target_pos = target_transform.translation;

        // Gerçek mesafeyi (Yüksekliği de hesaba katarak) bul
        let dir_to_target = target_pos - player_pos;
        let distance_to_target = dir_to_target.length();
        
        // Y eksenini sıfırlayarak sadece yatay yönelimleri bul
        let mut dir_2d = dir_to_target;
        dir_2d.y = 0.0;
        let forward = dir_2d.normalize_or_zero();
        let right = Vec3::new(-forward.z, 0.0, forward.x); 

        // --- DİNAMİK KAMERA DEĞERLERİ ---
        // Hedef uzaklaştıkça kameranın temel mesafesini (4.0) orantılı olarak artırır (Maksimum 8.0'a kadar)
        let base_distance = 4.0;
        let dynamic_distance = (base_distance + (distance_to_target * 0.25)).clamp(4.0, 8.0);
        
        let height_offset = 2.5;
        let side_offset = 1.5;
        let camera_speed = 5.0;

        // 3. Kameranın olması GEREKEN ideal konumu hesapla (Dinamik mesafeyi kullanarak)
        let desired_camera_pos = player_pos 
            - (forward * dynamic_distance) 
            + (Vec3::Y * height_offset) 
            + (right * side_offset);

        // 4. RAYCAST - Duvar Kontrolü
        let ray_origin = player_pos + Vec3::Y * 1.0;
        let ray_vector = desired_camera_pos - ray_origin;
        let max_distance = ray_vector.length();
        let ray_direction = ray_vector.normalize_or_zero();

        let filter = QueryFilter::new()
            .exclude_collider(player_entity)
            .exclude_collider(target_entity);

        let final_camera_pos = if max_distance > 0.0 {
            if let Some((_hit_entity, toi)) = rapier_context.cast_ray(
                ray_origin,
                ray_direction,
                max_distance,
                true,
                filter
            ) {
                ray_origin + ray_direction * (toi - 0.2).max(0.5)
            } else {
                desired_camera_pos
            }
        } else {
            desired_camera_pos
        };

        // 5. Kamerayı ideal konuma kaydır
        camera_transform.translation = camera_transform.translation.lerp(final_camera_pos, time.delta_seconds() * camera_speed);
        
        // 6. AĞIRLIKLI BAKIŞ NOKTASI (Weighted Look-Target)
        // Kamera sadece düşmana bakarsa yüksek düşmanlarda oyuncu ekrandan çıkar.
        // Bu yüzden oyuncunun kafası ile düşmanın gövdesi arasında bir orta noktaya bakacağız.
        let player_head = player_pos + Vec3::Y * 1.5;
        let target_center = target_pos + Vec3::Y * 1.0;
        
        // Hedefe %70, oyuncuya %30 odaklan (lerp değeri 0.0 oyuncu, 1.0 hedef demektir)
        let look_target = player_head.lerp(target_center, 0.7);

        camera_transform.look_at(look_target, Vec3::Y);
    }
}