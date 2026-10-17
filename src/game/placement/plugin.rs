use bevy::prelude::*;
use super::components::{Placement, PlacementConfig};

pub struct PlacementPlugin;

impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        // İleride save/load sistemleri ve editor için register işlemi
        app.register_type::<Placement>();
        
        // Yoğunluk ayarlarımızı Resource olarak başlatıyoruz
        app.init_resource::<PlacementConfig>();
    }
}
