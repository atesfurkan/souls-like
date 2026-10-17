use bevy::prelude::*;
use crate::game::world::generation::chunk_tracker::{ChunkLoadEvent, ChunkUnloadEvent};
use crate::game::world::generation::field::ChunkManager;
use crate::game::world::generation::world_config::WorldGenerationConfig;
use crate::game::world::generation::ChunkGenTask;
use crate::game::world::generation::region::RegionManager;
use crate::game::world::mesh::CaveMaterialResource;

#[derive(Component)]
pub struct ChunkEntity(pub IVec3);

pub fn spawn_chunk_tasks(
    _commands: Commands,
    mut _load_events: EventReader<ChunkLoadEvent>,
    _config: Res<WorldGenerationConfig>,
    _region_manager: Res<RegionManager>,
) {
    // Eski sin/cos dalgalı yüzey jeneratörü devreden çıkarıldı.
    // Gerçek üretim artık mesh/mod.rs üzerinden yapılıyor.
}

pub fn handle_completed_chunk_tasks(
    _commands: Commands,
    _chunk_manager: ResMut<ChunkManager>,
    _task_query: Query<(Entity, &mut ChunkGenTask)>,
    _meshes: ResMut<Assets<Mesh>>,
    _cave_material_res: Option<Res<CaveMaterialResource>>,
) {
    // Race condition (görev hırsızlığı) engellendi.
}

pub fn handle_chunk_unloads(
    _commands: Commands,
    mut _unload_events: EventReader<ChunkUnloadEvent>,
    _chunk_manager: ResMut<ChunkManager>,
    _chunk_entities: Query<(Entity, &ChunkEntity)>,
) {
    // Unload işlemleri zaten mesh/mod.rs içinde CaveChunk bileşeni ile ele alınıyor.
}