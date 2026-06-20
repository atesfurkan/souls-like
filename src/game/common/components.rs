use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera {
    pub distance: f32,
    pub pitch: f32,
    pub yaw: f32,
}
