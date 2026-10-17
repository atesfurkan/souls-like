use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::world::generation::field::ChunkManager;
use crate::game::world::generation::config::{ActiveCaveType, CaveGenerationConfig, CaveType};

/// Hareketli nesnelerin statik ışıktan etkilenmesini sağlayan bileşen
#[derive(Component, Default)]
pub struct StaticLightProbe {
    pub current_light: Vec3,
}

pub fn update_entity_light_probes(
    chunk_manager: Res<ChunkManager>,
    config: Res<CaveGenerationConfig>,
    active_type: Res<ActiveCaveType>,
    mut query: Query<(&Transform, &mut StaticLightProbe)>,
) {
    let current_config = match active_type.0 {
        CaveType::Natural => &config.natural,
        CaveType::Mine => &config.mine,
        CaveType::LavaTube => &config.lava_tube,
        CaveType::Ruin => &config.ruin,
    };
    
    // Mağaranın ne kadar kaydırılarak oluşturulduğunu buluyoruz
    let size = current_config.grid_size;
    let offset = Vec3::new(
        -(size.x as f32) / 2.0,
        -(size.y as f32) / 2.0,
        -(size.z as f32) / 2.0,
    );

    let chunk_size = chunk_manager.chunk_size;

    for (transform, mut probe) in query.iter_mut() {
        let pos = transform.translation;
        // Ofset parametresiyle birlikte doğru noktadaki ışığı sorguluyoruz
        probe.current_light = chunk_manager.get_light_at_world_pos(pos, chunk_size, offset);
    }
}

pub fn spawn_test_spheres(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = meshes.add(Sphere::new(0.5));
    let ball_texture: Handle<Image> = asset_server.load("textures/ball.png");

    for i in 0..5 {
        let pos = Vec3::new(
            0.0,                    
            26.0,                   
            (i as f32 * 2.0) - 4.0, 
        );

        let sphere_mat = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(ball_texture.clone()),
            perceptual_roughness: 0.3,
            // YENİ DÜZELTME: Sahnedeki fiziksel ışıkları yok sayıp sadece 
            // bizim verdiğimiz rengi saf olarak göstermesi için unlit yapıyoruz.
            unlit: true,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: sphere_mesh.clone(),
                material: sphere_mat,
                transform: Transform::from_translation(pos),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Restitution::coefficient(0.8),
            StaticLightProbe::default(),
        ));
    }
}

pub fn update_sphere_materials(
    q_spheres: Query<(&StaticLightProbe, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (probe, mat_handle) in q_spheres.iter() {
        if let Some(mat) = materials.get_mut(mat_handle) {
            // Artık unlit kullandığımız için zifiri karanlık ortamda da 
            // objemiz bizim voxel ağından çektiğimiz iple "aydınlanmış" gibi görünecektir.
            mat.base_color = Color::srgb(
                probe.current_light.x.clamp(0.05, 1.0),
                probe.current_light.y.clamp(0.05, 1.0),
                probe.current_light.z.clamp(0.05, 1.0),
            );
        }
    }
}