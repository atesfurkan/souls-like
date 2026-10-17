use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::animation::components::CharacterBones;
use crate::game::combat::components::{
    AttachedBoneHurtbox, CharacterHurtboxProfile, HitRegion, Hurtbox, HurtboxRoot,
};

pub fn spawn_anatomical_hurtboxes(
    mut commands: Commands,
    mut query: Query<(Entity, &CharacterBones, &CharacterHurtboxProfile, &mut HurtboxRoot)>,
) {
    for (owner, bones, profile, mut root) in query.iter_mut() {
        if !bones.is_linked || root.is_spawned {
            continue;
        }

        let is_monster = profile.is_monster;

        let arm_radius = if is_monster { 0.12 } else { 0.06 };
        let forearm_radius = if is_monster { 0.11 } else { 0.055 };
        let hand_radius = if is_monster { 0.16 } else { 0.07 };

        let rot_left_arm = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2);
        let rot_right_arm = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
        let rot_identity = Quat::IDENTITY;
        let rot_foot = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);

        let bone_specs: [(Option<Entity>, HitRegion, Collider, Vec3, Quat); 15] = [
            (
                bones.head,
                HitRegion::Head,
                Collider::ball(0.12),
                Vec3::new(0.0, 0.10, 0.02),
                rot_identity,
            ),
            (
                bones.spine,
                HitRegion::Torso,
                Collider::capsule_y(0.12, 0.13),
                Vec3::new(0.0, 0.08, 0.0),
                rot_identity,
            ),
            (
                bones.hips,
                HitRegion::Pelvis,
                Collider::capsule_y(0.06, 0.12),
                Vec3::ZERO,
                rot_identity,
            ),
            (
                bones.left_arm,
                HitRegion::LeftUpperArm,
                Collider::capsule_y(0.09, arm_radius),
                Vec3::new(0.11, 0.0, 0.0),
                rot_left_arm,
            ),
            (
                bones.left_forearm,
                HitRegion::LeftForearm,
                Collider::capsule_y(0.09, forearm_radius),
                Vec3::new(0.10, 0.0, 0.0),
                rot_left_arm,
            ),
            (
                bones.left_hand,
                HitRegion::LeftHand,
                Collider::ball(hand_radius),
                Vec3::new(0.05, 0.0, 0.0),
                rot_identity,
            ),
            (
                bones.right_arm,
                HitRegion::RightUpperArm,
                Collider::capsule_y(0.09, arm_radius),
                Vec3::new(-0.11, 0.0, 0.0),
                rot_right_arm,
            ),
            (
                bones.right_forearm,
                HitRegion::RightForearm,
                Collider::capsule_y(0.09, forearm_radius),
                Vec3::new(-0.10, 0.0, 0.0),
                rot_right_arm,
            ),
            (
                bones.right_hand,
                HitRegion::RightHand,
                Collider::ball(hand_radius),
                Vec3::new(-0.05, 0.0, 0.0),
                rot_identity,
            ),
            (
                bones.left_thigh,
                HitRegion::LeftThigh,
                Collider::capsule_y(0.14, 0.08),
                Vec3::new(0.0, -0.16, 0.0),
                rot_identity,
            ),
            (
                bones.left_shin,
                HitRegion::LeftShin,
                Collider::capsule_y(0.14, 0.07),
                Vec3::new(0.0, -0.16, 0.0),
                rot_identity,
            ),
            (
                bones.left_foot,
                HitRegion::LeftFoot,
                Collider::capsule_y(0.06, 0.04),
                Vec3::new(0.0, -0.04, 0.05),
                rot_foot,
            ),
            (
                bones.right_thigh,
                HitRegion::RightThigh,
                Collider::capsule_y(0.14, 0.08),
                Vec3::new(0.0, -0.16, 0.0),
                rot_identity,
            ),
            (
                bones.right_shin,
                HitRegion::RightShin,
                Collider::capsule_y(0.14, 0.07),
                Vec3::new(0.0, -0.16, 0.0),
                rot_identity,
            ),
            (
                bones.right_foot,
                HitRegion::RightFoot,
                Collider::capsule_y(0.06, 0.04),
                Vec3::new(0.0, -0.04, 0.05),
                rot_foot,
            ),
        ];

        for (bone_opt, region, collider, offset, rotation_offset) in bone_specs {
            if let Some(bone_entity) = bone_opt {
                commands.spawn((
                    SpatialBundle::default(),
                    collider,
                    Sensor,
                    Hurtbox { owner, region },
                    AttachedBoneHurtbox {
                        bone: bone_entity,
                        offset,
                        rotation_offset,
                    },
                ));
            }
        }

        root.is_spawned = true;
    }
}

pub fn sync_bone_hurtboxes(
    mut hurtbox_query: Query<(Entity, &AttachedBoneHurtbox, &Hurtbox, &mut Transform)>,
    bone_query: Query<&GlobalTransform>,
    owner_query: Query<Entity>,
    mut commands: Commands,
) {
    for (hurtbox_entity, attached, hurtbox, mut transform) in hurtbox_query.iter_mut() {
        if owner_query.get(hurtbox.owner).is_err() {
            commands.entity(hurtbox_entity).despawn();
            continue;
        }

        if let Ok(bone_global) = bone_query.get(attached.bone) {
            let bone_transform = bone_global.compute_transform();
            let final_rotation = bone_transform.rotation * attached.rotation_offset;
            let final_translation =
                bone_transform.translation + bone_transform.rotation * attached.offset;

            transform.translation = final_translation;
            transform.rotation = final_rotation;
            transform.scale = Vec3::ONE;
        }
    }
}