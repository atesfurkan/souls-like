use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::combat::components::{Hitbox, Hurtbox, StaggerEvent};
use crate::game::health::events::{DamageEvent, HealEvent};
use crate::game::shield::components::{BlockingState, Shield, ShieldBox};
use crate::game::shield::events::{BlockEvent, RecoilEvent};

pub fn hit_detection(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut hitboxes: Query<&mut Hitbox>,
    hurtboxes: Query<&Hurtbox>,
    shieldboxes: Query<&ShieldBox>,
    shield_query: Query<(&Shield, &BlockingState)>,
    mut damage_events: EventWriter<DamageEvent>,
    mut stagger_events: EventWriter<StaggerEvent>,
    mut heal_events: EventWriter<HealEvent>,
    mut recoil_events: EventWriter<RecoilEvent>,
    mut block_events: EventWriter<BlockEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // --- 1. ÖNCELİK: KALKAN ÇARPIŞMASI ---
            if let Some((hitbox_entity, _, attacker, defender)) = check_shield_hit(*e1, *e2, &hitboxes, &shieldboxes) {
                if let Ok((_shield, blocking_state)) = shield_query.get(defender) {
                    if attacker == defender {
                        continue;
                    }

                    match *blocking_state {
                        BlockingState::Parrying { .. } => {
                            println!("MÜKEMMEL PARRY! Saldırgan sekti.");
                            commands.entity(hitbox_entity).despawn_recursive();
                            recoil_events.send(RecoilEvent { target: attacker });
                            block_events.send(BlockEvent { defender, attacker, is_parry: true });
                            continue;
                        }
                        BlockingState::Blocking => {
                            println!("NORMAL BLOK! Hasar engellendi.");
                            commands.entity(hitbox_entity).despawn_recursive();
                            block_events.send(BlockEvent { defender, attacker, is_parry: false });
                            continue;
                        }
                        _ => {}
                    }
                }
            }

            // --- 2. ÖNCELİK: NORMAL HASAR (HURTBOX) ---
            check_and_apply_hit(*e1, *e2, &mut hitboxes, &hurtboxes, &mut damage_events, &mut stagger_events, &mut heal_events);
            check_and_apply_hit(*e2, *e1, &mut hitboxes, &hurtboxes, &mut damage_events, &mut stagger_events, &mut heal_events);
        }
    }
}

fn check_shield_hit(
    e1: Entity,
    e2: Entity,
    hitboxes: &Query<&mut Hitbox>,
    shieldboxes: &Query<&ShieldBox>,
) -> Option<(Entity, Entity, Entity, Entity)> {
    if let Ok(hitbox) = hitboxes.get(e1) {
        if let Ok(shieldbox) = shieldboxes.get(e2) {
            return Some((e1, e2, hitbox.owner, shieldbox.owner));
        }
    }
    if let Ok(hitbox) = hitboxes.get(e2) {
        if let Ok(shieldbox) = shieldboxes.get(e1) {
            return Some((e2, e1, hitbox.owner, shieldbox.owner));
        }
    }
    None
}

fn check_and_apply_hit(
    attacker_entity: Entity,
    victim_entity: Entity,
    hitboxes: &mut Query<&mut Hitbox>,
    hurtboxes: &Query<&Hurtbox>,
    damage_events: &mut EventWriter<DamageEvent>,
    stagger_events: &mut EventWriter<StaggerEvent>,
    heal_events: &mut EventWriter<HealEvent>,
) {
    if let Ok(mut hitbox) = hitboxes.get_mut(attacker_entity) {
        if let Ok(hurtbox) = hurtboxes.get(victim_entity) {
            if hitbox.owner == hurtbox.owner {
                return;
            }

            if !hitbox.hit_entities.contains(&hurtbox.owner) {
                hitbox.hit_entities.insert(hurtbox.owner);
                damage_events.send(DamageEvent {
                    target: hurtbox.owner,
                    amount: hitbox.damage,
                });
                stagger_events.send(StaggerEvent {
                    target: hurtbox.owner,
                    force: 1.0,
                    duration: 0.25,
                });
                if hitbox.lifesteal > 0.0 {
                    heal_events.send(HealEvent {
                        target: hitbox.owner,
                        amount: hitbox.lifesteal,
                    });
                    println!("LIFESTEAL TETİKLENDİ: {} can kazanıldı!", hitbox.lifesteal);
                }
            }
        }
    }
}
