use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::common::components::{BoneColliderMarker, HurtboxProfile, ProxyProcessed};
use crate::game::combat::components::Hurtbox;

pub fn setup_anatomical_hurtboxes(
    mut commands: Commands,
    profile_query: Query<(Entity, &HurtboxProfile)>,
    children_query: Query<&Children>,
    names_query: Query<&Name>,
    mesh_handles: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
    unprocessed_proxies: Query<Entity, (Without<ProxyProcessed>, With<Name>)>,
) {
    for (owner_entity, profile) in profile_query.iter() {
        for descendant in children_query.iter_descendants(owner_entity) {
            if unprocessed_proxies.contains(descendant) {
                if let Ok(name) = names_query.get(descendant) {
                    let name_str = name.as_str();
                    
                    if profile.proxies.contains(&name_str) {
                        // 1. Orijinal Blender Mesh'ini gizle
                        commands.entity(descendant).insert(Visibility::Hidden);
                        
                        // 2. Mesh Handle'ı bul
                        let mut target_mesh_entity = None;
                        let mut target_mesh_handle = None;

                        if let Ok(handle) = mesh_handles.get(descendant) {
                            target_mesh_entity = Some(descendant);
                            target_mesh_handle = Some(handle.clone());
                        } else {
                            for sub_desc in children_query.iter_descendants(descendant) {
                                if let Ok(handle) = mesh_handles.get(sub_desc) {
                                    target_mesh_entity = Some(sub_desc);
                                    target_mesh_handle = Some(handle.clone());
                                    break;
                                }
                            }
                        }

                        // 3. Collider ve Combat Hurtbox'ı Oluştur
                        if let (Some(mesh_ent), Some(handle)) = (target_mesh_entity, target_mesh_handle) {
                            if let Some(mesh_data) = meshes.get(&handle) {
                                if let Some(collider) = Collider::from_bevy_mesh(mesh_data, &ComputedColliderShape::ConvexHull) {
                                    
                                    commands.entity(mesh_ent).insert((
                                        collider,
                                        BoneColliderMarker { 
                                            bone_name: name_str.to_string(), 
                                            owner: owner_entity 
                                        },
                                        Hurtbox { owner: owner_entity },
                                        Sensor,
                                        ActiveEvents::COLLISION_EVENTS,
                                    ));

                                    commands.entity(descendant).insert(ProxyProcessed);
                                    info!("BAŞARILI: Owner {:?} için {} mesh'inden ortak ConvexHull hurtbox üretildi!", owner_entity, name_str);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
