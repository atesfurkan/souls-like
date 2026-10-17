use bevy::prelude::*;
use crate::game::placement::components::EnemySpawnVariant;
use crate::game::world::analysis::components::SurfaceType;
use crate::game::world::generation::layout::ZoneType;

// YENİ: Load/Unload döngülerinde düşman yerleşimlerini hatırlayacak yapı
#[derive(Clone, Debug)]
pub struct PlacedEnemyData {
    pub chunk_coord: IVec3,
    pub variant: EnemySpawnVariant,
    pub position: Vec3,
    pub normal: Vec3,
    pub surface_type: SurfaceType,
    pub zone: ZoneType,
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct ZoneEncounter {
    pub region_coord: IVec2,
    pub room_id: usize,
    pub encounter_id: String,
    pub remaining_budget: u32,
    #[reflect(ignore)]
    pub pending_enemies: Vec<EnemySpawnVariant>,
    // YENİ: Tüketilen placement verileri burada kalıcı olarak saklanır
    #[reflect(ignore)]
    pub placed_enemies: Vec<PlacedEnemyData>,
    pub is_resolved: bool,
}

impl ZoneEncounter {
    pub fn new(region_coord: IVec2, room_id: usize, encounter_id: String, budget: u32, enemies: Vec<EnemySpawnVariant>) -> Self {
        Self {
            region_coord,
            room_id,
            encounter_id,
            remaining_budget: budget,
            pending_enemies: enemies,
            placed_enemies: Vec::new(),
            is_resolved: false,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct EncounterPlanned;