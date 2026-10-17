use super::components::{IkTarget, TwoBoneIkChain};
use crate::game::combat::components::{AttackIntent, CombatState, EquippedWeapon, GuardIntent, LeftHand, RightHand};
use crate::game::lock_on::components::LockOnState;
use crate::game::player::components::Player;
use crate::game::shield::components::{BlockingState, EquippedShield, ShieldVisualAnchor};
use crate::game::weapon::components::AttackType;
use bevy::prelude::*;
use crate::game::shield::data::ShieldType;

pub fn maintain_weapon_orientation(
    weapon_query: Query<(Entity, &Parent, &IkTarget), With<EquippedWeapon>>,
    mut transforms: Query<&mut Transform>,
    parents: Query<&Parent>,
    combat_query: Query<&CombatState>,
) {
    for (weapon_entity, weapon_parent, ik_target) in weapon_query.iter() {
        if ik_target.weight <= 0.01 {
            continue;
        }

        let mut current_entity = weapon_parent.get();
        let mut combat_data = None;
        let mut owner_entity = current_entity;

        loop {
            if let Ok(data) = combat_query.get(current_entity) {
                combat_data = Some(data);
                owner_entity = current_entity;
                break;
            }
            if let Ok(p) = parents.get(current_entity) {
                current_entity = p.get();
            } else {
                break;
            }
        }

        let Some(combat_state) = combat_data else {
            continue;
        };

        let owner_global = compute_manual_global_transform(owner_entity, &transforms, &parents);
        let hand_global =
            compute_manual_global_transform(weapon_parent.get(), &transforms, &parents);

        let base_weapon_rot = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4);

        let (is_thrust, thrust_weight) = match combat_state {
            CombatState::Windup {
                timer, attack_type, ..
            } if *attack_type == AttackType::Thrust => (true, timer.fraction()),
            CombatState::Active { attack_type, .. } if *attack_type == AttackType::Thrust => {
                (true, 1.0)
            }
            CombatState::Recovery { timer, attack_type } if *attack_type == AttackType::Thrust => {
                (true, 1.0 - timer.fraction())
            }
            _ => (false, 0.0),
        };

        let target_local_rot = if is_thrust && thrust_weight > 0.0 {
            let desired_global_rot =
                owner_global.rotation * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
            let thrust_local_rot = hand_global.rotation.inverse() * desired_global_rot;
            base_weapon_rot.slerp(thrust_local_rot, thrust_weight)
        } else {
            base_weapon_rot
        };

        if let Ok(mut weapon_transform) = transforms.get_mut(weapon_entity) {
            let safe_weight = ik_target.weight.clamp(0.0, 1.0);
            weapon_transform.rotation = weapon_transform
                .rotation
                .slerp(target_local_rot, safe_weight);
        }
    }
}

