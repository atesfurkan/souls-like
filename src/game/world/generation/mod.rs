pub mod config;
pub mod field;
pub mod layout;
pub mod noise;
pub mod sdf;
pub mod world_config;
pub mod chunk_tracker;
pub mod region;
pub mod chunk_lifecycle; // YENİ: Modül eklendi!

use bevy::prelude::*;
use bevy::tasks::Task;

use config::{ActiveCaveType, CaveGenerationConfig, CaveType};
use field::{ChunkManager, ScalarField};

use world_config::WorldGenerationConfig;
use chunk_tracker::{
    ActiveChunks, ChunkLoadEvent, ChunkUnloadEvent, 
    attach_chunk_viewer_to_camera, chunk_tracking_system,
};
use region::RegionManager;

// YENİ: Lifecycle sistemlerini içeri aktarıyoruz
use chunk_lifecycle::{
    spawn_chunk_tasks,
    handle_completed_chunk_tasks,
    handle_chunk_unloads,
};

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        // ChunkManager'ın ihtiyacı olan mantıksal boyutu tanımlıyoruz
        let chunk_logical_size = UVec3::new(32, 32, 32);

        app.init_resource::<CaveGenerationConfig>()
            .insert_resource(ActiveCaveType(CaveType::Natural))
            .init_resource::<WorldGenerationConfig>()
            .init_resource::<ActiveChunks>()
            .init_resource::<RegionManager>()
            .insert_resource(ChunkManager::new(chunk_logical_size)) 
            .add_event::<ChunkLoadEvent>()
            .add_event::<ChunkUnloadEvent>()
            .add_systems(Update, (
                attach_chunk_viewer_to_camera,
                chunk_tracking_system,
                // YENİ: Chunk üretim ve temizleme sistemlerini Bevy App'ine kaydediyoruz
                spawn_chunk_tasks,
                handle_completed_chunk_tasks,
                handle_chunk_unloads,
            ));
    }
}

pub struct ChunkResult {
    pub coord: IVec3,
    pub field: ScalarField,
    pub mesh: Option<Mesh>,
    pub generated_lights: Vec<(Vec3, Vec3)>,
    pub surfaces: Vec<crate::game::world::analysis::components::SurfacePoint>,
}

#[derive(Component)]
pub struct ChunkGenTask(pub Option<Task<ChunkResult>>);
