use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct FreeCam {
    pub speed: f32,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 8.0, 15.0),
            ..default()
        },
        FreeCam {
            speed: 25.0,
            sensitivity: 0.002,
            pitch: 0.0,
            yaw: 0.0,
        },
    ));
}

pub fn camera_look(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut ev_mouse: EventReader<MouseMotion>,
    mut q_camera: Query<(&mut Transform, &mut FreeCam)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if q_windows.is_empty() {
        return;
    }

    let Ok((mut transform, mut cam)) = q_camera.get_single_mut() else {
        return;
    };

    // Yalnızca farenin sağ tuşuna basılı tutulduğunda kamerayı döndür
    if mouse_buttons.pressed(MouseButton::Right) {
        let mut delta = Vec2::ZERO;
        for ev in ev_mouse.read() {
            delta += ev.delta;
        }

        if delta != Vec2::ZERO {
            cam.yaw -= delta.x * cam.sensitivity;
            cam.pitch -= delta.y * cam.sensitivity;
            
            // Kameranın ters dönmesini engellemek için dikey açıyı sınırla
            cam.pitch = cam.pitch.clamp(
                -std::f32::consts::FRAC_PI_2 + 0.01,
                std::f32::consts::FRAC_PI_2 - 0.01,
            );

            // Yeni rotasyonu Quaternion olarak hesapla ve uygula
            transform.rotation = Quat::from_axis_angle(Vec3::Y, cam.yaw)
                * Quat::from_axis_angle(Vec3::X, cam.pitch);
        }
    } else {
        // Fare olaylarını tüketerek birikmesini önle
        ev_mouse.clear(); 
    }
}

pub fn camera_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut q_camera: Query<(&mut Transform, &FreeCam)>,
) {
    let Ok((mut transform, cam)) = q_camera.get_single_mut() else {
        return;
    };

    let mut velocity = Vec3::ZERO;
    
    // Rotasyona göre yerel yön vektörleri
    let forward = transform.rotation * Vec3::NEG_Z;
    let right = transform.rotation * Vec3::X;
    let up = Vec3::Y;

    if keys.pressed(KeyCode::KeyW) {
        velocity += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        velocity -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        velocity += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        velocity -= right;
    }
    if keys.pressed(KeyCode::Space) {
        velocity += up;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        velocity -= up;
    }

    if velocity.length_squared() > 0.0 {
        // Hızlı uçuş için sol kontrol tuşuna basılı tut
        let speed_multiplier = if keys.pressed(KeyCode::ControlLeft) {
            3.0
        } else {
            1.0
        };
        
        velocity = velocity.normalize();
        let translation = velocity * cam.speed * speed_multiplier * time.delta_seconds();
        transform.translation += translation;
    }
}