pub fn update_weapon_ik_target(
    mut weapon_query: Query<
        (
            &mut IkTarget,
            &mut TwoBoneIkChain,
            &Parent,
            Option<&EquippedWeapon>,
            Option<&RightHand>,
            Option<&LeftHand>,
        ),
    >,
    // Player'dan BlockingState ve ShieldType bilgisini çekiyoruz
    owner_query: Query<(&CombatState, &AttackIntent, &Transform, Option<&BlockingState>, Option<&ShieldType>)>,
    parent_query: Query<&Parent>,
    time: Res<Time>,
) {
    for (mut ik_target, mut chain, initial_parent, _is_weapon, is_right, is_left) in weapon_query.iter_mut() {
        let mut current_entity = initial_parent.get();
        let mut owner_data = None;

        loop {
            if let Ok(data) = owner_query.get(current_entity) {
                owner_data = Some(data);
                break;
            }
            if let Ok(parent) = parent_query.get(current_entity) {
                current_entity = parent.get();
            } else {
                break;
            }
        }

        let Some((combat_state, attack_intent, owner_transform, blocking_state_opt, shield_type_opt)) = owner_data else {
            continue;
        };

        // --- UNARMED FRONT GUARD MANTIĞI ---
        let is_blocking = match blocking_state_opt {
            Some(BlockingState::Blocking) | Some(BlockingState::Parrying { .. }) => true,
            _ => false,
        };
        // Oyuncunun üzerinde ShieldType yoksa kalkanı yoktur (Unarmed).
        let is_unarmed = shield_type_opt.is_none();

        if is_blocking && *combat_state == CombatState::Idle && is_unarmed {
            let target_weight = 1.0;
            let lerp_factor = (time.delta_seconds() * 25.0).clamp(0.0, 1.0);
            ik_target.weight = ik_target.weight.lerp(target_weight, lerp_factor);

            if ik_target.weight > 0.01 {
                if is_left.is_some() {
                    // Sol el pozisyonu: Merkeze sarılmak yerine önde, dışarıda ve göğüs hizasında
                    let local_target = Vec3::new(-0.25, 0.35, -0.4);
                    ik_target.position = owner_transform.transform_point(local_target);
                    // Dirsek (Pole Vector) dışarı ve aşağı baksın, doğal kompakt bir savunma hattı olsun
                    chain.pole_vector = Vec3::new(-1.0, -1.0, 0.0).normalize_or_zero();
                } else if is_right.is_some() {
                    // Sağ el pozisyonu: Sol el ile simetrik, önde ve göğüs hizasında
                    let local_target = Vec3::new(0.25, 0.35, -0.4);
                    ik_target.position = owner_transform.transform_point(local_target);
                    // Dirsek dışarı ve aşağı baksın
                    chain.pole_vector = Vec3::new(1.0, -1.0, 0.0).normalize_or_zero();
                }
            }
            
            // Eğer karakter savunma (block) pozisyonundaysa, silah atak pozisyonlarına geçmesin
            continue;
        }
        // --- UNARMED FRONT GUARD MANTIĞI BİTİŞ ---

        if is_right.is_some() && attack_intent.current == AttackType::UnarmedLeft {
            ik_target.weight = ik_target.weight.lerp(0.0, (time.delta_seconds() * 25.0).clamp(0.0, 1.0));
            continue;
        }
        if is_left.is_some() && attack_intent.current == AttackType::UnarmedRight {
            ik_target.weight = ik_target.weight.lerp(0.0, (time.delta_seconds() * 25.0).clamp(0.0, 1.0));
            continue;
        }

        if (is_right.is_some() || is_left.is_some()) 
            && (attack_intent.current != AttackType::UnarmedRight && attack_intent.current != AttackType::UnarmedLeft) 
        {
            ik_target.weight = ik_target.weight.lerp(0.0, (time.delta_seconds() * 25.0).clamp(0.0, 1.0));
            continue;
        }

        let (target_local_pos, target_weight, target_pole) = match combat_state {
            CombatState::Idle => (Vec3::ZERO, 0.0, Vec3::new(1.0, -1.0, -1.0)),
            CombatState::Windup { timer, .. }
            | CombatState::Active { timer, .. }
            | CombatState::Recovery { timer, .. } => {
                let phase = match combat_state {
                    CombatState::Windup { .. } => 0,
                    CombatState::Active { .. } => 1,
                    CombatState::Recovery { .. } => 2,
                    _ => 0,
                };
                let progress = timer.fraction();

                match attack_intent.current {
                    AttackType::Thrust => match phase {
                        0 => (
                            Vec3::new(0.4, 0.4, 0.2).lerp(Vec3::new(0.4, 0.4, 0.8), progress),
                            1.0,
                            Vec3::new(1.0, 0.0, -1.0),
                        ),
                        1 => (
                            Vec3::new(0.4, 0.4, 0.8).lerp(Vec3::new(0.0, 0.4, -1.5), progress),
                            1.0,
                            Vec3::new(1.0, 0.0, -1.0),
                        ),
                        2 => (
                            Vec3::new(0.0, 0.4, -1.5).lerp(Vec3::ZERO, progress),
                            1.0 - progress,
                            Vec3::new(1.0, -1.0, -1.0),
                        ),
                        _ => (Vec3::ZERO, 0.0, Vec3::new(1.0, -1.0, -1.0)),
                    },
                    AttackType::LeftSwing => match phase {
                        0 => (
                            Vec3::new(-0.2, 0.6, 0.2).lerp(Vec3::new(-0.8, 0.7, 0.0), progress),
                            1.0,
                            Vec3::new(1.0, 1.0, 0.0),
                        ),
                        1 => (
                            Vec3::new(-0.8, 0.7, 0.0).lerp(Vec3::new(1.0, 0.4, -0.8), progress),
                            1.0,
                            Vec3::new(1.0, 1.0, 0.0),
                        ),
                        2 => (
                            Vec3::new(1.0, 0.4, -0.8).lerp(Vec3::ZERO, progress),
                            1.0 - progress,
                            Vec3::new(1.0, -1.0, -1.0),
                        ),
                        _ => (Vec3::ZERO, 0.0, Vec3::new(1.0, -1.0, -1.0)),
                    },
                    AttackType::RightSwing => match phase {
                        0 => (
                            Vec3::new(0.2, 0.6, 0.2).lerp(Vec3::new(1.0, 0.7, 0.0), progress),
                            1.0,
                            Vec3::new(1.0, 1.0, 0.0),
                        ),
                        1 => (
                            Vec3::new(1.0, 0.7, 0.0).lerp(Vec3::new(-0.8, 0.4, -0.8), progress),
                            1.0,
                            Vec3::new(1.0, 1.0, 0.0),
                        ),
                        2 => (
                            Vec3::new(-0.8, 0.4, -0.8).lerp(Vec3::ZERO, progress),
                            1.0 - progress,
                            Vec3::new(1.0, -1.0, -1.0),
                        ),
                        _ => (Vec3::ZERO, 0.0, Vec3::new(1.0, -1.0, -1.0)),
                    },
                    AttackType::Overhead => match phase {
                        0 => (
                            Vec3::new(0.2, 0.8, 0.2).lerp(Vec3::new(0.0, 1.6, 0.2), progress),
                            1.0,
                            Vec3::new(1.0, 1.0, -1.0),
                        ),
                        1 => (
                            Vec3::new(0.0, 1.6, 0.2).lerp(Vec3::new(0.0, 0.1, -1.2), progress),
                            1.0,
                            Vec3::new(1.0, 1.0, -1.0),
                        ),
                        2 => (
                            Vec3::new(0.0, 0.1, -1.2).lerp(Vec3::ZERO, progress),
                            1.0 - progress,
                            Vec3::new(1.0, -1.0, -1.0),
                        ),
                        _ => (Vec3::ZERO, 0.0, Vec3::new(1.0, -1.0, -1.0)),
                    },
                    AttackType::UnarmedLeft => match phase {
                        0 => (
                            Vec3::new(-0.4, 0.35, 0.0).lerp(Vec3::new(-0.75, 0.70, 0.35), progress),
                            1.0,
                            Vec3::new(-1.2, 0.8, 0.4),
                        ),
                        1 => (
                            Vec3::new(-0.75, 0.70, 0.35).lerp(Vec3::new(0.35, 0.15, -1.05), progress),
                            1.0,
                            Vec3::new(-0.8, 0.3, -0.6),
                        ),
                        2 => (
                            Vec3::new(0.35, 0.15, -1.05).lerp(Vec3::ZERO, progress),
                            1.0 - progress,
                            Vec3::new(-1.0, -0.4, -0.5),
                        ),
                        _ => (Vec3::ZERO, 0.0, Vec3::new(-1.0, 1.0, -1.0)),
                    },
                    AttackType::UnarmedRight => match phase {
                        0 => (
                            Vec3::new(0.4, 0.35, 0.0).lerp(Vec3::new(0.75, 0.70, 0.35), progress),
                            1.0,
                            Vec3::new(1.2, 0.8, 0.4),
                        ),
                        1 => (
                            Vec3::new(0.75, 0.70, 0.35).lerp(Vec3::new(-0.35, 0.15, -1.05), progress),
                            1.0,
                            Vec3::new(0.8, 0.3, -0.6),
                        ),
                        2 => (
                            Vec3::new(-0.35, 0.15, -1.05).lerp(Vec3::ZERO, progress),
                            1.0 - progress,
                            Vec3::new(1.0, -0.4, -0.5),
                        ),
                        _ => (Vec3::ZERO, 0.0, Vec3::new(1.0, 1.0, -1.0)),
                    },
                }
            }
            CombatState::Recoiled { timer } => (
                Vec3::new(0.3, 0.8, 0.5),
                1.0 - timer.fraction(),
                Vec3::new(1.0, 0.0, 0.0),
            ),
        };

        let lerp_factor = (time.delta_seconds() * 25.0).clamp(0.0, 1.0);
        ik_target.weight = ik_target.weight.lerp(target_weight, lerp_factor);

        if ik_target.weight > 0.01 {
            ik_target.position = owner_transform.transform_point(target_local_pos);
            chain.pole_vector = target_pole.normalize_or_zero();
        }
    }
}

