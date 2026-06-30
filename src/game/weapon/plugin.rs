use bevy::prelude::*;
use crate::game::weapon::systems::{auto_spawn_weapon_visuals, process_weapon_swap};
use crate::game::weapon::events::WeaponSwapRequestEvent;
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        // Event'i register ediyoruz
        app.add_event::<WeaponSwapRequestEvent>();
        
        // Silahların görsel parçalarını otomatik oluşturan ve değişimleri yöneten sistemler 
        app.add_systems(Update, (
            auto_spawn_weapon_visuals,
            process_weapon_swap,
        ));
    }
}
