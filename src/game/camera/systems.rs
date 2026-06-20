use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::game::common::components::MainCamera;
use crate::game::player::components::Player;

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
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.get_single() else { return };
    let Ok(player_transform) = player_query.get_single() else { return };
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

    // Oyuncunun pozisyonuna offset (mesafe) ekle
    let target_offset = rotation * Vec3::new(0.0, 0.0, cam_state.distance);
    
    // Kameranın hedefini oyuncunun biraz yukarısı (gövdesi/kafası) olarak ayarla
    let look_target = player_transform.translation + Vec3::new(0.0, 1.0, 0.0);
    
    cam_transform.translation = look_target + target_offset;
    cam_transform.look_at(look_target, Vec3::Y);
}
