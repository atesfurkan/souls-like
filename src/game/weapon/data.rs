use bevy::utils::HashMap;
use bevy::prelude::*; // Vec3 için gerekli
use crate::game::weapon::components::{Weapon, AttackDefinition, 
    AttackType , WeaponVisuals ,
     WeaponPart , WeaponType};

const WOOD_COLOR: Color = Color::srgb(0.4, 0.2, 0.1);
const METAL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const BLOOD_RED: Color = Color::srgb(0.8, 0.1, 0.1);



pub fn get_weapon_data(weapon_type: WeaponType) -> Weapon {
    match weapon_type {
        WeaponType::Spear => Weapon {
            name: "Guard Spear".to_string(), // YENİ: UI'da görünecek havalı isim
            weapon_type: WeaponType::Spear,       // YENİ: Enum referansı
            attacks: HashMap::from([
                (AttackType::Thrust, AttackDefinition {
                    windup_duration: 0.3, active_duration: 0.2, recovery_duration: 0.5,
                    stamina_cost: 15.0, damage: 25.0, hitbox_radius: 0.3, hitbox_offset: Vec3::new(0.0, 0.8, -1.5),
                    lifesteal: 0.0, // Mevcut silahlarda lifesteal yok
                }),
                (AttackType::Overhead, AttackDefinition {
                    windup_duration: 0.5, active_duration: 0.2, recovery_duration: 0.7,
                    stamina_cost: 25.0, damage: 40.0, hitbox_radius: 0.4, hitbox_offset: Vec3::new(0.0, 0.5, -1.0),
                    lifesteal: 0.0,
                }),
                (AttackType::LeftSwing, AttackDefinition {
                    windup_duration: 0.4, active_duration: 0.2, recovery_duration: 0.6,
                    stamina_cost: 20.0, damage: 20.0, hitbox_radius: 0.6, hitbox_offset: Vec3::new(0.0, 1.0, -0.8),
                    lifesteal: 0.0,
                }),
                (AttackType::RightSwing, AttackDefinition {
                    windup_duration: 0.4, active_duration: 0.2, recovery_duration: 0.6,
                    stamina_cost: 20.0, damage: 20.0, hitbox_radius: 0.6, hitbox_offset: Vec3::new(0.0, 1.0, -0.8),
                    lifesteal: 0.0,
                }),
            ]),
            visuals: WeaponVisuals {
                parts: vec![
                    (WeaponPart::Cylinder { radius: 0.05, height: 2.0, color: WOOD_COLOR }, Transform::IDENTITY),
                    (WeaponPart::Cone { radius: 0.1, height: 0.4, color: METAL_COLOR }, Transform::from_xyz(0.0, 1.2, 0.0)),
                ],
            },
        },
        
        // YENİ: Lifesteal özelliğine sahip silahımız
        WeaponType::VampireBlade => Weapon {
            name: "Vampire Blade".to_string(), // YENİ: UI'da görünecek havalı isim
            weapon_type: WeaponType::VampireBlade,       // YENİ: Enum referansı
            attacks: HashMap::from([
                (AttackType::RightSwing, AttackDefinition {
                    windup_duration: 0.4, active_duration: 0.2, recovery_duration: 0.6,
                    stamina_cost: 20.0, damage: 15.0, hitbox_radius: 0.5, hitbox_offset: Vec3::new(0.0, 1.0, -1.0),
                    lifesteal: 5.0, // Başarılı vuruş başına 5 birim can çalar
                }),
                (AttackType::Thrust, AttackDefinition {
                    windup_duration: 0.3, active_duration: 0.2, recovery_duration: 0.5,
                    stamina_cost: 15.0, damage: 20.0, hitbox_radius: 0.3, hitbox_offset: Vec3::new(0.0, 0.8, -1.5),
                    lifesteal: 4.0, // Başarılı vuruş başına 5 birim can çalar
                }),
            ]),
            visuals: WeaponVisuals {
                parts: vec![
                    // Görsel olarak farklılaşması için kan kırmızısı ve biraz daha kısa bir silindir
                    (WeaponPart::Cylinder { radius: 0.06, height: 1.5, color: BLOOD_RED }, Transform::IDENTITY),
                    (WeaponPart::Cone { radius: 0.12, height: 0.5, color: METAL_COLOR }, Transform::from_xyz(0.0, 1.0, 0.0)),
                ],
            },
        },
        _ => panic!("Silah ismi bulunamadı!"),
    }
}