use bevy::animation::animate_targets;
use bevy::prelude::*;
use bevy::transform::TransformSystem;

use crate::game::ik::systems::{
    maintain_shield_orientation, maintain_weapon_orientation, solve_two_bone_ik,
    update_shield_ik_target, update_weapon_ik_target,
};

pub struct IkPlugin;

impl Plugin for IkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                // 1. Önce IK hedeflerini ve ağırlıklarını güncelle
                update_shield_ik_target,
                update_weapon_ik_target,
                // 2. Ardından kemik rotasyonlarını hesaplayıp Transform'lara yaz
                solve_two_bone_ik,
                maintain_shield_orientation,
                maintain_weapon_orientation,
            )
                .chain()
                // Bevy 0.14: Önce animasyon karesi kemiklere uygulansın (animate_targets)
                .after(animate_targets)
                // Daha sonra IK sonuçları dünyadaki nihai global matrislere aktarılsın (TransformPropagate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}