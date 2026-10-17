use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct Chest;

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct Stalactite;

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct Tree;