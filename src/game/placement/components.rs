use bevy::prelude::*;
use crate::game::world::analysis::components::SurfaceType;
use crate::game::world::generation::layout::ZoneType;

// --- YENİ EKLENEN ORTAK BİLEŞENLER ---
#[derive(Component, Debug, Clone, Copy)]
pub struct ChunkEntityLink(pub IVec3);

#[derive(Component, Debug, Clone, Copy)]
pub struct LinkedPlacement(pub Entity);
// ------------------------------------

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum PlacementRuleKind {
    DynamicEnemy,
    Object(ObjectSpawnVariant),
}

#[derive(Reflect, Clone, Debug)]
pub struct PlacementRule {
    pub kind: PlacementRuleKind,
    pub count_per_chunk: usize,
    pub surface_requirement: SurfaceType,
    pub open_sky_requirement: Option<bool>,
}

#[derive(Resource, Clone, Debug)]
pub struct PlacementConfig {
    pub rules: Vec<PlacementRule>,
}

impl Default for PlacementConfig {
    fn default() -> Self {
        Self {
            rules: vec![
                PlacementRule {
                    kind: PlacementRuleKind::Object(ObjectSpawnVariant::Chest),
                    count_per_chunk: 1,
                    surface_requirement: SurfaceType::Floor,
                    open_sky_requirement: None,
                },
                PlacementRule {
                    kind: PlacementRuleKind::Object(ObjectSpawnVariant::Stalactite),
                    count_per_chunk: 0,
                    surface_requirement: SurfaceType::Ceiling,
                    open_sky_requirement: None,
                },
                PlacementRule {
                    kind: PlacementRuleKind::DynamicEnemy,
                    count_per_chunk: 2,
                    surface_requirement: SurfaceType::Floor,
                    open_sky_requirement: None,
                },
                PlacementRule {
                    kind: PlacementRuleKind::Object(ObjectSpawnVariant::Tree),
                    count_per_chunk: 0,
                    surface_requirement: SurfaceType::Floor,
                    open_sky_requirement: Some(true),
                },
            ],
        }
    }
}

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct Placement {
    pub kind: PlacementKind,
    pub state: PlacementState,
    pub chunk_coord: IVec3, 
    pub local_id: usize,
    pub normal: Vec3,
    pub surface_type: SurfaceType,
    #[reflect(ignore)]
    pub zone: ZoneType, 
}

#[derive(Reflect, Clone, Debug, PartialEq, Default)]
pub enum PlacementKind {
    #[default]
    PlayerSpawn,
    Enemy(EnemySpawnVariant),
    Object(ObjectSpawnVariant),
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum EnemySpawnVariant {
    LightSpearman,
    Vanguard,
    HeavyDefender,
    BossArena,
    UnarmedMonster,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum ObjectSpawnVariant {
    Chest,
    Stalactite,
    Tree,
}

#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlacementState {
    #[default]
    Active,
    Consumed,  
    Discarded, 
}