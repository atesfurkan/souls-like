use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::components::Stalactite;

pub fn build_stalactite(
    commands: &mut Commands,
    position: Vec3,
    _normal: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let radius = 0.3;
    let height = 1.5;

    let mesh = meshes.add(Cone { radius, height });
    let material = materials.add(Color::srgb(0.35, 0.35, 0.35));
    let collider = Collider::cone(height / 2.0, radius);

    let offset = Vec3::NEG_Y * (height / 2.0);
    let rotation = Quat::from_rotation_x(std::f32::consts::PI);

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(position + offset).with_rotation(rotation),
            ..default()
        },
        RigidBody::Fixed,
        collider,
        Stalactite,
    )).id()
}