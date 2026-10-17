use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::game::common::components::MainCamera;
use crate::game::player::components::Player;
use crate::game::lock_on::components::LockOnState;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        MainCamera {
            distance: 6.0,
            pitch: 0.3, // Hafifçe yukarıdan bak
            yaw: 0.0,
        },
    ));
}

pub fn cursor_management(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut window) = windows.get_single_mut() else { return };

    // Ekrana tıklayınca fareyi kilitle
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
    // ESC'ye basınca fareyi serbest bırak
    if keys.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

pub fn camera_controls(
    mut ev_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<MainCamera>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    lock_on: Res<LockOnState>,
    rapier_context: Res<RapierContext>, // Raycast için fizik bağlamı eklendi
) {
    if lock_on.target.is_some() {
        return;
    }
    
    let Ok(window) = windows.get_single() else { return };
    let Ok((player_entity, player_transform)) = player_query.get_single() else { return };
    let Ok((mut cam_transform, mut cam_state)) = camera_query.get_single_mut() else { return };

    // Sadece fare kilitliyken kamerayı döndür
    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let mut mouse_delta = Vec2::ZERO;
    for ev in ev_motion.read() {
        mouse_delta += ev.delta;
    }

    let sensitivity = 0.003;
    cam_state.yaw -= mouse_delta.x * sensitivity;
    cam_state.pitch -= mouse_delta.y * sensitivity;

    // Pitch (yukarı/aşağı) açısını sınırla, kameranın ters dönmesini engelle
    cam_state.pitch = cam_state.pitch.clamp(-1.0, 1.5);

    // Yeni dönüş rotasyonunu hesapla (Önce Yaw, sonra Pitch)
    let rotation = Quat::from_axis_angle(Vec3::Y, cam_state.yaw)
        * Quat::from_axis_angle(Vec3::X, cam_state.pitch);

    // Kameranın hedefini oyuncunun biraz yukarısı (gövdesi/kafası) olarak ayarla
    let look_target = player_transform.translation + Vec3::new(0.0, 1.0, 0.0);
    
    // Engeller olmasaydı kameranın durması gereken ideal mesafe ve yön
    let ideal_offset = rotation * Vec3::new(0.0, 0.0, cam_state.distance);
    let ray_direction = ideal_offset.normalize();
    let max_distance = cam_state.distance;

    // Raycast işlemi sırasında ışın oyuncunun içinden çıkacağı için oyuncu collider'ını görmezden geliyoruz
    let filter = QueryFilter::new().exclude_collider(player_entity);

    // Kameranın hedefine doğru ray atarak engelleri kontrol et
    let final_distance = if let Some((_hit_entity, toi)) = rapier_context.cast_ray(
        look_target,
        ray_direction,
        max_distance,
        true, // solid = true (Kamera doğrudan collider'ların içine girmemeli)
        filter
    ) {
        // Çarpışma tespit edildi (kamera duvarın arkasına geçiyor).
        // Kameranın duvar yüzeyine yapışıp içine girmemesi için mesafeyi 0.2 birim kısalt.
        // Karakterin modelinin içine tamamen girmemesi için de minimum 0.5 mesafe tut.
        (toi - 0.2).max(0.5)
    } else {
        // Çarpışma yok, ideal uzaklığı kullan
        max_distance
    };

    // Bulunan güvenli mesafeye göre asıl ofseti uygula
    let actual_offset = ray_direction * final_distance;
    
    cam_transform.translation = look_target + actual_offset;
    cam_transform.look_at(look_target, Vec3::Y);
}