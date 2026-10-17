use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Artık her şeyi doğru bir şekilde components içinden alıyor
use super::components::{
    BlockingState, DefaultShieldShape, Shield, ShieldBox, ShieldVisualAnchor, ShieldVisualData,
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShieldType {
    Buckler,
    KiteShield,
    TowerShield,
}

pub struct ShieldTemplate {
    pub stats: Shield,
    pub visual: ShieldVisualData,
}

pub fn get_shield_data(shield_type: ShieldType) -> ShieldTemplate {
    match shield_type {
        ShieldType::Buckler => ShieldTemplate {
            stats: Shield {
                name: "Buckler".to_string(),
                block_rate: 0.65,
                stamina_cost_multiplier: 1.5,
                parry_window: 0.35,
            },
            visual: ShieldVisualData::ProgrammerArt {
                shape: DefaultShieldShape::Cylinder { radius: 0.25, height: 0.05 },
                color: Color::srgb(0.6, 0.4, 0.2),
            },
        },
        ShieldType::KiteShield => ShieldTemplate {
            stats: Shield {
                name: "Kite Shield".to_string(),
                block_rate: 1.0,
                stamina_cost_multiplier: 0.8,
                parry_window: 0.15,
            },
            visual: ShieldVisualData::ProgrammerArt {
                shape: DefaultShieldShape::Box { width: 0.4, height: 0.6, depth: 0.05 },
                color: Color::srgb(0.7, 0.7, 0.8),
            },
        },
        ShieldType::TowerShield => ShieldTemplate {
            stats: Shield {
                name: "Tower Shield".to_string(),
                block_rate: 1.0,
                stamina_cost_multiplier: 0.2,
                parry_window: 0.0,
            },
            visual: ShieldVisualData::ProgrammerArt {
                shape: DefaultShieldShape::Box { width: 0.6, height: 1.2, depth: 0.15 },
                color: Color::srgb(0.2, 0.2, 0.25),
            },
        },
    }
}

pub fn get_shield_stats_bundle(shield_type: ShieldType) -> (Shield, BlockingState) {
    let template = get_shield_data(shield_type);
    (template.stats, BlockingState::default())
}

pub fn build_shield_hierarchy(
    builder: &mut ChildBuilder,
    shield_type: ShieldType,
    owner: Entity,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let template = get_shield_data(shield_type);

    let (collider, mesh, material, board_offset) = match &template.visual {
        ShieldVisualData::ProgrammerArt { shape, color } => {
            let mat = materials.add(StandardMaterial {
                base_color: *color,
                ..default()
            });

            match shape {
                DefaultShieldShape::Box { width, height, depth } => (
                    Collider::cuboid(*width / 2.0, *height / 2.0, *depth / 2.0),
                    meshes.add(Cuboid::new(*width, *height, *depth)),
                    mat,
                    Vec3::new(0.0, 0.0, 0.1),
                ),
                DefaultShieldShape::Cylinder { radius, height } => (
                    Collider::cylinder(*height / 2.0, *radius),
                    meshes.add(Cylinder { radius: *radius, half_height: *height / 2.0 }),
                    mat,
                    Vec3::new(0.0, 0.0, 0.05),
                ),
            }
        }
    };

    builder
        .spawn((
            ShieldVisualAnchor,
            ShieldBox { owner },
            collider,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            SpatialBundle {
                transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 3.0, 3.0)),
                ..default()
            },
        ))
        .with_children(|shield_anchor| {
            shield_anchor.spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(board_offset)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                ..default()
            });

            let black_material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.05, 0.05, 0.05),
                ..default()
            });

            shield_anchor.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.12, 0.04, 0.12)),
                material: black_material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });

            shield_anchor.spawn(PbrBundle {
                mesh: meshes.add(Cylinder { radius: 0.02, half_height: 0.08 }),
                material: black_material,
                transform: Transform::from_xyz(0.0, -0.25, 0.05)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            });
        });
}