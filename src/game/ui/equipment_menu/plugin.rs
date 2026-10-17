use bevy::prelude::*;
use super::components::{MenuState, ActiveSelectionSlot}; // ActiveSelectionSlot'u import etmeyi unutma!
use super::systems::{
    toggle_menu_state, build_equipment_menu, destroy_equipment_menu,
    update_equipment_texts, handle_menu_interactions,
    unlock_cursor_on_menu_open, lock_cursor_on_menu_close,
};

pub struct EquipmentMenuPlugin;

impl Plugin for EquipmentMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .init_resource::<ActiveSelectionSlot>() // İŞTE BURAYA EKLİYORUZ
            .add_systems(Update, toggle_menu_state)
            
            // Menü AÇILDIĞINDA
            .add_systems(OnEnter(MenuState::EquipmentOpen), (
                build_equipment_menu,
                unlock_cursor_on_menu_open,
            ))
            
            // Menü KAPANDIĞINDA
            .add_systems(OnExit(MenuState::EquipmentOpen), (
                destroy_equipment_menu,
                lock_cursor_on_menu_close,
            ))
            
            // Sadece menü açıkken çalışacak sistemler
            .add_systems(
                Update,
                (
                    update_equipment_texts,
                    handle_menu_interactions,
                ).run_if(in_state(MenuState::EquipmentOpen))
            );
    }
}