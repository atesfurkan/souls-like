use bevy::prelude::*;
use super::components::Health;
use super::events::{DamageEvent, HealEvent , DeathEvent};

pub fn process_damage(
    mut events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for ev in events.read() {
        // Eğer hasar alan entity'nin Health componenti varsa canını düşür
        if let Ok(mut health) = health_query.get_mut(ev.target) {
            health.current -= ev.amount;
        }
    }
}

// YENİ EKLENEN KISIM
pub fn process_heal(
    mut events: EventReader<HealEvent>,
    mut health_query: Query<&mut Health>,
) {
    for ev in events.read() {
        if let Ok(mut health) = health_query.get_mut(ev.target) {
            // Canı artır, ancak maksimum canı geçmesine izin verme
            health.current = (health.current + ev.amount).min(health.max);
        }
    }
}

pub fn check_death(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
    mut death_writer: EventWriter<DeathEvent>,
) {
    for (entity, health) in query.iter() {
        if health.current <= 0.0 {
            death_writer.send(DeathEvent { target: entity });
            // Canı sıfırlanan entity'i ve tüm çocuklarını (varsa silahı, zırhı vs) dünyadan sil
            commands.entity(entity).despawn_recursive();
        }
    }
}
