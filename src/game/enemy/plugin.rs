use bevy::prelude::*;
use super::systems:: {spawn_enemy , enemy_ai_system };

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy)
           .add_systems(Update, enemy_ai_system);
    }
}
