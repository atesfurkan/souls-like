use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::components::Tree;

pub fn build_tree(
    commands: &mut Commands,
    position: Vec3,
    normal: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let trunk_radius = 0.3;
    let trunk_height = 2.0;
    let leaves_radius = 1.5;
    let leaves_height = 3.0;

    let trunk_mesh = meshes.add(Cylinder::new(trunk_radius, trunk_height));
    let trunk_mat = materials.add(Color::srgb(0.35, 0.22, 0.13)); 

    let leaves_mesh = meshes.add(Cone { radius: leaves_radius, height: leaves_height });
    let leaves_mat = materials.add(Color::srgb(0.15, 0.45, 0.15));

    let rotation = if normal.distance(Vec3::NEG_Y) < 0.01 {
        Quat::from_rotation_x(std::f32::consts::PI)
    } else {
        Quat::from_rotation_arc(Vec3::Y, normal)
    };

    let trunk_offset = normal * (trunk_height / 2.0);
    let collider = Collider::cylinder(trunk_height / 2.0, trunk_radius);

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position + trunk_offset).with_rotation(rotation),
            ..default()
        },
        RigidBody::Fixed,
        collider,
        Tree,
    )).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: trunk_mesh,
            material: trunk_mat,
            transform: Transform::default(),
            ..default()
        });

        parent.spawn(PbrBundle {
            mesh: leaves_mesh,
            material: leaves_mat,
            transform: Transform::from_translation(Vec3::Y * ((trunk_height / 2.0) + (leaves_height / 2.0))),
            ..default()
        });
    }).id()
}