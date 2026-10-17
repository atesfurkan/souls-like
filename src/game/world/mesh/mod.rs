pub mod builder;
pub mod marching_cubes;
pub mod material; 
pub mod tables;

use bevy::prelude::*;
use bevy::render::texture::{
    ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
};
use bevy_rapier3d::prelude::*;
use futures_lite::future;
use rand::Rng;

use crate::game::world::generation::chunk_tracker::{ChunkLoadEvent, ChunkUnloadEvent};
use crate::game::world::generation::config::{ActiveCaveType, CaveGenerationConfig, CaveType};
use crate::game::world::generation::field::{ChunkManager, ScalarField};
use crate::game::world::generation::{ChunkGenTask, ChunkResult};
use crate::game::world::mesh::builder::build_mesh;
use crate::game::world::mesh::marching_cubes::extract_surface;
use crate::game::world::mesh::material::TriplanarMaterial;
use crate::game::world::analysis::components::{SurfacePoint, SurfaceType, ChunkAnalysisData};

#[derive(Resource)]
pub struct CaveMaterialResource(pub Handle<TriplanarMaterial>);

fn setup_cave_materials(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<TriplanarMaterial>>,
) {
    let tile_sampler = |settings: &mut ImageLoaderSettings| {
        settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            address_mode_w: ImageAddressMode::Repeat,
            ..default()
        });
    };

    let triplanar_mat = TriplanarMaterial {
        wall_texture: asset_server.load_with_settings("textures/cave_wall.png", tile_sampler),
        floor_texture: asset_server.load_with_settings("textures/cave_floor.png", tile_sampler),
        ceiling_texture: asset_server.load_with_settings("textures/cave_ceiling.png", tile_sampler),
    };

    let handle = materials.add(triplanar_mat);
    commands.insert_resource(CaveMaterialResource(handle));
}

#[derive(Component)]
pub struct CaveChunk(pub IVec3);

#[derive(Resource)]
pub struct CaveMaterialCache(pub Handle<StandardMaterial>);

pub struct MeshGenPlugin;

impl Plugin for MeshGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TriplanarMaterial>::default())
            .add_systems(Startup, (setup_material, setup_cave_materials))
            .add_systems(
                Update,
                (
                    handle_chunk_load_events,
                    poll_chunk_tasks,
                    handle_chunk_unload_events,
                ).chain(),
            );
    }
}

