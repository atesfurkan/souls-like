use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::combat::components::Hurtbox;
use crate::game::player::components::Player;

// Blender'da hazırladığımız tüm proxy isimlerinin eksiksiz listesi
pub const ALL_PROXIES: &[&str] = &[
    "Head_Proxy",
    "Hips_Proxy",
    "LeftArm_Proxy",
    "LeftFoot_Proxy",
    "LeftForeArm_Proxy",
    "LeftHand_Proxy",
    "LeftLeg_Proxy",
    "LeftUpLeg_Proxy",
    "RightArm_Proxy",
    "RightFoot_Proxy",
    "RightForeArm_Proxy",
    "RightHand_Proxy",
    "RightLeg_Proxy",
    "RightUpLeg_Proxy",
    "Spine2_Proxy",
];

// Collider gizmo çizimleri ve debug için kullanılacak işaretçi
#[derive(Component)]
pub struct BoneColliderMarker {
    pub bone_name: &'static str,
    pub owner: Entity,
}

// Bir proxy bir kez işlendikten sonra tekrar taranmaması için güvenlik bileşeni
#[derive(Component)]
pub struct ProxyProcessed;

pub fn hide_blender_proxies(
    mut commands: Commands,
    proxies_query: Query<(Entity, &Name, Option<&Visibility>), Without<BoneColliderMarker>>,
) {
    for (entity, name, vis) in proxies_query.iter() {
        let name_str = name.as_str();
        if ALL_PROXIES.contains(&name_str) {
            if vis.is_none() {
                commands.entity(entity).insert(Visibility::Hidden);
            } else if let Some(v) = vis {
                if *v != Visibility::Hidden {
                    commands.entity(entity).insert(Visibility::Hidden);
                }
            }
        }
    }
}

pub fn setup_bone_colliders(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    children_query: Query<&Children>,
    names_query: Query<&Name>,
    mesh_handles: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
    // Yalnızca henüz işlenmemiş (ProxyProcessed olmayan) nesneleri tara
    unprocessed_proxies: Query<Entity, (Without<ProxyProcessed>, With<Name>)>,
) {
    for player_entity in player_query.iter() {
        for descendant in children_query.iter_descendants(player_entity) {
            if unprocessed_proxies.contains(descendant) {
                if let Ok(name) = names_query.get(descendant) {
                    let name_str = name.as_str();
                    
                    // Nesne adı proxy listemizde var mı?
                    if let Some(&proxy_name) = ALL_PROXIES.iter().find(|&&p| p == name_str) {
                        
                        let mut target_mesh_entity = None;
                        let mut target_mesh_handle = None;

                        // Mesh bileşeni doğrudan objede olabilir veya GLTF yükleyicisi mesh'i bir alt Node'a atmış olabilir
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

                        if let (Some(mesh_ent), Some(handle)) = (target_mesh_entity, target_mesh_handle) {
                            if let Some(mesh_data) = meshes.get(&handle) {
                                // Blender'daki polygon yapısını milimetrik kapsayan Dışbükey Örtü (ConvexHull) collider
                                if let Some(collider) = Collider::from_bevy_mesh(mesh_data, &ComputedColliderShape::ConvexHull) {
                                    
                                    // 1. Mesh'in olduğu entity'e collider ve hurtbox'ı ekle
                                    commands.entity(mesh_ent).insert((
                                        collider,
                                        BoneColliderMarker { 
                                            bone_name: proxy_name, 
                                            owner: player_entity 
                                        },
                                        Hurtbox { owner: player_entity },
                                        Sensor,
                                        ActiveEvents::COLLISION_EVENTS,
                                    ));

                                    // 2. Ana proxy objesini "İşlendi" olarak işaretle ki sonsuz döngüye girmesin
                                    commands.entity(descendant).insert(ProxyProcessed);
                                    
                                    info!("BAŞARILI: {} mesh'inden ConvexHull collider üretildi ve entegre edildi!", proxy_name);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
