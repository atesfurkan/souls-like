use bevy::prelude::*;
use crate::game::combat::components::{CombatState, EquippedWeapon};
use crate::game::weapon::components::AttackType;

pub fn animate_weapon(
    entity_query: Query<(&CombatState, &Children)>,
    mut weapon_query: Query<&mut Transform, With<EquippedWeapon>>,
) {
    for (state, children) in entity_query.iter() {
        let Some(&weapon_entity) = children.iter().find(|&&child| weapon_query.contains(child)) else {
            continue;
        };
        let Ok(mut transform) = weapon_query.get_mut(weapon_entity) else {
            continue;
        };

        let idle_pos = Vec3::new(0.6, 0.0, -0.5);
        let idle_rot = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4);

        match state {
            CombatState::Idle => {
                transform.translation = idle_pos;
                transform.rotation = idle_rot;
            }
            CombatState::Windup { timer, attack_type } => {
                let t = timer.fraction();
                let (windup_pos, windup_rot) = get_windup_transform(*attack_type);
                transform.translation = idle_pos.lerp(windup_pos, t);
                transform.rotation = idle_rot.slerp(windup_rot, t);
            }
            CombatState::Active { timer, attack_type, .. } => {
                let t = timer.fraction();
                let (windup_pos, windup_rot) = get_windup_transform(*attack_type);
                let (active_pos, active_rot) = get_active_transform(*attack_type);
                transform.translation = windup_pos.lerp(active_pos, t);
                transform.rotation = windup_rot.slerp(active_rot, t);
            }
            CombatState::Recovery { timer, attack_type } => {
                let t = timer.fraction();
                let (active_pos, active_rot) = get_active_transform(*attack_type);
                transform.translation = active_pos.lerp(idle_pos, t);
                transform.rotation = active_rot.slerp(idle_rot, t);
            }
            CombatState::Recoiled { timer } => {
                let t = timer.fraction();
                let recoil_pos = Vec3::new(0.5, 1.2, 0.5);
                let recoil_rot = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4) * Quat::from_rotation_z(0.5);
                transform.translation = recoil_pos.lerp(idle_pos, t);
                transform.rotation = recoil_rot.slerp(idle_rot, t);
            }
        }
    }
}

fn get_windup_transform(attack: AttackType) -> (Vec3, Quat) {
    match attack {
        AttackType::Thrust => (Vec3::new(0.6, 0.5, 0.2), Quat::from_rotation_x(-0.1)),
        AttackType::Overhead => (Vec3::new(0.0, 1.5, 0.0), Quat::from_rotation_x(0.5)),
        AttackType::LeftSwing => (
            Vec3::new(-0.8, 0.5, -0.2),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_4),
        ),
        AttackType::RightSwing => (
            Vec3::new(1.2, 0.5, -0.2),
            Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4),
        ),
    }
}

fn get_active_transform(attack: AttackType) -> (Vec3, Quat) {
    match attack {
        AttackType::Thrust => (
            Vec3::new(0.0, 0.8, -1.3),
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
        AttackType::Overhead => (
            Vec3::new(0.0, 0.2, -1.2),
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
        AttackType::LeftSwing => (
            Vec3::new(1.0, 0.5, -1.0),
            Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2) * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
        AttackType::RightSwing => (
            Vec3::new(-1.0, 0.5, -1.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2) * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
    }
}
