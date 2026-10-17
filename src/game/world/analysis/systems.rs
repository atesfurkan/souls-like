use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::game::world::generation::region::RegionManager;
use super::components::*;

#[derive(Resource, Default)]
pub struct AnalysisTracker {
    pub processed_regions: HashSet<IVec2>,
}

pub fn extract_region_analysis_system(
    mut commands: Commands,
    region_manager: Res<RegionManager>,
    mut tracker: ResMut<AnalysisTracker>,
) {
    for (&region_coord, layout) in &region_manager.layouts {
        if !tracker.processed_regions.contains(&region_coord) {
            
            let mut connection_counts = vec![0; layout.rooms.len()];
            for c in &layout.corridors {
                connection_counts[c.from_room_id] += 1;
                connection_counts[c.to_room_id] += 1;
            }

            let mut chokepoints = Vec::new();
            for c in &layout.corridors {
                let p_a = layout.rooms[c.from_room_id].center;
                let r_a = layout.rooms[c.from_room_id].radius;
                let p_b = layout.rooms[c.to_room_id].center;
                let r_b = layout.rooms[c.to_room_id].radius;

                let dir = (p_b - p_a).normalize_or_zero();
                if dir != Vec3::ZERO {
                    chokepoints.push(p_a + dir * r_a);
                    chokepoints.push(p_b - dir * r_b);
                }
            }

            let mut rooms = Vec::new();
            for r in &layout.rooms {
                let extents = Vec3::new(
                    r.radius * r.scale.x,
                    r.radius * r.scale.y,
                    r.radius * r.scale.z,
                );
                
                rooms.push(RoomAnalysis {
                    id: r.id,
                    center: r.center,
                    radius: r.radius,
                    bounds_min: r.center - extents,
                    bounds_max: r.center + extents,
                    is_dead_end: connection_counts[r.id] == 1,
                    max_clearance: 20.0,
                    average_slope: 0.0,
                    true_floor_y: r.center.y,
                });
            }

            let mut corridors = Vec::new();
            for c in &layout.corridors {
                corridors.push(CorridorAnalysis {
                    from_room_id: c.from_room_id,
                    to_room_id: c.to_room_id,
                    radius: c.radius,
                });
            }

            let mut entrances = Vec::new();
            for e in &layout.entrances {
                entrances.push(EntranceAnalysis {
                    start_pos: e.start_pos,
                    end_pos: e.end_pos,
                    radius: e.radius,
                });
            }

            commands.spawn(RegionAnalysisData {
                region_coord,
                rooms,
                corridors,
                entrances,
                sockets: Vec::new(),
                chokepoints,
            });

            tracker.processed_regions.insert(region_coord);
        }
    }
}

pub fn debug_region_analysis_system(
    _query: Query<&RegionAnalysisData>,
    _gizmos: Gizmos,
) {}
