use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TriplanarMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub wall_texture: Handle<Image>,

    #[texture(2)]
    #[sampler(3)]
    pub floor_texture: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    pub ceiling_texture: Handle<Image>,
}

impl Material for TriplanarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/triplanar.wgsl".into()
    }
}