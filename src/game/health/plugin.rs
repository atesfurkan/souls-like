use bevy::prelude::*;
use super::events::DamageEvent;
use super::systems::{check_death, process_damage};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        // Event'i sisteme kaydediyoruz
        app.add_event::<DamageEvent>()
           // chain() ile önce hasarın düşülmesini, hemen ardından ölüm kontrolü yapılmasını garanti ediyoruz
           .add_systems(Update, (process_damage, check_death).chain());
    }
}