pub fn maintain_shield_orientation(
    owner_query: Query<(Entity, &IkTarget), With<GuardIntent>>,
    anchor_query: Query<(Entity, &Parent), With<ShieldVisualAnchor>>,
    mut transforms: Query<&mut Transform>,
    parents: Query<&Parent>,
) {
    for (owner_entity, ik_target) in owner_query.iter() {
        if ik_target.weight <= 0.01 {
            continue;
        }

        let owner_global = compute_manual_global_transform(owner_entity, &transforms, &parents);

        for (anchor_entity, anchor_parent) in anchor_query.iter() {
            let mut current_entity = anchor_parent.get();
            let mut found_owner = false;
            while let Ok(p) = parents.get(current_entity) {
                if current_entity == owner_entity {
                    found_owner = true;
                    break;
                }
                current_entity = p.get();
            }

            if found_owner || current_entity == owner_entity {
                let root_global =
                    compute_manual_global_transform(anchor_parent.get(), &transforms, &parents);
                let base_shield_rot = Quat::from_euler(EulerRot::XYZ, 0.0, 3.0, 3.0);

                let desired_global_rot = owner_global.rotation * base_shield_rot;
                let target_local_rot = root_global.rotation.inverse() * desired_global_rot;

                if let Ok(mut anchor_transform) = transforms.get_mut(anchor_entity) {
                    let safe_weight = ik_target.weight.clamp(0.0, 1.0);
                    anchor_transform.rotation = anchor_transform
                        .rotation
                        .slerp(target_local_rot, safe_weight);
                }
            }
        }
    }
}

