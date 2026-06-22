use bevy::prelude::*;
use bevy::utils::HashSet;


// Silahın istatistiklerini tutan data-driven bileşen
#[derive(Component)]
pub struct Weapon {
    pub windup_duration: f32,
    pub active_duration: f32,
    pub recovery_duration: f32,
    pub stamina_cost: f32,
    // Hitbox ve Hasar Ayarları
    pub damage: f32,
    pub hitbox_radius: f32,
    pub hitbox_offset: Vec3,
}

// Oyuncunun savaş durumunu tutan State Machine
#[derive(Component, Default)]
pub enum CombatState {
    #[default]
    Idle,
    Windup(Timer),
    Active(Timer , Option<Entity> ),
    Recovery(Timer),
}

// Dünyada geçici olarak var olacak Hitbox bileşeni
#[derive(Component)]
pub struct Hitbox {
    pub damage: f32,
    pub owner: Entity, // Vuran kişi (kendine vurmasın diye)
    pub hit_entities: HashSet<Entity>, // Bu saldırıda daha önce vurulanların listesi
}

#[derive(Component)]
pub struct EquippedWeapon;
