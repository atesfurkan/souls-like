use bevy::prelude::*;
use super::equipment_menu::plugin::EquipmentMenuPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // İleride Inventory, Crafting vb. eklendiğinde buraya alt pluginler olarak eklenecek.
        app.add_plugins(EquipmentMenuPlugin);
    }
}
