use bevy::prelude::*;

// Hangi varlığa kilitlendiğimizi tutan global durum
#[derive(Resource, Default)]
pub struct LockOnState {
    pub target: Option<Entity>,
}

// Kilitlenilebilir nesneleri (düşmanları) belirten işaretçi
#[derive(Component)]
pub struct Lockable;
