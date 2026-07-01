use bevy::prelude::*;
use super::events::{BlockEvent, RecoilEvent};
use super::systems::handle_shield_input;

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecoilEvent>();
        app.add_event::<BlockEvent>();
        
        // Sistemleri Update döngüsüne ekliyoruz
        app.add_systems(Update, (
            handle_shield_input,
            
        ).chain()); // Sıralı çalışması veri bütünlüğü için iyidir
    }
}