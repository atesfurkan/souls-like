use crate::game::combat::components::UnarmedConfig;
use crate::game::weapon::components::{
    AttackDefinition, AttackType, Weapon, WeaponPart, WeaponType, WeaponVisuals,
};
use bevy::prelude::*;
use bevy::utils::HashMap;

const WOOD_COLOR: Color = Color::srgb(0.4, 0.2, 0.1);
const METAL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const BLOOD_RED: Color = Color::srgb(0.8, 0.1, 0.1);

pub fn get_unarmed_config() -> UnarmedConfig {
    UnarmedConfig {
        attacks: HashMap::from([
            (
                AttackType::UnarmedLeft,
                AttackDefinition {
                    windup_duration: 0.2,
                    active_duration: 0.25,
                    recovery_duration: 0.3,
                    max_charge_duration: 1.0,
                    stamina_cost: 10.0,
                    damage: 10.0,
                    poise_damage: 15.0,
                    hitbox_radius: 0.4,
                    hitbox_offset: Vec3::ZERO,
                    lifesteal: 0.0,
                },
            ),
            (
                AttackType::UnarmedRight,
                AttackDefinition {
                    windup_duration: 0.2,
                    active_duration: 0.25,
                    recovery_duration: 0.3,
                    max_charge_duration: 1.0,
                    stamina_cost: 10.0,
                    damage: 10.0,
                    poise_damage: 15.0,
                    hitbox_radius: 0.4,
                    hitbox_offset: Vec3::ZERO,
                    lifesteal: 0.0,
                },
            ),
        ]),
    }
}

