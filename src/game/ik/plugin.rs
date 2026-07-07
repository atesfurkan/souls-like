use bevy::prelude::*;
use bevy::transform::TransformSystem; // Bunu import etmen şart
use crate::game::ik::systems::{solve_two_bone_ik, update_shield_ik_target, maintain_shield_orientation};
pub struct IkPlugin;

impl Plugin for IkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate, 
            (
                // Önce hedefi belirle
                update_shield_ik_target,
                // Sonra kemikleri bük
                
                solve_two_bone_ik,
                maintain_shield_orientation
            )
            .chain() // Birbiri ardına çalışmasını garantile
            // EN ÖNEMLİ SATIR: Bevy dünyayı çizmeden hemen önce araya gir!
            .before(TransformSystem::TransformPropagate) 
        );
    }
}