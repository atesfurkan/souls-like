use bevy::prelude::*;
use std::time::Duration;

// --- OLAYLAR (EVENTS) ---
// Olaylar (Events), sistemler arası iletişim kurmak için kullanılır.
#[derive(Event)]
struct EnemyDeathEvent(Entity);

// --- KAYNAKLAR (RESOURCES) ---
// Kaynaklar (Resources), sahnede bir tane olması gereken global verilerdir.
#[derive(Resource)]
struct EnemyShootTimer(Timer);

// --- BİLEŞENLER (COMPONENTS) ---
// Bileşenler (Components), objelerin (Entity) özelliklerini tanımlar.

#[derive(Component)]
struct Player {
    facing_direction: Vec3, // Oyuncunun en son hareket ettiği ve baktığı yön
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Projectile {
    direction: Vec3,
    speed: f32,
    is_from_player: bool, // Merminin kimden çıktığını bilmek için
}

// Çarpışma tespiti için kullanacağımız basit bir küresel çarpıştırıcı
#[derive(Component)]
struct Collider {
    radius: f32,
}

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Kaynakları başlat
        .insert_resource(EnemyShootTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)))
        // Olayları kaydet
        .add_event::<EnemyDeathEvent>()
        // Sistemleri ekle
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (
            player_movement,
            camera_follow,
            enemy_ai_movement,
            player_shoot,
            enemy_shoot,
            projectile_movement,
            player_enemy_collision_system, // Yeni ayrılmış sistem 1
            projectile_collision_system,   // Yeni ayrılmış sistem 2
            handle_enemy_death,
        ))
        .run();
}

// --- SİSTEMLER (SYSTEMS) ---

// 1. Sadece Oyuncu ve Düşman etkileşimleri
// 1. Sistem: Sadece Oyuncu-Düşman
fn player_enemy_collision_system(
    mut player_query: Query<(&mut Transform, &Collider), With<Player>>,
    enemy_query: Query<(&Transform, &Collider), With<Enemy>>,
) {
    let (mut p_trans, p_coll) = player_query.single_mut();
    
    for (e_trans, e_coll) in enemy_query.iter() {
        if p_trans.translation.distance(e_trans.translation) < (p_coll.radius + e_coll.radius) {
            let push = (p_trans.translation - e_trans.translation).normalize_or_zero();
            p_trans.translation += push * 0.2;
        }
    }
}

// 2. Sistem: Sadece Mermi
fn projectile_collision_system(
    mut commands: Commands,
    proj_query: Query<(Entity, &Transform, &Projectile)>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut ev_death: EventWriter<EnemyDeathEvent>,
) {
    for (p_ent, p_trans, proj) in proj_query.iter() {
        if proj.is_from_player {
            for (e_ent, e_trans) in enemy_query.iter() {
                if p_trans.translation.distance(e_trans.translation) < 0.5 {
                    ev_death.send(EnemyDeathEvent(e_ent));
                    commands.entity(p_ent).despawn();
                }
            }
        }
    }
}


/// Sahneyi, kamerayı, oyuncuyu ve düşmanları oluşturur
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Zemin (Plane)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(15.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        }),
        ..default()
    });

    // 2. Duvarlar (Kutular)
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5),
        ..default()
    });
    let wall_mesh = meshes.add(Cuboid::new(2.0, 2.0, 2.0));

    let wall_positions = [
        Vec3::new(10.0, 1.0, 0.0),
        Vec3::new(-10.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 10.0),
        Vec3::new(0.0, 1.0, -10.0),
    ];

    for pos in wall_positions {
        commands.spawn((
            PbrBundle {
                mesh: wall_mesh.clone(),
                material: wall_material.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            Collider { radius: 1.5 },
        ));
    }

    // 3. Işık (Görsel kalite için)
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // 4. Kamera (Tepeden takip eden)
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    // 5. Oyuncu (Mavi Küp)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.0, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player { facing_direction: Vec3::NEG_Z }, // Başlangıçta ileriye bakıyor
        Collider { radius: 0.7 },
    ));

    // 6. Düşmanlar (Kırmızı Küpler)
    let enemy_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        ..default()
    });
    let enemy_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let enemy_spawn_points = [
        Vec3::new(5.0, 0.5, 5.0),
        Vec3::new(-6.0, 0.5, 4.0),
        Vec3::new(2.0, 0.5, -7.0),
    ];

    for pos in enemy_spawn_points {
        commands.spawn((
            PbrBundle {
                mesh: enemy_mesh.clone(),
                material: enemy_material.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            Enemy,
            Collider { radius: 0.7 },
        ));
    }
}

