use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Poise {
    pub current: f32,
    pub max: f32,
}

impl Default for Poise {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}
