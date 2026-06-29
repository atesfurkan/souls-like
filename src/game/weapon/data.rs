use bevy::utils::HashMap;
use bevy::prelude::*; // Vec3 için gerekli
use crate::game::weapon::components::{Weapon, AttackDefinition, 
    AttackType , WeaponVisuals , WeaponPart};

const WOOD_COLOR: Color = Color::srgb(0.4, 0.2, 0.1);
const METAL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponType {
    Spear,
}

pub fn get_weapon_data(weapon_type: WeaponType) -> Weapon {
    match weapon_type {
        WeaponType::Spear => Weapon {
            attacks: HashMap::from([
                (AttackType::Thrust, AttackDefinition {
                    windup_duration: 0.3, active_duration: 0.2, recovery_duration: 0.5,
                    stamina_cost: 15.0, damage: 25.0, hitbox_radius: 0.3, hitbox_offset: Vec3::new(0.0, 0.8, -1.5),
                }),
                (AttackType::Overhead, AttackDefinition {
                    windup_duration: 0.5, active_duration: 0.2, recovery_duration: 0.7,
                    stamina_cost: 25.0, damage: 40.0, hitbox_radius: 0.4, hitbox_offset: Vec3::new(0.0, 0.5, -1.0),
                }),
                (AttackType::LeftSwing, AttackDefinition {
                    windup_duration: 0.4, active_duration: 0.2, recovery_duration: 0.6,
                    stamina_cost: 20.0, damage: 20.0, hitbox_radius: 0.6, hitbox_offset: Vec3::new(0.0, 1.0, -0.8),
                }),
                (AttackType::RightSwing, AttackDefinition {
                    windup_duration: 0.4, active_duration: 0.2, recovery_duration: 0.6,
                    stamina_cost: 20.0, damage: 20.0, hitbox_radius: 0.6, hitbox_offset: Vec3::new(0.0, 1.0, -0.8),
                }),
            ]),
            visuals: WeaponVisuals {
        parts: vec![
            (WeaponPart::Cylinder { radius: 0.05, height: 2.0, color: WOOD_COLOR }, Transform::IDENTITY),
            (WeaponPart::Cone { radius: 0.1, height: 0.4, color: METAL_COLOR }, Transform::from_xyz(0.0, 1.2, 0.0)),
        ],
    },
        },
        _ => panic!("Silah ismi bulunamadı!"),
    }
}