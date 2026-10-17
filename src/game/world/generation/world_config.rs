use bevy::prelude::*;
use crate::game::world::generation::config::CaveGenerationConfig;

#[derive(Resource, Reflect, Clone, Debug)]
#[reflect(Resource)]
pub struct WorldGenerationConfig {
    pub chunk_logical_size: UVec3,
    pub render_distance: u32,
    pub unload_distance: u32,
    pub cave_config: CaveGenerationConfig,
}

impl Default for WorldGenerationConfig {
    fn default() -> Self {
        Self {
            // Dünya geneli chunk boyutu. Eski Cave sistemindeki değerlerle uyumlu.
            chunk_logical_size: UVec3::new(32, 32, 32),
            // Kameranın etrafında yüklenecek chunk yarıçapı
            render_distance: 2,
            // Kameradan ne kadar uzaklaşan chunk'ların bellekten silineceği
            unload_distance: 4,
            cave_config: CaveGenerationConfig::default(),
        }
    }
}
