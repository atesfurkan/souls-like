use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::components::{CombatState, Weapon , Hitbox , EquippedWeapon };
use crate::game::player::components::Player;
use crate::game::stamina::components::Stamina;
use crate::game::health::events::DamageEvent; // Event'imizi import ediyoruz
use bevy::utils::HashSet;



// Sol tık dinleyip, staminayı kontrol eden ve saldırıyı başlatan sistem
pub fn handle_attacks(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut CombatState, &Weapon, &mut Stamina), With<Player>>,
) {
    let Ok((mut combat_state, weapon, mut stamina)) = query.get_single_mut() else { return };

    // Sadece Idle durumundayken sol tıka basılırsa saldırı başlar
    if matches!(*combat_state, CombatState::Idle) && mouse.just_pressed(MouseButton::Left) {
        // Staminada yer varsa tüket ve saldırıya geç [cite: 47]
        if stamina.try_drain(weapon.stamina_cost) {
            *combat_state = CombatState::Windup(Timer::from_seconds(weapon.windup_duration, TimerMode::Once));
            println!("Attack Started: Windup"); // Debug için
        } else {
            println!("Not enough stamina to attack!"); // Debug için
        }
    }
}


pub fn process_attack_states(
    mut commands: Commands,
    time: Res<Time>,
    // DİKKAT: Artık oyuncuya özel değil. 
    mut query: Query<(Entity, &mut CombatState, &Weapon)>, 
) {
    // Dünyada mızrağı ve savaş durumu olan herkesi sırayla dönüyoruz
    for (entity, mut combat_state, weapon) in query.iter_mut() {
        let mut next_state = None;

        match &mut *combat_state {
            CombatState::Idle => {} 
            CombatState::Windup(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    // --- WINDUP BİTTİ, ACTIVE BAŞLIYOR: HITBOX YARAT ---
                    let hitbox_entity = commands.spawn((
                        Hitbox {
                            damage: weapon.damage,
                            // DİKKAT: Artık owner 'player_entity' değil, döngüdeki 'entity' (Vuran kişi kimse o)
                            owner: entity, 
                            hit_entities: bevy::utils::HashSet::new(),
                        },
                        Collider::ball(weapon.hitbox_radius),
                        Sensor,
                        ActiveEvents::COLLISION_EVENTS,
                        TransformBundle::from_transform(Transform::from_translation(weapon.hitbox_offset)),
                    )).id();

                    // Hitbox'ı vuran kişiye (oyuncu veya düşman) bağla
                    commands.entity(entity).add_child(hitbox_entity);

                    next_state = Some(CombatState::Active(
                        Timer::from_seconds(weapon.active_duration, TimerMode::Once),
                        Some(hitbox_entity)
                    ));
                }
            }
            CombatState::Active(timer, hitbox_entity) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    // --- ACTIVE BİTTİ, RECOVERY BAŞLIYOR: HITBOX'I SİL ---
                    if let Some(h_entity) = hitbox_entity {
                        if let Some(mut entity_cmd) = commands.get_entity(*h_entity) {
                            entity_cmd.despawn_recursive();
                        }
                    }
                    next_state = Some(CombatState::Recovery(Timer::from_seconds(weapon.recovery_duration, TimerMode::Once)));
                }
            }
            CombatState::Recovery(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    next_state = Some(CombatState::Idle);
                }
            }
        }

        if let Some(state) = next_state {
            *combat_state = state;
        }
    }
}

// YENİ SİSTEM: Çarpışmaları yakalayıp Hasar Event'ine dönüştüren sistem
pub fn hit_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut hitboxes: Query<&mut Hitbox>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // Hangi objenin Hitbox, hangi objenin Hedef olduğunu bilmiyoruz. İki ihtimali de deniyoruz.
            check_and_apply_hit(*entity1, *entity2, &mut hitboxes, &mut damage_events);
            check_and_apply_hit(*entity2, *entity1, &mut hitboxes, &mut damage_events);
        }
    }
}

// Yardımcı fonksiyon: Hitbox'ı ve kurbanı doğrular, event fırlatır
fn check_and_apply_hit(
    potential_hitbox: Entity,
    potential_target: Entity,
    hitboxes: &mut Query<&mut Hitbox>,
    damage_events: &mut EventWriter<DamageEvent>,
) {
    if let Ok(mut hitbox) = hitboxes.get_mut(potential_hitbox) {
        // 1. Kural: Kendine hasar veremezsin
        if hitbox.owner == potential_target { return; }

        // 2. Kural: Bu saldırıda bu hedefe daha önce vurmadıysan vur
        if !hitbox.hit_entities.contains(&potential_target) {
            hitbox.hit_entities.insert(potential_target); // Hedefi hafızaya al
            
            println!("HIT DETECTED! Dealing {} damage.", hitbox.damage);

            // Sağlık sistemine haber ver. (Eğer hedefte Health bileşeni yoksa, Health sistemi bu olayı görmezden gelecektir.)
            damage_events.send(DamageEvent {
                target: potential_target,
                amount: hitbox.damage,
            });
        }
    }
}



pub fn animate_weapon(
    // DİKKAT: With<Player> kaldırıldı. Artık hem oyuncuyu hem düşmanları bulacak.
    entity_query: Query<(&CombatState, &Children)>, 
    mut weapon_query: Query<&mut Transform, With<EquippedWeapon>>,
) {
    // Dünyadaki tüm savaşabilen karakterleri (oyuncu + düşmanlar) dönüyoruz
    for (state, children) in entity_query.iter() {
        
        // 1. ADIM: Bu karakterin çocukları arasında silah var mı?
        // Eğer bulamazsa 'return' (sistemi toptan durdur) DEĞİL, 'continue' (sonraki karaktere geç) diyoruz.
        let Some(&weapon_entity) = children.iter().find(|&&child| weapon_query.contains(child)) else { 
            continue; 
        };

        // 2. ADIM: Silahın Transform'unu mutlak (mutable) olarak al.
        let Ok(mut transform) = weapon_query.get_mut(weapon_entity) else { 
            continue; 
        };

        // --- ANİMASYON ANAHTAR KARELERİ (KEYFRAMES) ---
        
        // 1. Bekleme Duruşu (Sağ elde, hafif öne eğik)
        let idle_pos = Vec3::new(0.6, 0.0, -0.5);
        let idle_rot = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4); // -45 derece

        // 2. Windup - Geriye Çekilme 
        let windup_pos = Vec3::new(0.6, 0.5, 0.2); 
        let windup_rot = Quat::from_rotation_x(-0.1); 

        // 3. Active - Saplama 
        let active_pos = Vec3::new(0.0, 0.8, -1.3); 
        let active_rot = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2); 

        // Matematiksel interpolasyon:
        match state {
            CombatState::Idle => {
                transform.translation = idle_pos;
                transform.rotation = idle_rot;
            }
            CombatState::Windup(timer) => {
                let t = timer.fraction(); // 0.0'dan 1.0'a doğru ilerler
                transform.translation = idle_pos.lerp(windup_pos, t);
                transform.rotation = idle_rot.slerp(windup_rot, t);
            }
            CombatState::Active(timer, _) => {
                let t = timer.fraction();
                transform.translation = windup_pos.lerp(active_pos, t);
                transform.rotation = windup_rot.slerp(active_rot, t);
            }
            CombatState::Recovery(timer) => {
                let t = timer.fraction();
                transform.translation = active_pos.lerp(idle_pos, t);
                transform.rotation = active_rot.slerp(idle_rot, t);
            }
        }
    }
}


