use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

// Stagger durumu kaldırıldı, ortak CombatState veya Poise sistemi bunu yönetecek.
#[derive(Component, Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum EnemyAIState {
    #[default]
    Idle,
    Chase,
    Attack,
    Dead,
}

// StaggerState struct'ı tamamen silindi.

#[derive(Component)]
pub struct EnemyAI {
    pub detection_range: f32,
    pub attack_range: f32,
    pub speed: f32,
    pub attack_cooldown: Timer,
}