use bevy::prelude::*;
use rand::Rng; 
use rand::SeedableRng;
use rand::rngs::StdRng;
use super::components::{ChunkAnalysisData, SurfaceType};
use crate::game::placement::components::{
    Placement, PlacementKind, PlacementState, PlacementRuleKind, PlacementConfig
};
use crate::game::world::generation::field::ChunkManager;
use crate::game::world::generation::layout::ZoneType;
use crate::game::encounter::components::{ZoneEncounter, PlacedEnemyData};

#[derive(Component)]
pub struct SpawnedChunk;

pub fn chunk_spawner_system(
    mut commands: Commands,
    config: Res<PlacementConfig>, 
    chunk_manager: Res<ChunkManager>,
    mut encounters: Query<&mut ZoneEncounter>,
    query: Query<(Entity, &ChunkAnalysisData), Without<SpawnedChunk>>,
) {
    for (entity, analysis) in &query {
        let seed = (analysis.coord.x.wrapping_mul(73856093) 
                  ^ analysis.coord.y.wrapping_mul(19349663) 
                  ^ analysis.coord.z.wrapping_mul(83492791)) as u64;
        let mut rng = StdRng::seed_from_u64(seed);

        let mut true_floors = Vec::new();
        let mut true_ceilings = Vec::new();

        for surface in &analysis.surfaces {
            let corrected_normal = -surface.normal;
            
            let corrected_type = if corrected_normal.y > 0.5 {
                SurfaceType::Floor
            } else if corrected_normal.y < -0.5 {
                SurfaceType::Ceiling
            } else {
                SurfaceType::Wall
            };

            if corrected_type == SurfaceType::Floor {
                true_floors.push((surface, corrected_normal, corrected_type));
            } else if corrected_type == SurfaceType::Ceiling {
                true_ceilings.push((surface, corrected_normal, corrected_type));
            }
        }

        let mut cave_floors = Vec::new();
        let mut surface_floors = Vec::new();

        for (surface, normal, surf_type) in true_floors.clone() {
            let head_pos = surface.position + Vec3::Y * 1.5;
            let head_density = chunk_manager.get_density_at_world_pos(head_pos).unwrap_or(0.0);
            
            if head_density >= 0.35 {
                continue; 
            }

            let mut has_roof = false;
            for step in 2..=30 {
                let check_pos = surface.position + Vec3::Y * (step as f32);
                if let Some(density) = chunk_manager.get_density_at_world_pos(check_pos) {
                    if density >= 0.35 {
                        has_roof = true;
                        break;
                    }
                }
            }
            
            if has_roof {
                cave_floors.push((surface, normal, surf_type));
            } else {
                surface_floors.push((surface, normal, surf_type));
            }
        }

        let mut all_valid_floors = Vec::new();
        all_valid_floors.extend(cave_floors.clone());
        all_valid_floors.extend(surface_floors.clone());

        if all_valid_floors.is_empty() && true_ceilings.is_empty() {
            commands.entity(entity).insert(SpawnedChunk);
            continue;
        }

        let mut local_id_counter = 0;

        for rule in &config.rules {
            let candidates = match rule.surface_requirement {
                SurfaceType::Ceiling => &true_ceilings,
                SurfaceType::Wall => &Vec::new(), 
                SurfaceType::Floor => {
                    match rule.open_sky_requirement {
                        Some(true) => &surface_floors,
                        Some(false) => &cave_floors,
                        None => &all_valid_floors,
                    }
                }
            };

            if candidates.is_empty() {
                continue; 
            }

            if rule.kind == PlacementRuleKind::DynamicEnemy {
                let mut matched_candidates = Vec::new();
                
                for choice in candidates.iter() {
                    if let ZoneType::Room(room_id) = choice.0.zone {
                        matched_candidates.push((choice.0.region_coord, room_id, choice.clone()));
                    }
                }

                for mut encounter in encounters.iter_mut() {
                    let pre_placed: Vec<PlacedEnemyData> = encounter.placed_enemies
                        .iter()
                        .filter(|p| p.chunk_coord == analysis.coord)
                        .cloned()
                        .collect();

                    if !pre_placed.is_empty() {
                        for placed_data in pre_placed {
                            commands.spawn((
                                SpatialBundle {
                                    transform: Transform::from_translation(placed_data.position),
                                    ..default()
                                },
                                Placement {
                                    kind: PlacementKind::Enemy(placed_data.variant),
                                    state: PlacementState::Active,
                                    chunk_coord: analysis.coord,
                                    local_id: local_id_counter,
                                    normal: placed_data.normal,
                                    surface_type: placed_data.surface_type.clone(),
                                    zone: placed_data.zone.clone(),
                                },
                            ));
                            local_id_counter += 1;
                        }
                        continue; 
                    }

                    if encounter.pending_enemies.is_empty() { 
                        encounter.is_resolved = true;
                        continue; 
                    }
                    
                    let mut valid_spots: Vec<_> = matched_candidates.iter()
                        .filter(|(r, id, _)| *r == encounter.region_coord && *id == encounter.room_id)
                        .map(|(_, _, c)| c.clone())
                        .collect();
                        
                    if valid_spots.is_empty() { continue; }
                    
                    while !encounter.pending_enemies.is_empty() {
                        if valid_spots.is_empty() {
                            break; 
                        }
                        let pick_idx = rng.random_range(0..valid_spots.len());
                        let choice = valid_spots.remove(pick_idx);
                        
                        let variant = encounter.pending_enemies.remove(0);
                        let spawn_pos = choice.0.position;
                        
                        encounter.placed_enemies.push(PlacedEnemyData {
                            chunk_coord: analysis.coord,
                            variant: variant.clone(),
                            position: spawn_pos,
                            normal: choice.1,
                            surface_type: choice.2.clone(),
                            zone: choice.0.zone.clone(),
                        });
                        
                        commands.spawn((
                            SpatialBundle {
                                transform: Transform::from_translation(spawn_pos),
                                ..default()
                            },
                            Placement {
                                kind: PlacementKind::Enemy(variant),
                                state: PlacementState::Active,
                                chunk_coord: analysis.coord,
                                local_id: local_id_counter,
                                normal: choice.1,
                                surface_type: choice.2,
                                zone: choice.0.zone,
                            },
                        ));
                        local_id_counter += 1;
                    }

                    if encounter.pending_enemies.is_empty() {
                        encounter.is_resolved = true;
                    }
                }
            } else {
                let mut available_candidates = candidates.clone();
                for _ in 0..rule.count_per_chunk {
                    if available_candidates.is_empty() { break; }
                    
                    let pick_idx = rng.random_range(0..available_candidates.len());
                    let choice = available_candidates.remove(pick_idx);
                    
                    let spawn_pos = choice.0.position;
                    let normal = choice.1;
                    let surface_type = choice.2;
                    let zone = choice.0.zone;

                    if let PlacementRuleKind::Object(obj) = &rule.kind {
                        commands.spawn((
                            SpatialBundle {
                                transform: Transform::from_translation(spawn_pos),
                                ..default()
                            },
                            Placement {
                                kind: PlacementKind::Object(obj.clone()),
                                state: PlacementState::Active,
                                chunk_coord: analysis.coord,
                                local_id: local_id_counter,
                                normal,
                                surface_type,
                                zone, 
                            },
                        ));
                        local_id_counter += 1;
                    }
                }
            }
        }

        commands.entity(entity).insert(SpawnedChunk);
    }
}