pub fn handle_chunk_load_events(
    mut commands: Commands,
    mut load_events: EventReader<ChunkLoadEvent>,
    config: Res<CaveGenerationConfig>,
    active_type: Res<ActiveCaveType>,
    mut region_manager: ResMut<crate::game::world::generation::region::RegionManager>,
    chunk_manager: Res<ChunkManager>,
) {
    let current_config = match active_type.0 {
        CaveType::Natural => &config.natural,
        CaveType::Mine => &config.mine,
        CaveType::LavaTube => &config.lava_tube,
        CaveType::Ruin => &config.ruin,
    };
    
    let chunk_logical_size = chunk_manager.chunk_size;
    let thread_pool = bevy::tasks::AsyncComputeTaskPool::get();

    for &ChunkLoadEvent(coord) in load_events.read() {
        let chunk_world_offset = Vec3::new(
            (coord.x as f32) * chunk_logical_size.x as f32,
            (coord.y as f32) * chunk_logical_size.y as f32,
            (coord.z as f32) * chunk_logical_size.z as f32,
        );

        let chunk_center = chunk_world_offset + Vec3::new(
            chunk_logical_size.x as f32 / 2.0,
            chunk_logical_size.y as f32 / 2.0,
            chunk_logical_size.z as f32 / 2.0,
        );

        let current_region_coord = crate::game::world::generation::region::get_region_coord(chunk_center, current_config.grid_size);

        let mut all_rooms = Vec::new();
        let mut all_corridors = Vec::new();
        let mut all_entrances = Vec::new();

        let mut relevant_layouts = bevy::utils::HashMap::new();

        for rx in (current_region_coord.x - 1)..=(current_region_coord.x + 1) {
            for rz in (current_region_coord.y - 1)..=(current_region_coord.y + 1) {
                
                if crate::game::world::generation::region::should_spawn_cave(rx, rz, current_config.spawn_density) {
                    let r_coord = IVec2::new(rx, rz);
                    
                    let layout = region_manager.layouts.entry(r_coord).or_insert_with(|| {
                        crate::game::world::generation::region::generate_region_layout(rx, rz, current_config, &config)
                    });
                    
                    relevant_layouts.insert(r_coord, layout.clone());
                    
                    let room_offset = all_rooms.len();
                    for mut r in layout.rooms.clone() {
                        r.id += room_offset;
                        all_rooms.push(r);
                    }
                    for mut c in layout.corridors.clone() {
                        c.from_room_id += room_offset;
                        c.to_room_id += room_offset;
                        all_corridors.push(c);
                    }
                    all_entrances.extend(layout.entrances.clone());
                }
            }
        }

        let config_clone = current_config.clone();
        let global_seed = 424242; 

        let task = thread_pool.spawn(async move {
            let mut field = ScalarField::new(coord, chunk_logical_size, chunk_world_offset);
            let mut chunk_lights = Vec::new();
            
            let chunk_radius = 28.0; 
            let safety_margin = 45.0; 

            let local_rooms: Vec<_> = all_rooms.iter().filter(|r| {
                r.center.distance(chunk_center) < (r.radius + safety_margin + chunk_radius)
            }).cloned().collect();

            struct ResolvedCorridor {
                p_a: Vec3, p_b: Vec3, radius: f32, scale: Vec3,
                shape: crate::game::world::generation::config::ShapeType,
                floor_noise: f32, noise_scale: f32,
            }
            let local_corridors: Vec<_> = all_corridors.iter().map(|c| {
                ResolvedCorridor {
                    p_a: all_rooms[c.from_room_id].center,
                    p_b: all_rooms[c.to_room_id].center,
                    radius: c.radius, scale: c.scale, shape: c.shape,
                    floor_noise: c.floor_noise, noise_scale: c.noise_scale,
                }
            }).filter(|c| {
                let mid = (c.p_a + c.p_b) * 0.5;
                let length = c.p_a.distance(c.p_b);
                mid.distance(chunk_center) < (length / 2.0) + c.radius + safety_margin + chunk_radius
            }).collect();
            
            let local_entrances: Vec<_> = all_entrances.into_iter().filter(|e| {
                let mid = (e.start_pos + e.end_pos) * 0.5;
                let length = e.start_pos.distance(e.end_pos);
                mid.distance(chunk_center) < (length / 2.0) + e.radius + safety_margin + chunk_radius
            }).collect();

            for z in 0..field.padded_size.z {
                for y in 0..field.padded_size.y {
                    for x in 0..field.padded_size.x {
                        let world_pos = field.world_origin + Vec3::new(x as f32, y as f32, z as f32);
                        
                        let mut smooth_min_dist = f32::MAX;
                        let mut true_min_dist = f32::MAX;
                        
                        let mut current_floor_noise = 0.1;
                        let mut current_height_var = 0.0;
                        let mut current_noise_scale = config_clone.global_noise_scale;

                        for room in &local_rooms {
                            let dist = match room.shape {
                                crate::game::world::generation::config::ShapeType::Round => {
                                    crate::game::world::generation::sdf::sdf_ellipsoid(world_pos, room.center, room.radius, room.scale)
                                }
                                crate::game::world::generation::config::ShapeType::Boxy => {
                                    crate::game::world::generation::sdf::sdf_boxy(world_pos, room.center, room.radius, room.scale)
                                }
                            };
                            
                            if dist < true_min_dist {
                                true_min_dist = dist;
                                if dist < 35.0 {
                                    current_floor_noise = room.floor_noise;
                                    current_height_var = room.height_variance;
                                    current_noise_scale = config_clone.global_noise_scale;
                                } else {
                                    current_floor_noise = 0.1;
                                    current_height_var = 0.0;
                                    current_noise_scale = config_clone.global_noise_scale;
                                }
                            }
                            smooth_min_dist = crate::game::world::generation::sdf::smooth_min(smooth_min_dist, dist, 4.0);
                        }

                        for corridor in &local_corridors {
                            let scaled_pos = corridor.p_a + (world_pos - corridor.p_a) * corridor.scale;

                            let dist = match corridor.shape {
                                crate::game::world::generation::config::ShapeType::Round => {
                                    crate::game::world::generation::sdf::sdf_capsule(scaled_pos, corridor.p_a, corridor.p_b, corridor.radius)
                                }
                                crate::game::world::generation::config::ShapeType::Boxy => {
                                    crate::game::world::generation::sdf::sdf_boxy_capsule(scaled_pos, corridor.p_a, corridor.p_b, corridor.radius)
                                }
                            };
                            
                            if dist < true_min_dist {
                                true_min_dist = dist;
                                if dist < 35.0 {
                                    current_floor_noise = corridor.floor_noise;
                                    current_height_var = 0.0;
                                    current_noise_scale = corridor.noise_scale;
                                } else {
                                    current_floor_noise = 0.1;
                                    current_height_var = 0.0;
                                    current_noise_scale = config_clone.global_noise_scale;
                                }
                            }
                            let smooth_factor = if corridor.shape == crate::game::world::generation::config::ShapeType::Boxy { 1.0 } else { 4.0 };
                            smooth_min_dist = crate::game::world::generation::sdf::smooth_min(smooth_min_dist, dist, smooth_factor);
                        }
                        
                        for entrance in &local_entrances {
                            let dist = crate::game::world::generation::sdf::sdf_capsule(world_pos, entrance.start_pos, entrance.end_pos, entrance.radius);
                            
                            if dist < true_min_dist {
                                true_min_dist = dist;
                                if dist < 35.0 {
                                    current_floor_noise = 0.1;
                                    current_height_var = 0.0;
                                    current_noise_scale = config_clone.global_noise_scale;
                                } else {
                                    current_floor_noise = 0.1;
                                    current_height_var = 0.0;
                                    current_noise_scale = config_clone.global_noise_scale;
                                }
                            }
                            smooth_min_dist = crate::game::world::generation::sdf::smooth_min(smooth_min_dist, dist, 4.0);
                        }
                        
                        let surface_level = 20.0;
                        let terrain_sdf = surface_level - world_pos.y;
                        
                        let final_sdf = terrain_sdf.min(smooth_min_dist);

                        let nx = world_pos.x * current_noise_scale;
                        let ny = world_pos.y * current_noise_scale;
                        let nz = world_pos.z * current_noise_scale;
                        let noise_perturbation = (crate::game::world::generation::noise::fbm_3d(nx, ny, nz, 4, global_seed) - 0.5) * 1.5;

                        let height_gradient = (world_pos.y / (config_clone.grid_size.y as f32)).clamp(0.0, 1.0);
                        let h_variance_effect = current_height_var * height_gradient;

                        let final_density = 0.35 + (final_sdf * current_floor_noise) + noise_perturbation + (h_variance_effect * 0.1);
                        field.set_density(x, y, z, final_density);
                    }
                }
            }

            let mut surfaces = Vec::new();
            for z in 1..(field.padded_size.z - 1) {
                for y in 1..(field.padded_size.y - 1) {
                    for x in 1..(field.padded_size.x - 1) {
                        let density = field.get_density(x, y, z);
                        
                        if density < 0.35 {
                            let is_surface = 
                                field.get_density(x + 1, y, z) >= 0.35 || field.get_density(x - 1, y, z) >= 0.35 ||
                                field.get_density(x, y + 1, z) >= 0.35 || field.get_density(x, y - 1, z) >= 0.35 ||
                                field.get_density(x, y, z + 1) >= 0.35 || field.get_density(x, y, z - 1) >= 0.35;

                            if is_surface {
                                let grad_x = field.get_density(x + 1, y, z) - field.get_density(x - 1, y, z);
                                let grad_y = field.get_density(x, y + 1, z) - field.get_density(x, y - 1, z);
                                let grad_z = field.get_density(x, y, z + 1) - field.get_density(x, y, z - 1);
                                
                                let normal = Vec3::new(grad_x, grad_y, grad_z).normalize_or_zero();
                                
                                let surface_type = if normal.y > 0.6 {
                                    SurfaceType::Floor
                                } else if normal.y < -0.6 {
                                    SurfaceType::Ceiling
                                } else {
                                    SurfaceType::Wall
                                };

                                let world_pos = field.world_origin + Vec3::new(x as f32, y as f32, z as f32);
                                let region_coord = crate::game::world::generation::region::get_region_coord(world_pos, config_clone.grid_size);
                                
                                let mut zone = if let Some(layout) = relevant_layouts.get(&region_coord) {
                                    layout.get_zone_at(world_pos)
                                } else {
                                    crate::game::world::generation::layout::ZoneType::Wilderness
                                };

                                // KRİTİK ÇÖZÜM (ZONE FALLBACK) DÜZELTİLDİ: Corridor(..) kullanılıyor.
                                if matches!(zone, crate::game::world::generation::layout::ZoneType::Wilderness | crate::game::world::generation::layout::ZoneType::Corridor(..)) {
                                    if let Some(layout) = relevant_layouts.get(&region_coord) {
                                        let mut min_margin = 25.0; // 25 birimlik devasa bir gürültü/sarkma toleransı
                                        let mut best_room = None;
                                        
                                        for room in &layout.rooms {
                                            // Odanın scale (ölçek) değerini hesaba katarak eliptik mesafeyi bul
                                            let delta = (world_pos - room.center) / room.scale;
                                            let dist = delta.length();
                                            let margin = dist - room.radius;
                                            
                                            // En iyi (en yakın) odayı seç
                                            if margin < min_margin {
                                                min_margin = margin;
                                                best_room = Some(room.id);
                                            }
                                        }
                                        
                                        // Eğer uygun bir oda bulunduysa, taşıyan zemini o odaya zorla ata
                                        if let Some(r_id) = best_room {
                                            zone = crate::game::world::generation::layout::ZoneType::Room(r_id);
                                        }
                                    }
                                }

                                surfaces.push(SurfacePoint {
                                    position: world_pos,
                                    normal,
                                    surface_type,
                                    zone,
                                    region_coord,
                                });
                            }
                        }
                    }
                }
            }
            
            let mut rng = rand::thread_rng();
            for z in (2..field.padded_size.z - 2).step_by(6) {
                for y in (2..field.padded_size.y - 2).step_by(6) {
                    for x in (2..field.padded_size.x - 2).step_by(6) {
                        let density = field.get_density(x, y, z);
                        if density < 0.35 {
                            let n_x = field.get_density(x + 1, y, z);
                            let n_y = field.get_density(x, y - 1, z);
                            
                            if n_x >= 0.35 || n_y >= 0.35 {
                                if rng.gen::<f32>() < config_clone.wall_light_chance {
                                    let light_pos = field.world_origin + Vec3::new(x as f32, y as f32, z as f32);
                                    let l_color_linear = config_clone.light_color.to_linear();
                                    let intensity_multiplier = config_clone.light_intensity / 1000.0;
                                    let l_color = Vec3::new(
                                        l_color_linear.red * intensity_multiplier,
                                        l_color_linear.green * intensity_multiplier,
                                        l_color_linear.blue * intensity_multiplier,
                                    );
                                    
                                    field.compute_lighting(light_pos, l_color, config_clone.light_range as i32);
                                    chunk_lights.push((light_pos, l_color));
                                }
                            }
                        }
                    }
                }
            }
            
            let surface = extract_surface(&field, 0.35, 1.0);
            let mesh = if !surface.vertices.is_empty() { Some(build_mesh(surface)) } else { None };

            ChunkResult { coord, field, mesh, generated_lights: chunk_lights, surfaces }
        });

        commands.spawn((ChunkGenTask(Some(task)), CaveChunk(coord)));
    }
}

