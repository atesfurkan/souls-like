use bevy::prelude::*;
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
        // Eğer zaten kilitliysek, kilidi kaldır
        if lock_on.target.is_some() {
            lock_on.target = None;
            println!("Lock-on Disabled");
        } else {
            // Değilsek en yakın düşmanı bul
            let Ok(player_transform) = player_query.get_single() else { return };
            let player_pos = player_transform.translation;
            
            let mut closest_dist = f32::MAX;
            let mut closest_enemy = None;
            
            for (entity, transform, health) in lockable_query.iter() {
                if health.current > 0.0 {
                    let dist = player_pos.distance(transform.translation);
                    // Sadece 20 birim yakındaki düşmanlara kilitlenilebilir
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
            // Canı 0'a düştüyse kilidi kır
            if health.current <= 0.0 {
                lock_on.target = None;
                println!("Target Dead -> Lock-on Disabled");
            }
        } else {
            // Entity bir şekilde dünyadan tamamen silindiyse
            lock_on.target = None;
        }
    }
}

pub fn lock_on_camera_focus(
    lock_on: Res<LockOnState>,
    time: Res<Time>, // Yumuşak geçiş (lerp) için eklendi
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    target_query: Query<&Transform, (Without<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>, // Oyuncuyu bulmak için eklendi
) {
    if let Some(target_entity) = lock_on.target {
        let Ok(target_transform) = target_query.get(target_entity) else { return };
        let Ok(player_transform) = player_query.get_single() else { return };
        let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };
        
        let player_pos = player_transform.translation;
        let target_pos = target_transform.translation;

        // 1. Oyuncudan hedefe doğru olan yönü bul (Sadece X ve Z ekseninde)
        let mut dir_to_target = target_pos - player_pos;
        dir_to_target.y = 0.0;
        let forward = dir_to_target.normalize_or_zero();
        
        // 2. Kamerayı sağa veya sola kaydırmak için "Sağ" (Right) vektörünü hesapla
        // İleride bunu ayarlardan okuyup sola almak istersen: Vec3::new(forward.z, 0.0, -forward.x) yapabilirsin.
        let right = Vec3::new(-forward.z, 0.0, forward.x); 

        // --- KENDİ ZEVKİNE GÖRE DEĞİŞTİREBİLECEĞİN DEĞERLER ---
        let height_offset = 2.5;     // Kameranın yüksekliği (Oyuncunun tepesinden ne kadar yukarıda)
        let distance_behind = 4.0;   // Oyuncunun ne kadar gerisinde duracak
        let side_offset = 1.5;       // Sağ omuz boşluğu (Karakteri görmeni sağlar)
        let camera_speed = 5.0;      // Kameranın hedefe süzülme hızı (Glitchleri önler)

        // 3. Kameranın olması GEREKEN konumu hesapla
        let desired_camera_pos = player_pos 
            - (forward * distance_behind) 
            + (Vec3::Y * height_offset) 
            + (right * side_offset);

        // 4. Kamerayı anında ışınlamak yerine, olduğu yerden olması gereken yere yumuşakça kaydır
        camera_transform.translation = camera_transform.translation.lerp(desired_camera_pos, time.delta_seconds() * camera_speed);

        // 5. Kameranın yüzünü düşmanın gövdesine doğru çevir
        let look_target = target_pos + Vec3::Y * 1.0;
        camera_transform.look_at(look_target, Vec3::Y);
    }
}
