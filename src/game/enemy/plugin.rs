use bevy::prelude::*;
use super::systems::enemy_ai_system;
use super::spawn::spawn_enemy;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy)
           .add_systems(Update, enemy_ai_system);
    }
}