fn poll_chunk_tasks(
    mut commands: Commands,
    mut task_query: Query<(Entity, &mut ChunkGenTask)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    material_cache: Res<CaveMaterialCache>, 
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut task) in task_query.iter_mut() {
        if let Some(mut inner_task) = task.0.take() {
            if let Some(result) = future::block_on(future::poll_once(&mut inner_task)) {
                let ChunkResult { coord, field, mesh, generated_lights, surfaces } = result;

                let Some(mut entity_commands) = commands.get_entity(entity) else {
                    continue;
                };

                chunk_manager.chunks.insert(coord, field);

                entity_commands.insert(ChunkAnalysisData {
                    coord,
                    surfaces,
                });

                if let Some(cave_mesh) = mesh {
                    let collider = Collider::from_bevy_mesh(&cave_mesh, &ComputedColliderShape::TriMesh)
                            .expect("Mesh'ten collider uretilemedi!");

                    let chunk_material = material_cache.0.clone();

                    let chunk_entity = entity_commands.insert((
                        PbrBundle {
                            mesh: meshes.add(cave_mesh),
                            material: chunk_material,
                            transform: Transform::IDENTITY,
                            ..default()
                        },
                        RigidBody::Fixed,
                        collider,
                    )).id();

                    for (l_pos, l_color) in generated_lights {
                        let torch_mat = standard_materials.add(StandardMaterial {
                            base_color: Color::srgb(l_color.x, l_color.y, l_color.z),
                            unlit: true,
                            ..default()
                        });
                        let torch_mesh = meshes.add(Capsule3d::new(0.3, 1.0));
                        let torch_entity = commands.spawn(PbrBundle {
                            mesh: torch_mesh,
                            material: torch_mat,
                            transform: Transform::from_translation(l_pos),
                            ..default()
                        }).id();
                        commands.entity(chunk_entity).add_child(torch_entity);
                    }
                } else {
                    entity_commands.despawn_recursive();
                }
            } else {
                task.0 = Some(inner_task);
            }
        }
    }
}

fn handle_chunk_unload_events(
    mut commands: Commands,
    mut unload_events: EventReader<ChunkUnloadEvent>,
    mut chunk_manager: ResMut<ChunkManager>,
    q_chunks: Query<(Entity, &CaveChunk)>,
) {
    for &ChunkUnloadEvent(coord) in unload_events.read() {
        chunk_manager.chunks.remove(&coord);
        
        for (entity, chunk_comp) in q_chunks.iter() {
            if chunk_comp.0 == coord {
                commands.entity(entity).despawn_recursive();
                break;
            }
        }
    }
}

fn setup_material(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    let sampler_settings = |settings: &mut ImageLoaderSettings| {
        settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            address_mode_w: ImageAddressMode::Repeat,
            ..default()
        });
    };

    let wall_tex: Handle<Image> =
        asset_server.load_with_settings("textures/cave_wall.png", sampler_settings);

    let material = StandardMaterial {
        base_color_texture: Some(wall_tex),
        perceptual_roughness: 0.85, 
        ..default()
    };

    let mat_handle = materials.add(material);
    commands.insert_resource(CaveMaterialCache(mat_handle));
}