use bevy::prelude::*;
use super::components::Health;
use super::events::DamageEvent;

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

pub fn check_death(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.current <= 0.0 {
            // Canı sıfırlanan entity'i ve tüm çocuklarını (varsa silahı, zırhı vs) dünyadan sil
            commands.entity(entity).despawn_recursive();
        }
    }
}
