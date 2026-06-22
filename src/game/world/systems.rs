use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng; // Rastgele engeller için eklendi

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Görsel zemin (Plane)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::srgb(0.3, 0.4, 0.3)),
            ..default()
        },
        // Zemin Fizikleri
        RigidBody::Fixed,
        Collider::cuboid(25.0, 0.05, 25.0), // Zemin için 50x50'nin yarısı
    ));

    // Güneş Işığı
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        ..default()
    });

    // Rastgele Engeller (Obstacles)
    let mut rng = rand::thread_rng();
    let cube_mesh = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let cube_mat = materials.add(Color::srgb(0.4, 0.4, 0.4));

    for _ in 0..20 {
        let x: f32 = rng.gen_range(-20.0..20.0);
        let z: f32 = rng.gen_range(-20.0..20.0);

        // Oyuncunun tam merkezde (0, 0, 0) doğduğunu biliyoruz.
        // O yüzden merkeze 3 birimden yakınsa küp oluşturmayı atla ki oyuncu taşın içinde sıkışmasın.
        if x > -3.0 && x < 3.0 && z > -3.0 && z < 3.0 {
            continue;
        }

        commands.spawn((
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: cube_mat.clone(),
                transform: Transform::from_xyz(x, 1.0, z),
                ..default()
            },
            // Engel Fizikleri
            RigidBody::Fixed,
            Collider::cuboid(1.0, 1.0, 1.0), // 2x2x2 küp için yarı boyutları (half-extents)
        ));
    }
}
