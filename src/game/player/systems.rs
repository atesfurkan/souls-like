use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::common::components::MainCamera;
use crate::game::stamina::components::Stamina;
use crate::game::dodge::components::DodgeState;
use crate::game::combat::components::{CombatState, Weapon};
// Yeni bileşenleri import et
use super::components::{Player, PlayerState, PlayerMovementConfig}; 


pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Materyalleri hazırlayalım
    let body_material = materials.add(Color::srgb(0.2, 0.5, 0.8));
    let skin_material = materials.add(Color::srgb(0.9, 0.7, 0.6));
    let weapon_wood = materials.add(Color::srgb(0.4, 0.2, 0.1));
    let weapon_metal = materials.add(Color::srgb(0.8, 0.8, 0.8));

    // Ana oyuncu kapsülü (Ebeveyn / Parent) 
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d { radius: 0.5, half_length: 0.5 }), // DÜZELTİLDİ
            material: body_material.clone(),
            transform: Transform::from_xyz(0.0, 3.0, 0.0), // Havadan düşme ayarı 
            ..default()
        },
        Player,
        PlayerState::default(),
        CombatState::default(), // YENİ: Başlangıçta Idle
        // YENİ: Mızrak istatistikleri
        Weapon {
            windup_duration: 0.3,
            active_duration: 0.2,
            recovery_duration: 0.5,
            stamina_cost: 20.0,
            damage: 25.0, // Bir vuruşta verilecek hasar
            hitbox_radius: 0.4, // Vuruş küresinin büyüklüğü
            // Karakterin merkezine göre önünde, biraz yukarıda bir alanda hasar oluşturacak
            hitbox_offset: Vec3::new(0.0, 1.0, -1.2), 
        },
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.5, 0.5), 
        KinematicCharacterController::default(),
        Stamina::default(),
    ))
    .with_children(|parent| {
        // --- ANATOMİ ---
        
        // Kafa (Sphere)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Sphere { radius: 0.35 }), // DÜZELTİLDİ
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.0, 0.9, 0.0), // Kapsülün tam üstünde
            ..default()
        });

        // Sol Kol (Cylinder)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.12, half_height: 0.35 }), // DÜZELTİLDİ (half_height kullanır)
            material: skin_material.clone(),
            transform: Transform::from_xyz(-0.6, 0.2, 0.0),
            ..default()
        });

        // Sağ Kol (Cylinder)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.12, half_height: 0.35 }), // DÜZELTİLDİ
            material: skin_material.clone(),
            transform: Transform::from_xyz(0.6, 0.2, 0.0),
            ..default()
        });

        // Sol Bacak (Cylinder)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.15, half_height: 0.4 }), // DÜZELTİLDİ
            material: body_material.clone(),
            transform: Transform::from_xyz(-0.25, -0.9, 0.0),
            ..default()
        });

        // Sağ Bacak (Cylinder)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.15, half_height: 0.4 }), // DÜZELTİLDİ
            material: body_material.clone(),
            transform: Transform::from_xyz(0.25, -0.9, 0.0),
            ..default()
        });

        // Yön Göstergesi (Çocuk / Child) (Göz niyetine tutulabilir veya silinebilir, şimdilik koruyoruz) [cite: 9]
        
        // --- SİLAH (SPEAR) ---
        // Silah ebeveyni (Sağ elde tutuluyor gibi)
        parent.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.6, 0.0, -0.5)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
                ..default()
            },
            // YENİ: Silah ebeveynine işaretçimizi ekliyoruz
            crate::game::combat::components::EquippedWeapon, 
        )).with_children(|weapon_parent| {
            // Mızrak Sapı (Long cylinder shaft)
            weapon_parent.spawn(PbrBundle {
                mesh: meshes.add(Cylinder { radius: 0.05, half_height: 1.0 }), // Toplam boy 2.0 olur
                material: weapon_wood,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });

            // Mızrak Ucu (Small cone spear tip)
            weapon_parent.spawn(PbrBundle {
                mesh: meshes.add(Cone { radius: 0.1, height: 0.4 }), // DÜZELTİLDİ: Doğrudan struct alanları verildi
                material: weapon_metal,
                transform: Transform::from_xyz(0.0, 1.2, 0.0), // Sapın hemen üstünde
                ..default()
            });
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
        &mut Stamina,
    ), (With<Player>, Without<DodgeState>)>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
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

    if direction != Vec3::ZERO {
        let target_position = player_transform.translation + direction;
        player_transform.look_at(target_position, Vec3::Y);
    }
}