pub fn get_weapon_data(weapon_type: WeaponType) -> Weapon {
    match weapon_type {
        WeaponType::Spear => Weapon {
            name: "Guard Spear".to_string(),
            weapon_type: WeaponType::Spear,
            attacks: HashMap::from([
                (
                    AttackType::Thrust,
                    AttackDefinition {
                        windup_duration: 0.3,
                        active_duration: 0.2,
                        recovery_duration: 0.5,
                        max_charge_duration: 5.0,
                        stamina_cost: 15.0,
                        damage: 100.0,
                        poise_damage: 100.0,
                        hitbox_radius: 0.4,
                        hitbox_offset: Vec3::new(0.0, 1.2, 0.0),
                        lifesteal: 0.0,
                    },
                ),
                (
                    AttackType::Overhead,
                    AttackDefinition {
                        windup_duration: 0.5,
                        active_duration: 0.2,
                        recovery_duration: 0.7,
                        max_charge_duration: 3.0,
                        stamina_cost: 25.0,
                        damage: 40.0,
                        poise_damage: 15.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 0.8, 0.0),
                        lifesteal: 0.0,
                    },
                ),
                (
                    AttackType::LeftSwing,
                    AttackDefinition {
                        windup_duration: 0.4,
                        active_duration: 0.2,
                        recovery_duration: 0.6,
                        max_charge_duration: 3.0,
                        stamina_cost: 20.0,
                        damage: 20.0,
                        poise_damage: 15.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 0.8, 0.0),
                        lifesteal: 0.0,
                    },
                ),
                (
                    AttackType::RightSwing,
                    AttackDefinition {
                        windup_duration: 0.4,
                        active_duration: 0.2,
                        recovery_duration: 0.6,
                        max_charge_duration: 3.0,
                        stamina_cost: 20.0,
                        damage: 20.0,
                        poise_damage: 15.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 0.8, 0.0),
                        lifesteal: 0.0,
                    },
                ),
            ]),
            visuals: WeaponVisuals {
                parts: vec![
                    (
                        WeaponPart::Cylinder {
                            radius: 0.05,
                            height: 2.0,
                            color: WOOD_COLOR,
                        },
                        Transform::IDENTITY,
                    ),
                    (
                        WeaponPart::Cone {
                            radius: 0.1,
                            height: 0.4,
                            color: METAL_COLOR,
                        },
                        Transform::from_xyz(0.0, 1.2, 0.0),
                    ),
                ],
            },
        },

        WeaponType::VampireBlade => Weapon {
            name: "Vampire Blade".to_string(),
            weapon_type: WeaponType::VampireBlade,
            attacks: HashMap::from([
                (
                    AttackType::Thrust,
                    AttackDefinition {
                        windup_duration: 0.3,
                        active_duration: 0.2,
                        recovery_duration: 0.5,
                        max_charge_duration: 3.0,
                        stamina_cost: 15.0,
                        damage: 20.0,
                        poise_damage: 25.0,
                        hitbox_radius: 0.4,
                        hitbox_offset: Vec3::new(0.0, 1.2, 0.0),
                        lifesteal: 8.0,
                    },
                ),
                (
                    AttackType::Overhead,
                    AttackDefinition {
                        windup_duration: 0.5,
                        active_duration: 0.2,
                        recovery_duration: 0.7,
                        max_charge_duration: 3.0,
                        stamina_cost: 25.0,
                        damage: 40.0,
                        poise_damage: 25.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 0.8, 0.0),
                        lifesteal: 10.0,
                    },
                ),
                (
                    AttackType::LeftSwing,
                    AttackDefinition {
                        windup_duration: 0.4,
                        active_duration: 0.2,
                        recovery_duration: 0.6,
                        max_charge_duration: 3.0,
                        stamina_cost: 20.0,
                        damage: 15.0,
                        poise_damage: 15.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 0.8, 0.0),
                        lifesteal: 5.0,
                    },
                ),
                (
                    AttackType::RightSwing,
                    AttackDefinition {
                        windup_duration: 0.4,
                        active_duration: 0.2,
                        recovery_duration: 0.6,
                        max_charge_duration: 3.0,
                        stamina_cost: 20.0,
                        damage: 15.0,
                        poise_damage: 15.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 0.8, 0.0),
                        lifesteal: 5.0,
                    },
                ),
            ]),
            visuals: WeaponVisuals {
                parts: vec![
                    (
                        WeaponPart::Cylinder {
                            radius: 0.06,
                            height: 1.5,
                            color: BLOOD_RED,
                        },
                        Transform::IDENTITY,
                    ),
                    (
                        WeaponPart::Cone {
                            radius: 0.12,
                            height: 0.5,
                            color: METAL_COLOR,
                        },
                        Transform::from_xyz(0.0, 1.0, 0.0),
                    ),
                ],
            },
        },

        WeaponType::Hammer => Weapon {
            name: "Agir Silah".to_string(),
            weapon_type: WeaponType::Hammer,
            attacks: HashMap::from([
                (
                    AttackType::Thrust,
                    AttackDefinition {
                        windup_duration: 0.6,
                        active_duration: 0.3,
                        recovery_duration: 0.8,
                        max_charge_duration: 3.0,
                        stamina_cost: 30.0,
                        damage: 35.0,
                        poise_damage: 30.0,
                        hitbox_radius: 0.5,
                        hitbox_offset: Vec3::new(0.0, 1.2, 0.0),
                        lifesteal: 0.0,
                    },
                ),
                (
                    AttackType::Overhead,
                    AttackDefinition {
                        windup_duration: 0.8,
                        active_duration: 0.3,
                        recovery_duration: 1.0,
                        max_charge_duration: 3.0,
                        stamina_cost: 40.0,
                        damage: 60.0,
                        poise_damage: 60.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 1.2, 0.0),
                        lifesteal: 0.0,
                    },
                ),
                (
                    AttackType::LeftSwing,
                    AttackDefinition {
                        windup_duration: 0.7,
                        active_duration: 0.3,
                        recovery_duration: 0.9,
                        max_charge_duration: 3.0,
                        stamina_cost: 35.0,
                        damage: 45.0,
                        poise_damage: 45.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 1.2, 0.0),
                        lifesteal: 0.0,
                    },
                ),
                (
                    AttackType::RightSwing,
                    AttackDefinition {
                        windup_duration: 0.7,
                        active_duration: 0.3,
                        recovery_duration: 0.9,
                        max_charge_duration: 3.0,
                        stamina_cost: 35.0,
                        damage: 45.0,
                        poise_damage: 45.0,
                        hitbox_radius: 0.6,
                        hitbox_offset: Vec3::new(0.0, 1.2, 0.0),
                        lifesteal: 0.0,
                    },
                ),
            ]),
            visuals: WeaponVisuals {
                parts: vec![
                    (
                        WeaponPart::Cylinder {
                            radius: 0.05,
                            height: 1.2,
                            color: Color::srgb(0.35, 0.25, 0.15),
                        },
                        Transform::from_xyz(0.0, 0.6, 0.0),
                    ),
                    (
                        WeaponPart::Cuboid {
                            x: 0.3,
                            y: 0.4,
                            z: 0.5,
                            color: Color::srgb(0.4, 0.4, 0.45),
                        },
                        Transform::from_xyz(0.0, 1.2, 0.0),
                    ),
                ],
            },
        },
    }
}