use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::game::world::generation::config::{CaveTypeConfig, CaveGenerationConfig, SpawnRule, ShapeType};
use crate::game::world::generation::layout::{calculate_mst, CaveLayout, CorridorData, EntranceData, RoomData};

#[derive(Resource, Default)]
pub struct RegionManager {
    // Üretilen bölgelerin layout'larını önbellekte tutarız ki tekrar tekrar hesaplanmasın
    pub layouts: HashMap<IVec2, CaveLayout>,
}

/// Dünya koordinatından hangi X ve Z bölgesinde olduğumuzu buluruz.
pub fn get_region_coord(world_pos: Vec3, grid_size: UVec3) -> IVec2 {
    let rx = (world_pos.x / grid_size.x as f32).floor() as i32;
    let rz = (world_pos.z / grid_size.z as f32).floor() as i32;
    IVec2::new(rx, rz)
}

/// Sizin harika tam sayı bölmesi (Yuvarlama) mantığınız!
pub fn should_spawn_cave(rx: i32, rz: i32, density: f32) -> bool {
    if density <= 0.0 { return false; }
    
    // Adım 1: Tam sayı bölmesi ve yuvarlama
    let step = (1.0 / density).round() as i32;
    
    // 0.95 gibi değerler 1'e yuvarlanacağı için her bölgede çıkar
    if step <= 1 { return true; } 
    
    // Adım 2: Deterministik dağılım. (Zar atmak yerine koordinat modülünü alıyoruz 
    // ki geri döndüğümüzde mağara yer değiştirmiş olmasın).
    let hash = (rx.wrapping_mul(73856093) ^ rz.wrapping_mul(19349663)).abs();
    hash % step == 0
}

fn pick_profile_id<'a>(rules: &'a [SpawnRule], rng: &mut impl Rng) -> Option<&'a String> {
    let total_weight: f32 = rules.iter().map(|r| r.weight).sum();
    if total_weight <= 0.0 { return None; }
    
    let mut choice = rng.gen_range(0.0..total_weight);
    for rule in rules {
        choice -= rule.weight;
        if choice <= 0.0 { return Some(&rule.profile_id); }
    }
    rules.last().map(|r| &r.profile_id)
}

/// Eğer o bölgeye mağara düşüyorsa, sadece o bölge (Region) sınırları içinde Layout'u üret.
pub fn generate_region_layout(
    rx: i32,
    rz: i32,
    current_biome: &CaveTypeConfig,
    config: &CaveGenerationConfig,
) -> CaveLayout {
    // Bölge koordinatlarına göre özel seed üretimi (Aynı yere gelirsek aynı mağara çıkar)
    let seed_num = (rx.wrapping_mul(13371337) ^ rz.wrapping_mul(99991999)) as u64;
    let mut rng = StdRng::seed_from_u64(seed_num);

    let size = current_biome.grid_size;
    
    // Bu bölgenin dünya üzerindeki başlangıç (sıfır) noktası
    let origin_x = (rx as f32) * (size.x as f32);
    let origin_z = (rz as f32) * (size.z as f32);

    let mut rooms: Vec<Vec3> = Vec::new();
    let mut room_data_list: Vec<RoomData> = Vec::new();
    let target_room_count = rng.gen_range(current_biome.min_rooms..=current_biome.max_rooms);
    let surface_level = 20.0;

    for _ in 0..100 {
        if rooms.len() >= target_room_count { break; }
        
        let depth = rng.gen_range(current_biome.min_depth..=current_biome.max_depth);
        let pos_y = surface_level - depth;

        // X ve Z noktalarını kendi bölgesinin başlangıcına (origin) göre dağıtıyoruz
        let pos = Vec3::new(
            rng.gen_range((origin_x + 10.0)..(origin_x + size.x as f32 - 10.0)),
            pos_y, 
            rng.gen_range((origin_z + 10.0)..(origin_z + size.z as f32 - 10.0)),
        );
        
        if !rooms.iter().any(|&r| r.distance(pos) < 15.0) {
            rooms.push(pos);
            
            let mut r_radius = 5.0;
            let mut r_shape = ShapeType::Round;
            let mut r_scale = Vec3::ONE;
            let mut r_h_var = 0.0;
            let mut r_f_noise = 0.1;

            if let Some(profile_id) = pick_profile_id(&current_biome.room_spawn_rules, &mut rng) {
                if let Some(profile) = config.registered_rooms.iter().find(|p| &p.id == profile_id) {
                    r_radius = rng.gen_range(profile.min_radius..=profile.max_radius);
                    r_shape = profile.shape;
                    r_scale = profile.scale;
                    r_h_var = rng.gen_range(profile.min_height_variance..=profile.max_height_variance);
                    r_f_noise = profile.floor_noise;
                }
            }

            room_data_list.push(RoomData {
                id: rooms.len() - 1,
                center: pos,
                radius: r_radius,
                shape: r_shape,
                scale: r_scale,
                height_variance: r_h_var,
                floor_noise: r_f_noise,
            });
        }
    }

    let corridors_edges = calculate_mst(&rooms);
    let mut corridor_data_list = Vec::new();
    
    for edge in &corridors_edges {
        let mut c_radius = 3.0; 
        let mut c_noise_scale = current_biome.global_noise_scale;
        let mut c_floor_noise = 0.1;
        let mut c_shape = ShapeType::Round; 
        let mut c_scale = Vec3::ONE;        

        if let Some(profile_id) = pick_profile_id(&current_biome.corridor_spawn_rules, &mut rng) {
            if let Some(profile) = config.registered_corridors.iter().find(|p| &p.id == profile_id) {
                let width = rng.gen_range(profile.min_width..=profile.max_width);
                c_radius = width / 2.0; 
                c_noise_scale = profile.noise_scale;
                c_floor_noise = profile.floor_noise;
                c_shape = profile.shape;    
                c_scale = profile.scale;    
            }
        }

        corridor_data_list.push(CorridorData {
            from_room_id: edge.from,
            to_room_id: edge.to,
            radius: c_radius,
            noise_scale: c_noise_scale,
            floor_noise: c_floor_noise,
            shape: c_shape, 
            scale: c_scale, 
        });
    }

    let mut entrance_data_list = Vec::new();
    let actual_entrance_count = current_biome.entrance_count.min(rooms.len());

    for i in 0..actual_entrance_count {
        let start_room = rooms[i]; 
        let mut e_radius = 5.0;
        let mut e_ramp_length = 20.0; 

        if let Some(profile_id) = pick_profile_id(&current_biome.entrance_spawn_rules, &mut rng) {
            if let Some(profile) = config.registered_entrances.iter().find(|p| &p.id == profile_id) {
                e_radius = profile.radius;
                e_ramp_length = profile.ramp_length as f32;
            }
        }
        let end_pos = start_room + Vec3::new(e_ramp_length, (surface_level + 5.0) - start_room.y, e_ramp_length);
        entrance_data_list.push(EntranceData { start_pos: start_room, end_pos, radius: e_radius });
    }

    CaveLayout {
        rooms: room_data_list,
        corridors: corridor_data_list,
        entrances: entrance_data_list, 
        seed: seed_num as i32,
    }
}
