use bevy::prelude::*;
use bevy::utils::HashMap;
use std::collections::VecDeque;

#[derive(Resource, Clone)]
pub struct ChunkManager {
    pub chunks: HashMap<IVec3, ScalarField>,
    pub chunk_size: UVec3,
}

impl ChunkManager {
    pub fn new(chunk_size: UVec3) -> Self {
        Self {
            chunks: HashMap::new(),
            chunk_size,
        }
    }
    
    // YENİ: Dünya koordinatından density (katılık) okuyan fonksiyon
    pub fn get_density_at_world_pos(&self, world_pos: Vec3) -> Option<f32> {
        // Hangi offset'te olduğumuzu tahmin etmek yerine, tüm field'lara bakıp 
        // koordinatı içeren chunk'ı buluyoruz.
        for field in self.chunks.values() {
            let local_pos = world_pos - field.world_origin;
            let x = local_pos.x.round() as i32;
            let y = local_pos.y.round() as i32;
            let z = local_pos.z.round() as i32;

            if x >= 0 && y >= 0 && z >= 0 
                && x < field.padded_size.x as i32 
                && y < field.padded_size.y as i32 
                && z < field.padded_size.z as i32 
            {
                return Some(field.get_density(x as u32, y as u32, z as u32));
            }
        }
        None
    }
    
    pub fn get_light_at_world_pos(&self, world_pos: Vec3, chunk_size: UVec3, offset: Vec3) -> Vec3 {
        let local_to_grid = world_pos - offset;
        
        let chunk_x = (local_to_grid.x / chunk_size.x as f32).floor() as i32;
        let chunk_y = (local_to_grid.y / chunk_size.y as f32).floor() as i32;
        let chunk_z = (local_to_grid.z / chunk_size.z as f32).floor() as i32;
        let chunk_coord = IVec3::new(chunk_x, chunk_y, chunk_z);

        if let Some(field) = self.chunks.get(&chunk_coord) {
            let local_pos = world_pos - field.world_origin;
            
            let x = local_pos.x.round() as u32;
            let y = local_pos.y.round() as u32;
            let z = local_pos.z.round() as u32;

            if x < field.padded_size.x && y < field.padded_size.y && z < field.padded_size.z {
                return field.get_light(x, y, z);
            }
        }

        Vec3::ZERO 
    }
}

#[derive(Clone)]
pub struct ScalarField {
    pub chunk_coord: IVec3,
    pub logical_size: UVec3,
    pub padded_size: UVec3,
    pub data: Vec<f32>,
    pub light_data: Vec<Vec3>,
    pub world_origin: Vec3,
}

impl ScalarField {
    pub fn new(chunk_coord: IVec3, logical_size: UVec3, world_origin: Vec3) -> Self {
        let padded_size = UVec3::new(logical_size.x + 1, logical_size.y + 1, logical_size.z + 1);
        let total_cells = (padded_size.x * padded_size.y * padded_size.z) as usize;
        Self {
            chunk_coord,
            logical_size,
            padded_size,
            data: vec![0.0; total_cells],
            light_data: vec![Vec3::new(4.95, 4.95, 8.08); total_cells], 
            world_origin,
        }
    }

    pub fn get_index(&self, x: u32, y: u32, z: u32) -> usize {
        (z * self.padded_size.x * self.padded_size.y + y * self.padded_size.x + x) as usize
    }

    pub fn get_density(&self, x: u32, y: u32, z: u32) -> f32 {
        if x >= self.padded_size.x || y >= self.padded_size.y || z >= self.padded_size.z {
            return 0.0;
        }
        self.data[self.get_index(x, y, z)]
    }

    pub fn set_density(&mut self, x: u32, y: u32, z: u32, value: f32) {
        if x < self.padded_size.x && y < self.padded_size.y && z < self.padded_size.z {
            let idx = self.get_index(x, y, z);
            self.data[idx] = value;
        }
    }

    pub fn get_light(&self, x: u32, y: u32, z: u32) -> Vec3 {
        if x >= self.padded_size.x || y >= self.padded_size.y || z >= self.padded_size.z { return Vec3::ZERO; }
        self.light_data[self.get_index(x, y, z)]
    }

    pub fn set_light(&mut self, x: u32, y: u32, z: u32, color: Vec3) {
        if x < self.padded_size.x && y < self.padded_size.y && z < self.padded_size.z {
            let idx = self.get_index(x, y, z);
            self.light_data[idx] = color;
        }
    }

    pub fn compute_lighting(&mut self, world_light_pos: Vec3, color: Vec3, max_distance: i32) {
        let local_pos = world_light_pos - self.world_origin;
        let lx = local_pos.x.round() as i32;
        let ly = local_pos.y.round() as i32;
        let lz = local_pos.z.round() as i32;

        if lx < 0 || ly < 0 || lz < 0 || lx >= self.padded_size.x as i32 || ly >= self.padded_size.y as i32 || lz >= self.padded_size.z as i32 {
            return;
        }

        let mut queue = VecDeque::new();
        queue.push_back((lx as u32, ly as u32, lz as u32, max_distance));
        self.set_light(lx as u32, ly as u32, lz as u32, color);

        let falloff = 1.0 / (max_distance as f32);

        while let Some((x, y, z, dist)) = queue.pop_front() {
            if dist <= 0 { continue; }

            let neighbors = [
                (x.wrapping_sub(1), y, z), (x + 1, y, z),
                (x, y.wrapping_sub(1), z), (x, y + 1, z),
                (x, y, z.wrapping_sub(1)), (x, y, z + 1),
            ];

            let current_light = self.get_light(x, y, z);

            for &(nx, ny, nz) in &neighbors {
                if nx < self.padded_size.x && ny < self.padded_size.y && nz < self.padded_size.z {
                    let density = self.get_density(nx, ny, nz);
                    let target_color = current_light - (color * falloff); 
                    let target_color = target_color.max(Vec3::ZERO);
                    let existing_light = self.get_light(nx, ny, nz);

                    if target_color.length_squared() > existing_light.length_squared() + 0.01 {
                        self.set_light(nx, ny, nz, target_color);
                        if density < 0.35 {
                            queue.push_back((nx, ny, nz, dist - 1));
                        }
                    }
                }
            }
        }
    }
}