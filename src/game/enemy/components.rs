use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

// YENİ: FSM Durumları
#[derive(Component, Default, PartialEq, Eq, Clone, Copy ,Debug)]
pub enum EnemyAIState {
    #[default]
    Idle,
    Chase,
    Attack,
    Dead,
    Stagger,
}


#[derive(Component)]
pub struct StaggerState {
    pub timer: Timer,
}

// YENİ: Düşman Yapay Zeka Ayarları (Data-driven esneklik sağlar)
#[derive(Component)]
pub struct EnemyAI {
    pub detection_range: f32,
    pub attack_range: f32,
    pub speed: f32,
    pub attack_cooldown: Timer,
}
