use bevy::prelude::*;

// --- Bileşenler (Components) ---
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, move_enemies, check_collisions))
        .run();
}

// --- Sahne Kurulumu ---
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Kamera (Tepeden bakış)
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Zemin
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
        ..default()
    });

    // Oyuncu
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb(0.0, 0.7, 0.2)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
    ));

    // Düşmanlar
    for i in 0..3 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.8, 0.8, 0.8)),
                material: materials.add(Color::srgb(0.8, 0.0, 0.0)),
                transform: Transform::from_xyz(i as f32 * 3.0 - 3.0, 0.4, -5.0),
                ..default()
            },
            Enemy,
        ));
    }

    // Işık
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

// --- Sistemler ---

// Oyuncu Hareketi (WASD)
fn move_player(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    let mut transform = query.single_mut();
    let speed = 5.0;
    if keys.pressed(KeyCode::KeyW) { transform.translation.z -= speed * time.delta_seconds(); }
    if keys.pressed(KeyCode::KeyS) { transform.translation.z += speed * time.delta_seconds(); }
    if keys.pressed(KeyCode::KeyA) { transform.translation.x -= speed * time.delta_seconds(); }
    if keys.pressed(KeyCode::KeyD) { transform.translation.x += speed * time.delta_seconds(); }
}

// Düşman Takip Sistemi
fn move_enemies(
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_pos = player.single().translation;
    for mut enemy_transform in &mut enemies {
        let direction = (player_pos - enemy_transform.translation).normalize_or_zero();
        enemy_transform.translation += direction * 1.5 * time.delta_seconds();
    }
}

// Çarpışma Kontrolü (Basit mesafe kontrolü)
fn check_collisions(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let player_pos = player.single().translation;
    for (entity, enemy_transform) in &enemies {
        // Eğer mesafe 1 birimden azsa çarpışma say
        if player_pos.distance(enemy_transform.translation) < 1.0 {
            commands.entity(entity).despawn();
            println!("Düşman yok edildi!");
        }
    }
}
