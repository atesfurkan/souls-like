use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::common::components::MainCamera;

// Yeni bileşenleri import et
use super::components::{Player, PlayerState, PlayerMovementConfig}; 


//use super::components::Player; 
//use super::components::PlayerState; 
//use super::components::PlayerMovementConfig; 



pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ana oyuncu kapsülü (Ebeveyn / Parent)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
            material: materials.add(Color::srgb(0.2, 0.5, 0.8)),
            transform: Transform::from_xyz(0.0, 3.0, 0.0), // Havadan düşme ayarımız duruyor
            ..default()
        },

        Player,
        PlayerState::default(),
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.5, 0.5), 
        KinematicCharacterController::default(),
    ))
    .with_children(|parent| {
        // Yön Göstergesi (Çocuk / Child)
        parent.spawn(PbrBundle {
            // Küçük, uzun ince bir kutu
            mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.6)),
            // Dikkat çekici kırmızı bir renk
            material: materials.add(Color::srgb(1.0, 0.2, 0.2)),
            // X: Ortada (0.0)
            // Y: Kapsülün üst kısmına doğru (0.4)
            // Z: İleriye doğru burnu çıksın diye eksi Z yönünde (-0.6)
            transform: Transform::from_xyz(0.0, 0.4, -0.6), 
            ..default()
        });
    });
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<PlayerMovementConfig>, // EKLENDİ: Magic number'lar yerine Config
    mut player_query: Query<(
        &mut Transform, 
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>, // EKLENDİ: Yere değme kontrolü
        &mut PlayerState, // EKLENDİ: Dikey hız hafızası
    ), With<Player>>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok((mut player_transform, mut controller, output, mut state)) = player_query.get_single_mut() else { return };
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
    let speed = if keys.pressed(KeyCode::ShiftLeft) {
        config.sprint_speed
    } else {
        config.walk_speed
    };

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
            state.vertical_velocity = config.jump_velocity;
        }
    } else {
        // Havadaysak yerçekimini dikey hıza uygula (ivmelenme)
        state.vertical_velocity += config.gravity * time.delta_seconds();
    }

    // Dikey hızı (Y ekseni) asıl harekete dahil et
    movement.y = state.vertical_velocity * time.delta_seconds();

    controller.translation = Some(movement);

    if direction != Vec3::ZERO {
        let target_position = player_transform.translation + direction;
        player_transform.look_at(target_position, Vec3::Y);
    }
}
