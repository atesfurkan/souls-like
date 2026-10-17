use bevy::prelude::*;
use crate::game::placement::components::{
    Placement, PlacementKind, PlacementState, ObjectSpawnVariant, ChunkEntityLink, LinkedPlacement
};
use crate::game::world::analysis::components::SurfaceType;
use crate::game::world::generation::chunk_tracker::ChunkUnloadEvent;
use super::chest::build_chest;
use super::stalactite::build_stalactite;
use super::tree::build_tree;
use super::components::{Chest, Stalactite, Tree};

pub fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut placement_query: Query<(Entity, &Transform, &mut Placement)>,
) {
    for (placement_entity, transform, mut placement) in placement_query.iter_mut() {
        if placement.state != PlacementState::Active {
            continue;
        }

        let variant = match &placement.kind {
            PlacementKind::Object(v) => v.clone(),
            _ => continue,
        };

        placement.state = PlacementState::Consumed;

        let position = transform.translation;
        let normal = placement.normal;
        let surface_type = placement.surface_type;

        let spawned_entity_opt = match variant {
            ObjectSpawnVariant::Chest => {
                if surface_type == SurfaceType::Floor {
                    Some(build_chest(&mut commands, position, normal, &mut meshes, &mut materials))
                } else { None }
            }
            ObjectSpawnVariant::Stalactite => {
                if surface_type == SurfaceType::Ceiling {
                    Some(build_stalactite(&mut commands, position, normal, &mut meshes, &mut materials))
                } else { None }
            }
            ObjectSpawnVariant::Tree => {
                if surface_type == SurfaceType::Floor {
                    Some(build_tree(&mut commands, position, normal, &mut meshes, &mut materials))
                } else { None }
            }
        };

        if let Some(spawned_entity) = spawned_entity_opt {
            commands.entity(spawned_entity).insert((
                ChunkEntityLink(placement.chunk_coord),
                LinkedPlacement(placement_entity),
            ));
        }
    }
}

pub fn handle_object_despawn(
    mut commands: Commands,
    mut unload_events: EventReader<ChunkUnloadEvent>,
    object_query: Query<
        (Entity, &ChunkEntityLink, &LinkedPlacement, &Transform),
        Or<(With<Chest>, With<Tree>, With<Stalactite>)>
    >,
    mut placement_query: Query<(&mut Placement, &mut Transform), Without<ChunkEntityLink>>,
) {
    for unload_event in unload_events.read() {
        let unloaded_chunk = unload_event.0;

        for (entity, link, linked_placement, obj_transform) in object_query.iter() {
            if link.0 == unloaded_chunk {
                // Obje hayatta, durumunu güncelle ve tekrar doğması için Active hale getir
                if let Ok((mut placement, mut placement_transform)) = placement_query.get_mut(linked_placement.0) {
                    *placement_transform = *obj_transform;
                    placement.state = PlacementState::Active;
                }
                
                // Çalışma zamanı entity'sini güvenlice sil
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}