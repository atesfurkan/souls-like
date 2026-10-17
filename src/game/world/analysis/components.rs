use bevy::prelude::*;
use crate::game::world::generation::layout::ZoneType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[derive(Default)]
pub enum SurfaceType {
    #[default]
    Floor,
    Wall,
    Ceiling,
}

#[derive(Clone, Debug)]
pub struct SurfacePoint {
    pub position: Vec3,
    pub normal: Vec3,
    pub surface_type: SurfaceType,
    pub zone: ZoneType,
    pub region_coord: IVec2,
}

#[derive(Component, Clone, Debug)]
pub struct ChunkAnalysisData {
    pub coord: IVec3,
    pub surfaces: Vec<SurfacePoint>,
}

#[derive(Component, Clone, Debug)]
pub struct RegionAnalysisData {
    pub region_coord: IVec2,
    pub rooms: Vec<RoomAnalysis>,
    pub corridors: Vec<CorridorAnalysis>,
    pub entrances: Vec<EntranceAnalysis>,
    pub sockets: Vec<SocketAnalysis>,
    pub chokepoints: Vec<Vec3>,
}

#[derive(Clone, Debug)]
pub struct RoomAnalysis {
    pub id: usize,
    pub center: Vec3,
    pub radius: f32,
    pub bounds_min: Vec3,
    pub bounds_max: Vec3,
    pub is_dead_end: bool,
    pub max_clearance: f32,
    pub average_slope: f32,
    pub true_floor_y: f32,
}

#[derive(Clone, Debug)]
pub struct CorridorAnalysis {
    pub from_room_id: usize,
    pub to_room_id: usize,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct EntranceAnalysis {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct SocketAnalysis {
    pub world_position: Vec3,
    pub normal: Vec3,
    pub socket_type: u32,
}