use bevy::prelude::*;
use super::systems::{
    cache_character_bones, 
    load_animations, 
    setup_animation_graph, 
    update_animations_from_intent
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_animations)
           .add_systems(Update, (
               cache_character_bones,
               setup_animation_graph, // Eski play_default_animation yerine bu geldi
               update_animations_from_intent, // Niyete göre animasyon değiştiren sistem
           ));
    }
}