pub fn update_shield_ik_target(
    mut query: Query<(
        &mut IkTarget,
        &mut TwoBoneIkChain,
        &GuardIntent,
        &BlockingState,
        &Transform,
        Option<&Player>,
    )>,
    lock_on: Res<LockOnState>,
    time: Res<Time>,
) {
    for (mut ik_target, mut chain, intent, state, transform, player_opt) in query.iter_mut() {
        let is_guarding = if player_opt.is_some() {
            lock_on.target.is_some() && matches!(*state, BlockingState::Blocking)
        } else {
            matches!(*state, BlockingState::Blocking)
        };

        let target_w = if is_guarding { 1.0 } else { 0.0 };
        let lerp_factor = (time.delta_seconds() * 15.0).clamp(0.0, 1.0);
        ik_target.weight = ik_target.weight.lerp(target_w, lerp_factor);

        if ik_target.weight > 0.01 {
            let local_base_pos = Vec3::new(-0.15, 0.3, -0.2);

            let offset_x = intent.offset.x * 0.8;
            let offset_y = -intent.offset.y * 0.8;
            let offset_z = offset_x.abs() * 0.3;

            let local_target = local_base_pos + Vec3::new(offset_x, offset_y, offset_z);
            ik_target.position = transform.transform_point(local_target);

            let local_pole = Vec3::new(-1.0, 0.0, 0.0);
            chain.pole_vector = local_pole.normalize_or_zero();
        }
    }
}

fn compute_manual_global_transform(
    entity: Entity,
    transforms: &Query<&mut Transform>,
    parents: &Query<&Parent>,
) -> Transform {
    let mut current_entity = entity;
    let mut global_transform = Transform::IDENTITY;
    let mut hierarchy = Vec::new();

    while let Ok(transform) = transforms.get(current_entity) {
        hierarchy.push(*transform);
        if let Ok(parent) = parents.get(current_entity) {
            current_entity = parent.get();
        } else {
            break;
        }
    }

    for transform in hierarchy.into_iter().rev() {
        global_transform = global_transform * transform;
    }
    global_transform
}

