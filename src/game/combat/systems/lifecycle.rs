use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;
use crate::game::combat::components::{CombatState, Hitbox};
use crate::game::shield::events::RecoilEvent;
use crate::game::weapon::components::Weapon;

pub fn handle_recoil_events(
    mut recoil_events: EventReader<RecoilEvent>,
    mut query: Query<&mut CombatState>,
) {
    for event in recoil_events.read() {
        if let Ok(mut state) = query.get_mut(event.target) {
            *state = CombatState::Recoiled {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            };
            println!("RECOIL TETİKLENDİ: Entity {:?} sekti!", event.target);
        }
    }
}

pub fn cleanup_hitboxes(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &Hitbox)>,
    owner_query: Query<&CombatState>,
) {
    for (hitbox_entity, hitbox) in hitbox_query.iter() {
        if let Ok(combat_state) = owner_query.get(hitbox.owner) {
            if !matches!(combat_state, CombatState::Active { .. }) {
                commands.entity(hitbox_entity).despawn();
            }
        } else {
            commands.entity(hitbox_entity).despawn();
        }
    }
}

pub fn spawn_hitbox_on_attack(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CombatState, &Transform, &Weapon)>,
) {
    for (attacker_entity, mut combat_state, transform, weapon) in query.iter_mut() {
        if let CombatState::Active {
            ref mut hitbox_entity,
            attack_type,
            ..
        } = *combat_state
        {
            if hitbox_entity.is_none() {
                if let Some(def) = weapon.attacks.get(&attack_type) {
                    let new_hitbox = commands
                        .spawn((
                            Hitbox {
                                damage: def.damage,
                                owner: attacker_entity,
                                hit_entities: HashSet::new(),
                                lifesteal: def.lifesteal,
                            },
                            Collider::ball(def.hitbox_radius),
                            Sensor,
                            ActiveEvents::COLLISION_EVENTS,
                            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
                            TransformBundle::from_transform(Transform::from_translation(
                                transform.translation + transform.rotation * def.hitbox_offset,
                            )),
                        ))
                        .id();
                    *hitbox_entity = Some(new_hitbox);
                    println!("HITBOX YARATILDI! Hasar: {}", def.damage);
                }
            }
        }
    }
}
