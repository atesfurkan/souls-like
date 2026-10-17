use bevy::prelude::*;
use crate::game::poise::components::Poise;
use crate::game::poise::events::{ApplyPoiseDamageEvent, PoiseBrokenEvent};

pub fn handle_poise_damage(
    mut events: EventReader<ApplyPoiseDamageEvent>,
    mut poise_query: Query<&mut Poise>,
    mut broken_events: EventWriter<PoiseBrokenEvent>,
) {
    for event in events.read() {
        if let Ok(mut poise) = poise_query.get_mut(event.target) {
            poise.current -= event.amount;
            println!("Poise Hasarı Alındı! Hedef: {:?}, Kalan Poise: {}", event.target, poise.current);

            // Poise sıfıra veya altına düştüyse kırılma eventini fırlat
            if poise.current <= 0.0 {
                poise.current = poise.max; // Bu sprint için basitçe yeniliyoruz, sonraki sprintlerde cooldown/regen eklenebilir
                broken_events.send(PoiseBrokenEvent {
                    target: event.target,
                });
                println!("POISE KIRILDI! Hedef: {:?}", event.target);
            }
        }
    }
}
