use bevy::prelude::*;
use bevy::utils::HashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackType {
    Thrust,
    Overhead,
    LeftSwing,
    RightSwing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponType {
    Spear,
    VampireBlade,
}

// Tek bir saldırının tüm özellikleri
#[derive(Clone)]
pub struct AttackDefinition {
    pub windup_duration: f32,
    pub active_duration: f32,
    pub recovery_duration: f32,
    pub stamina_cost: f32,
    pub damage: f32,
    pub hitbox_radius: f32,
    pub hitbox_offset: Vec3,
    pub lifesteal: f32, // YENİ: Vuruş başına kazanılacak can miktarı
}

#[derive(Component)]
pub struct Weapon {
    pub name: String,              // YENİ: HUD için silahın görünen adı
    pub weapon_type: WeaponType,   // YENİ: Mantıksal kontroller için silah tipi
    pub attacks: HashMap<AttackType, AttackDefinition>,
    pub visuals: WeaponVisuals,
}

pub enum WeaponPart {
    Cylinder { radius: f32, height: f32, color: Color },
    Cone { radius: f32, height: f32, color: Color },
    // İleride 3D model eklediğimizde buraya şunu ekleyeceğiz:
    // GltfModel { path: String },
}

pub struct WeaponVisuals {
    pub parts: Vec<(WeaponPart, Transform)>, // Hangi parça nerede duracak?
}

#[derive(Component)]
pub struct WeaponChildrenCreated;