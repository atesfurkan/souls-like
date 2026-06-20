use bevy::prelude::*;
use bevy::render::mesh::Mesh3d;
use rand::Rng; // veya use rand::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Işık
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 2. Zemin
    commands.spawn((
        // Mesh ve Material bileşenlerini doğrudan struct olarak değil, 
        // Bundle veya component yapısına uygun şekilde tanımlıyoruz.
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.7, 0.2))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 3. Rastgele Engeller
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        // random_range kullanımı (rand 0.9+ için)
        let x = rng.random_range(-8.0..8.0);
        let z = rng.random_range(-8.0..8.0);
        
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
            Transform::from_xyz(x, 0.5, z),
        ));
    }
}