pub fn solve_two_bone_ik(
    chain_query: Query<(&TwoBoneIkChain, &IkTarget)>,
    mut transforms: Query<&mut Transform>,
    parent_query: Query<&Parent>,
) {
    for (chain, target) in chain_query.iter() {
        if target.weight <= 0.0 {
            continue;
        }

        let len1 = transforms
            .get(chain.mid)
            .map(|t| t.translation.length())
            .unwrap_or(0.001);
        let len2 = transforms
            .get(chain.tip)
            .map(|t| t.translation.length())
            .unwrap_or(0.001);
        let total_len = len1 + len2;

        let root_parent = parent_query
            .get(chain.root)
            .map(|p| p.get())
            .unwrap_or(chain.root);
        let root_parent_global =
            compute_manual_global_transform(root_parent, &transforms, &parent_query);

        let root_local = transforms.get(chain.root).cloned().unwrap_or_default();
        let root_global_pos = (root_parent_global * root_local).translation;

        let mut dir_target = target.position - root_global_pos;
        let mut dist_target = dir_target.length();
        let dir_target_norm = dir_target.normalize_or_zero();

        let d_max = total_len * 0.95;

        if dist_target > d_max {
            let diff = dist_target - d_max;
            let max_extend = total_len - d_max;
            dist_target = d_max + max_extend * (1.0 - (-diff / max_extend).exp());
        } else {
            dist_target = dist_target.max(0.001);
        }

        let global_pole_vector = root_parent_global.rotation * chain.pole_vector;
        let mut bend_normal = dir_target_norm
            .cross(global_pole_vector)
            .normalize_or_zero();

        if bend_normal == Vec3::ZERO {
            bend_normal = dir_target_norm.cross(Vec3::Y).normalize_or_zero();
        }
        if bend_normal == Vec3::ZERO {
            bend_normal = dir_target_norm.cross(Vec3::X).normalize_or_zero();
        }
        if bend_normal == Vec3::ZERO {
            bend_normal = Vec3::Z;
        }

        let mut cos_angle_root = ((len1 * len1) + (dist_target * dist_target) - (len2 * len2))
            / (2.0 * len1 * dist_target);
        if cos_angle_root.is_nan() {
            cos_angle_root = 1.0;
        }
        let angle_root = cos_angle_root.clamp(-1.0, 1.0).acos();

        let new_mid_dir = Quat::from_axis_angle(bend_normal, angle_root) * dir_target_norm;

        let root_global_rot = (root_parent_global * root_local).rotation;

        let default_bone_dir = transforms
            .get(chain.mid)
            .map(|t| t.translation.normalize_or_zero())
            .unwrap_or(Vec3::Y);
        let current_mid_global_dir = root_global_rot * default_bone_dir;

        let root_rot_diff = Quat::from_rotation_arc(current_mid_global_dir, new_mid_dir);
        let target_root_global_rot = root_rot_diff * root_global_rot;

        let target_root_local_rot = root_parent_global.rotation.inverse() * target_root_global_rot;
        let safe_weight = target.weight.clamp(0.0, 1.0);

        if let Ok(mut root_transform) = transforms.get_mut(chain.root) {
            root_transform.rotation = root_transform
                .rotation
                .slerp(target_root_local_rot, safe_weight);
        }

        let mut cos_angle_mid =
            ((len1 * len1) + (len2 * len2) - (dist_target * dist_target)) / (2.0 * len1 * len2);
        if cos_angle_mid.is_nan() {
            cos_angle_mid = 1.0;
        }
        let angle_mid = cos_angle_mid.clamp(-1.0, 1.0).acos();

        let bend_amount = std::f32::consts::PI - angle_mid;
        let mid_local_rot = Quat::from_axis_angle(Vec3::X, bend_amount);
        
        if let Ok(mut mid_transform) = transforms.get_mut(chain.mid) {
            let base_rot = mid_transform.rotation;
            mid_transform.rotation = base_rot.slerp(mid_local_rot, safe_weight);
        }
    }
}