/// WASD ile oyuncuyu hareket ettirir ve bakış yönünü (facing_direction) günceller
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyS) { direction.z += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            // Oyuncunun baktığı yönü güncelle (ateş etmek için kullanılacak)
            player.facing_direction = direction;
            // Pozisyonu güncelle
            transform.translation += direction * 5.0 * time.delta_seconds();
        }
    }
}

/// Kamera oyuncuyu yumuşak bir şekilde veya doğrudan tepeden takip eder
fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let offset = Vec3::new(0.0, 15.0, 10.0);
            camera_transform.translation = player_transform.translation + offset;
        }
    }
}

/// Düşmanlar sürekli oyuncuya doğru hareket eder
fn enemy_ai_movement(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut enemy_transform in enemy_query.iter_mut() {
            let direction = (player_transform.translation - enemy_transform.translation).normalize_or_zero();
            // Düşmanlar yavaşça hareket eder
            enemy_transform.translation += direction * 2.0 * time.delta_seconds();
        }
    }
}

/// Oyuncu sol tık yaptığında baktığı yöne mermi ateşler
fn player_shoot(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    player_query: Query<(&Transform, &Player)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok((transform, player)) = player_query.get_single() {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.2)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(1.0, 1.0, 0.0), // Sarı mermi
                        ..default()
                    }),
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                },
                Projectile {
                    direction: player.facing_direction,
                    speed: 15.0,
                    is_from_player: true,
                },
                Collider { radius: 0.2 },
            ));
        }
    }
}

/// Düşmanlar her 1 saniyede bir oyuncunun olduğu yöne ateş eder
fn enemy_shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemyShootTimer>,
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Zamanlayıcıyı güncelle
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok(player_transform) = player_query.get_single() {
            for enemy_transform in enemy_query.iter() {
                let direction = (player_transform.translation - enemy_transform.translation).normalize_or_zero();
                
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(0.2)),
                        material: materials.add(StandardMaterial {
                            base_color: Color::srgb(1.0, 0.5, 0.0), // Turuncu mermi
                            ..default()
                        }),
                        transform: Transform::from_translation(enemy_transform.translation),
                        ..default()
                    },
                    Projectile {
                        direction,
                        speed: 10.0,
                        is_from_player: false,
                    },
                    Collider { radius: 0.2 },
                ));
            }
        }
    }
}

/// Sahnede olan tüm mermileri ileriye doğru hareket ettirir
fn projectile_movement(
    mut query: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation += projectile.direction * projectile.speed * time.delta_seconds();
    }
}

/// Mesafe tabanlı (Distance-based) basit çarpışma tespiti sistemi

/// Düşman ölme Event'lerini dinler, objeyi sahneden siler ve terminale yazdırır
fn handle_enemy_death(
    mut commands: Commands,
    mut events: EventReader<EnemyDeathEvent>,
) {
    for event in events.read() {
        // Event'ten gelen objeyi (Düşmanı) sahneden tamamen kaldır
        if let Some(mut entity_commands) = commands.get_entity(event.0) {
            entity_commands.despawn();
            println!("Düşman yok edildi!");
        }
    }
}
