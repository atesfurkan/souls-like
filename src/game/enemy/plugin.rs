use bevy::prelude::*;
use crate::game::enemy::spawn::{spawn_enemy, spawn_test_enemies};
use crate::game::enemy::systems::{enemy_ai_system, handle_enemy_despawn};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_enemies)
           .add_systems(
               Update,
               (
                   spawn_enemy,
                   enemy_ai_system,
                   handle_enemy_despawn, // YENİ: Runtime State Retention & Despawn Sistemi
               ),
           );
    }
}