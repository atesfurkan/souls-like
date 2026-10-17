use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_rapier3d::prelude::*;
use crate::game::player::components::{PendingColliderFit, PendingEquipment, Player};
use crate::game::animation::components::CharacterBones;
use crate::game::combat::components::{EquippedWeapon, RightHand, LeftHand};
use crate::game::ik::components::{IkTarget, TwoBoneIkChain};
use crate::game::shield::data::ShieldType;

pub fn auto_fit_player_collider(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Collider), (With<Player>, With<PendingColliderFit>)>,
    children_query: Query<&Children>,
    aabb_query: Query<&Aabb>,
) {
    for (player_entity, mut collider) in player_query.iter_mut() {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        let mut found_mesh = false;

        for descendant in children_query.iter_descendants(player_entity) {
            if let Ok(aabb) = aabb_query.get(descendant) {
                let center = Vec3::from(aabb.center);
                let half_extents = Vec3::from(aabb.half_extents);

                min = min.min(center - half_extents);
                max = max.max(center + half_extents);
                found_mesh = true;
            }
        }

        if found_mesh {
            let size = max - min;
            let radius = (size.x.max(size.z)) / 5.0;
            let half_height = (size.y / 2.0) - radius;

            *collider = Collider::capsule_y(half_height.max(0.1), radius.max(0.1));
            
            commands.entity(player_entity).remove::<PendingColliderFit>();
            info!("BAŞARILI: Oyuncu fiziği 3D modele göre otomatik sarıldı! Yarıçap: {:.2}", radius);
        }
    }
}

pub fn attach_pending_equipment(
    mut commands: Commands,
    character_query: Query<(Entity, &CharacterBones)>,
    equipment_query: Query<
        (Entity, &Parent, Option<&EquippedWeapon>, Option<&ShieldType>),
        With<PendingEquipment>,
    >,
) {
    if equipment_query.is_empty() {
        return;
    }

    for (character_entity, bones) in character_query.iter() {
        if !bones.is_linked {
            continue;
        }

        for (eq_entity, parent, is_weapon, is_shield) in equipment_query.iter() {
            if parent.get() != character_entity {
                continue;
            }

            if is_weapon.is_some() {
                if let (Some(right_arm), Some(right_forearm), Some(right_hand)) =
                    (bones.right_arm, bones.right_forearm, bones.right_hand)
                {
                    commands
                        .entity(eq_entity)
                        .remove::<PendingEquipment>()
                        .insert(Transform::IDENTITY)
                        .insert(IkTarget { position: Vec3::ZERO, weight: 0.0 })
                        .insert(TwoBoneIkChain {
                            root: right_arm,
                            mid: right_forearm,
                            tip: right_hand,
                            pole_vector: Vec3::new(1.0, -1.0, -1.0),
                        });
                    commands.entity(right_hand).add_child(eq_entity);
                }
            } else if is_shield.is_some() {
                if let Some(left_forearm) = bones.left_forearm {
                    commands
                        .entity(eq_entity)
                        .remove::<PendingEquipment>()
                        .insert(Transform::IDENTITY);
                    commands.entity(left_forearm).add_child(eq_entity);

                    if let (Some(arm), Some(hand)) = (bones.left_arm, bones.left_hand) {
                        commands.entity(character_entity).insert(TwoBoneIkChain {
                            root: arm,
                            mid: left_forearm,
                            tip: hand,
                            pole_vector: Vec3::new(-1.0, -1.0, -1.0),
                        });
                    }
                }
            }
        }
    }
}

pub fn attach_unarmed_ik_chains(
    mut commands: Commands,
    character_query: Query<(Entity, &CharacterBones)>,
    right_hand_query: Query<(Entity, &Parent), (With<RightHand>, Without<TwoBoneIkChain>)>,
    left_hand_query: Query<(Entity, &Parent), (With<LeftHand>, Without<TwoBoneIkChain>)>,
) {
    for (character_entity, bones) in character_query.iter() {
        if !bones.is_linked {
            continue;
        }

        // Sağ elin marker entity'sini bul, gerçek animasyon kemiğine (right_hand_bone) taşı ve IK kur!
        for (hand_entity, parent) in right_hand_query.iter() {
            if parent.get() == character_entity {
                if let (Some(right_arm), Some(right_forearm), Some(right_hand_bone)) =
                    (bones.right_arm, bones.right_forearm, bones.right_hand)
                {
                    commands
                        .entity(hand_entity)
                        .insert(Transform::IDENTITY) // Eski statik konumu sıfırla, tam kemiğe otursun
                        .insert(IkTarget { position: Vec3::ZERO, weight: 0.0 })
                        .insert(TwoBoneIkChain {
                            root: right_arm,
                            mid: right_forearm,
                            tip: right_hand_bone,
                            pole_vector: Vec3::new(1.0, -1.0, -1.0),
                        });
                    
                    // Marker entity'yi CharacterRoot yerine El Kemiğine çocuk olarak bağla
                    commands.entity(right_hand_bone).add_child(hand_entity);
                }
            }
        }

        // Sol elin marker entity'sini bul, gerçek animasyon kemiğine (left_hand_bone) taşı ve IK kur!
        for (hand_entity, parent) in left_hand_query.iter() {
            if parent.get() == character_entity {
                if let (Some(left_arm), Some(left_forearm), Some(left_hand_bone)) =
                    (bones.left_arm, bones.left_forearm, bones.left_hand)
                {
                    commands
                        .entity(hand_entity)
                        .insert(Transform::IDENTITY) // Eski statik konumu sıfırla, tam kemiğe otursun
                        .insert(IkTarget { position: Vec3::ZERO, weight: 0.0 })
                        .insert(TwoBoneIkChain {
                            root: left_arm,
                            mid: left_forearm,
                            tip: left_hand_bone,
                            pole_vector: Vec3::new(-1.0, -1.0, -1.0),
                        });
                    
                    // Marker entity'yi CharacterRoot yerine El Kemiğine çocuk olarak bağla
                    commands.entity(left_hand_bone).add_child(hand_entity);
                }
            }
        }
    }
}
