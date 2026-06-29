use bevy::prelude::*;
use crate::game::weapon::systems::auto_spawn_weapon_visuals;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        // Silahların görsel parçalarını otomatik oluşturan sistem
        app.add_systems(Update, auto_spawn_weapon_visuals);
        
        // İleride silahların dayanıklılığı, envanter yönetimi veya 
        // başka silah mekanikleri eklemek isterseniz bu plugin 
        // çatısı altında kolayca ekleyebilirsiniz.
    }
}
