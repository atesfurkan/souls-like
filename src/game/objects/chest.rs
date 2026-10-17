use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::components::Chest;

pub fn build_chest(
    commands: &mut Commands,
    position: Vec3,
    normal: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material = materials.add(Color::srgb(0.4, 0.26, 0.13));
    let collider = Collider::cuboid(0.5, 0.5, 0.5);

    let offset = normal * 0.5 + Vec3::Y * 1.5;
    
    let rotation = if normal.distance(Vec3::NEG_Y) < 0.01 {
        Quat::from_rotation_x(std::f32::consts::PI)
    } else {
        Quat::from_rotation_arc(Vec3::Y, normal)
    };

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(position + offset).with_rotation(rotation),
            ..default()
        },
        RigidBody::Dynamic, 
        collider,
        LockedAxes::ROTATION_LOCKED,
        Damping { linear_damping: 3.0, angular_damping: 3.0 },
        Chest,
    )).id()
}