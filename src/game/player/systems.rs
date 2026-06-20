use bevy::prelude::*;
use super::components::Player;
use crate::game::common::components::MainCamera;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
            material: materials.add(Color::srgb(0.2, 0.5, 0.8)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Player,
    ));
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else { return };
    let Ok(camera_transform) = camera_query.get_single() else { return };

    let mut direction = Vec3::ZERO;

    // Kameranın baktığı yönü hesapla ve Y eksenini (yukarı/aşağı) yoksay
    let forward = camera_transform.forward();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = camera_transform.right();
    let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    if keys.pressed(KeyCode::KeyW) { direction += forward_flat; }
    if keys.pressed(KeyCode::KeyS) { direction -= forward_flat; }
    if keys.pressed(KeyCode::KeyD) { direction += right_flat; }
    if keys.pressed(KeyCode::KeyA) { direction -= right_flat; }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        let speed = 5.0;
        player_transform.translation += direction * speed * time.delta_seconds();
        
        // Oyuncunun yüzünü yürüdüğü yöne çevir
        let target_position = player_transform.translation + direction;
        player_transform.look_at(target_position, Vec3::Y);
    }
}
