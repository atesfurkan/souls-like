use bevy::prelude::*;
use super::events::{DamageEvent, HealEvent, DeathEvent};
use super::systems::{check_death, process_damage , process_heal};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        // Event'i sisteme kaydediyoruz
        app.add_event::<DamageEvent>()
           // Yeni event'i kaydediyoruz
           .add_event::<HealEvent>()
           // Zincir: Hasar İşle -> İyileşmeyi İşle -> Ölüm Kontrolü yap 
           .add_event::<DeathEvent>()
           .add_systems(Update, (process_damage, process_heal, check_death).chain());
    }
}
