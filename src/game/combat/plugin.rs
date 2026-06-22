use bevy::prelude::*;
use super::systems::{handle_attacks, process_attack_states , hit_detection ,animate_weapon }; // hit detection ekledik lan

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // Sistemlerimizi sıraya koyuyoruz
        app.add_systems(Update, (
            handle_attacks, 
            process_attack_states,
            hit_detection, 
            animate_weapon // YENİ EKLENDİ
        ).chain());
    }
}
