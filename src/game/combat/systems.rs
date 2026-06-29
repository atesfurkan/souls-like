use super::components::{StaggerEvent 
    ,AttackIntent,  CombatState, 
    EquippedWeapon, Hitbox, Hurtbox};
use crate::game::health::events::DamageEvent; // Event'imizi import ediyoruz
use crate::game::lock_on::components::LockOnState;
use crate::game::player::components::Player;
use crate::game::stamina::components::Stamina;
use crate::game::enemy::components::StaggerState;
use crate::game::health::components::Health;

use crate::game::weapon::components::{Weapon , AttackType}; 

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;


// 1. GERÇEK OYUNCU GİRDİSİ (Senin AttackIntent yapınla)
pub fn player_attack_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut CombatState, &Weapon, &AttackIntent),
    With<Player>>,
    
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (mut state, weapon, intent) in query.iter_mut() {
            // Sadece Idle durumundayken saldırılabilir
            if let CombatState::Idle = *state {
                // Karakterin mevcut niyetini (Mızrak için Thrust) çekiyoruz
                if let Some(def) = weapon.attacks.get(&intent.current) {
                    // Senin Weapon bileşenindeki süreleri kullanıyoruz
                    *state = CombatState::Windup {
                        timer: Timer::from_seconds(def.windup_duration, TimerMode::Once),
                        attack_type: intent.current,
                    };
                    println!("Mızrak saldırısı tetiklendi: {:?}", intent.current);
                }
            }
        }
    }
}

// 2. SAVAŞ DURUM MAKİNESİ (Senin orijinal silah sürelerini kullanır)
pub fn combat_state_machine(
    time: Res<Time>,
    mut query: Query<(&mut CombatState, &Weapon)>,
) {
    for (mut state, weapon) in query.iter_mut() {
        let mut next_state = None;

        match *state {
            CombatState::Windup { ref mut timer, attack_type } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let def = weapon.attacks.get(&attack_type).unwrap();
                    next_state = Some(CombatState::Active {
                        timer: Timer::from_seconds(def.active_duration, TimerMode::Once),
                        hitbox_entity: None,
                        attack_type,
                    });
                }
            }
            CombatState::Active { ref mut timer, attack_type, .. } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let def = weapon.attacks.get(&attack_type).unwrap();
                    next_state = Some(CombatState::Recovery {
                        timer: Timer::from_seconds(def.recovery_duration, TimerMode::Once),
                        attack_type,
                    });
                }
            }
            CombatState::Recovery { ref mut timer, .. } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    next_state = Some(CombatState::Idle);
                }
            }
            CombatState::Idle => {}
        }

        if let Some(new_state) = next_state {
            *state = new_state;
        }
    }
}



pub fn cleanup_hitboxes(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &Hitbox)>,
    owner_query: Query<&CombatState>,
) {
    for (hitbox_entity, hitbox) in hitbox_query.iter() {
        // Hitbox'ın sahibinin (Player/Enemy) mevcut durumunu al
        if let Ok(combat_state) = owner_query.get(hitbox.owner) {
            
            // Eğer sahibi artık 'Active' durumunda DEĞİLSE, saldırı bitmiş demektir. Hitbox'ı sil!
            if !matches!(combat_state, CombatState::Active { .. }) {
                commands.entity(hitbox_entity).despawn();
            }
            
        } else {
            // Eğer hitbox'ın sahibi (owner) oyundan tamamen despawn olduysa yetim kalan hitbox'ı sil
            commands.entity(hitbox_entity).despawn();
        }
    }
}

pub fn hit_registration_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut hitbox_query: Query<&mut Hitbox>,
    hurtbox_query: Query<&Hurtbox>,
    mut health_query: Query<&mut Health>,
) {
    for collision_event in collision_events.read() {
        // Sadece kesişme başladığında çalışsın (Started)
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            
            // Hangi Entity Hitbox, hangisi Hurtbox? Bunu belirle.
            let (hitbox_entity, hurtbox_entity) = if hitbox_query.contains(*entity1) && hurtbox_query.contains(*entity2) {
                (*entity1, *entity2)
            } else if hitbox_query.contains(*entity2) && hurtbox_query.contains(*entity1) {
                (*entity2, *entity1)
            } else {
                continue; // İkisi de Hitbox/Hurtbox eşleşmesi değilse pas geç
            };

            // Bileşenleri güvenli bir şekilde al
            let Ok(mut hitbox) = hitbox_query.get_mut(hitbox_entity) else { continue };
            let Ok(hurtbox) = hurtbox_query.get(hurtbox_entity) else { continue };

            // 1. Kendi kendine vurmayı engelle (Player kendi kılıcıyla vurulmasın)
            if hitbox.owner == hurtbox.owner { continue; }

            // 2. Double Hit (Tek saldırıda çoklu hasar) engelleme
            // Eğer bu Hurtbox'ın sahibi zaten vurulanlar listesindeyse, işlemi iptal et
            if hitbox.hit_entities.contains(&hurtbox.owner) { continue; }

            // 3. Health Pipeline Entegrasyonu
            // Hasarı kime uygulayacağız? Hurtbox'ın owner'ına (Player veya Enemy) [cite: 1]
            if let Ok(mut health) = health_query.get_mut(hurtbox.owner) {
                
                // Hasarı uygula [cite: 4]
                health.current -= hitbox.damage;
                
                println!(
                    "HIT REGISTERED! Entity {:?} dealt {} damage to Entity {:?}. Target Health: {}/{}",
                    hitbox.owner, hitbox.damage, hurtbox.owner, health.current, health.max
                );

                // 4. Hitbox'a bu hedefi vurduğunu kaydet ki bir sonraki frame tekrar vurmasın [cite: 3]
                hitbox.hit_entities.insert(hurtbox.owner);
                
                // Not: Eğer istersen burada Commands ile StaggerEvent de fırlatabilirsin. [cite: 1]
            }
        }
    }
}


// Bu sistem, CombatState Active'e geçtiğinde fiziksel Hitbox'ı üretir.
// Kendi mevcut state makinenin (FSM) içine de 
// bu "commands.spawn" kısmını yedirebilirsin.
pub fn spawn_hitbox_on_attack(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CombatState, &Transform, &Weapon)>, // Weapon eklendi
) {
    for (attacker_entity, mut combat_state, transform, weapon) in query.iter_mut() {
        if let CombatState::Active { ref mut hitbox_entity, attack_type, .. } = *combat_state {
            if hitbox_entity.is_none() {
                // Silah verisini çekiyoruz
                if let Some(def) = weapon.attacks.get(&attack_type) {
                    let new_hitbox = commands.spawn((
                        Hitbox {
                            damage: def.damage,
                            owner: attacker_entity,
                            hit_entities: HashSet::new(),
                        },
                        bevy_rapier3d::prelude::Collider::ball(def.hitbox_radius), 
                        bevy_rapier3d::prelude::Sensor, 
                        bevy_rapier3d::prelude::ActiveEvents::COLLISION_EVENTS, 
                        bevy_rapier3d::prelude::ActiveCollisionTypes::default() | bevy_rapier3d::prelude::ActiveCollisionTypes::KINEMATIC_STATIC,
                        TransformBundle::from_transform(
                            Transform::from_translation(transform.translation + transform.rotation * def.hitbox_offset)
                        ),
                    )).id();

                    *hitbox_entity = Some(new_hitbox);
                    println!("HITBOX YARATILDI! Hasar: {}", def.damage);
                }
            }
        }
    }
}


// Farenin hareket yönüne göre saldırı niyetini belirler
pub fn update_attack_intent(
    mut motion_evr: EventReader<MouseMotion>,
    lock_on: Res<LockOnState>, // YENİ EKLENDİ
    mut query: Query<&mut AttackIntent, With<Player>>,
) {
    let Ok(mut intent) = query.get_single_mut() else {
        return;
    };

    // --- 1. DURUM: Kilitlenme Yok (Serbest Kamera) ---
    if lock_on.target.is_none() {
        // Fare kamerayı kontrol ettiği için yön okumasını atlıyoruz
        // ve niyetimizi "Default (Varsayılan)" bir saldırıya sabitliyoruz.
        // İstersen Thrust yerine RightSwing de yapabilirsin.
        intent.current = AttackType::Thrust;

        // Event'leri okuyup tüketmek (flush) iyi bir pratiktir,
        // böylece kilitlenme açıldığında eski hareketler birikmiş olmaz.
        for _ in motion_evr.read() {}
        return;
    }

    // --- 2. DURUM: Kilitlenme Var (Savaş Duruşu) ---
    // Fare artık kamerayı kontrol etmiyor, o yüzden yön okuyabiliriz!
    let mut total_delta = Vec2::ZERO;
    for ev in motion_evr.read() {
        total_delta += ev.delta;
    }

    // Yeterince belirgin bir fare hareketi varsa niyeti değiştir
    let threshold = 5.0;
    if total_delta.length() > threshold {
        if total_delta.x.abs() > total_delta.y.abs() {
            // Yatay hareket baskın
            if total_delta.x > 0.0 {
                intent.current = AttackType::LeftSwing;
            } else {
                intent.current = AttackType::RightSwing;
            }
        } else {
            // Dikey hareket baskın (Bevy'de Y+ aşağı, Y- yukarı demektir)
            if total_delta.y > 0.0 {
                intent.current = AttackType::Thrust; // Aşağı çekme
            } else {
                intent.current = AttackType::Overhead; // İleri itme
            }
        }
    }
}

// GÜNCELLENDİ: Sol tıklandığında niyet edilen saldırıyı başlatır
pub fn handle_attacks(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut CombatState, &Weapon, &mut Stamina, &AttackIntent), With<Player>>,
) {
    let Ok((mut combat_state, weapon, mut stamina, intent)) = query.get_single_mut() else {
        return;
    };

    if matches!(*combat_state, CombatState::Idle) && mouse.just_pressed(MouseButton::Left) {
        // Niyet edilen saldırının verilerini silahtan çek
        if let Some(attack_def) = weapon.attacks.get(&intent.current) {
            if stamina.try_drain(attack_def.stamina_cost) {
                *combat_state = CombatState::Windup {
                    timer: Timer::from_seconds(attack_def.windup_duration, TimerMode::Once),
                    attack_type: intent.current,
                };
            }
        }
    }
}

// GÜNCELLENDİ: Durumları ve Hitboxları ilgili saldırı türüne göre işler
pub fn process_attack_states(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CombatState, &Weapon)>,
) {
    for (entity, mut combat_state, weapon) in query.iter_mut() {
        let mut next_state = None;

        match &mut *combat_state {
            CombatState::Idle => {}
            CombatState::Windup { timer, attack_type } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let attack_def = weapon.attacks.get(attack_type).unwrap();
                    let hitbox_entity = commands
                        .spawn((
                            Hitbox {
                                damage: attack_def.damage,
                                owner: entity,
                                hit_entities: HashSet::new(),
                            },
                            Collider::ball(attack_def.hitbox_radius),
                            Sensor,
                            ActiveEvents::COLLISION_EVENTS,
                            TransformBundle::from_transform(Transform::from_translation(
                                attack_def.hitbox_offset,
                            )),
                        ))
                        .id();

                    commands.entity(entity).add_child(hitbox_entity);

                    next_state = Some(CombatState::Active {
                        timer: Timer::from_seconds(attack_def.active_duration, TimerMode::Once),
                        hitbox_entity: Some(hitbox_entity),
                        attack_type: *attack_type,
                    });
                }
            }
            CombatState::Active {
                timer,
                hitbox_entity,
                attack_type,
            } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if let Some(h_entity) = hitbox_entity {
                        if let Some(mut entity_cmd) = commands.get_entity(*h_entity) {
                            entity_cmd.despawn_recursive();
                        }
                    }
                    let attack_def = weapon.attacks.get(attack_type).unwrap();
                    next_state = Some(CombatState::Recovery {
                        timer: Timer::from_seconds(attack_def.recovery_duration, TimerMode::Once),
                        attack_type: *attack_type,
                    });
                }
            }
            CombatState::Recovery { timer, .. } => {
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

// Güncellenmiş hit_detection sistemi
pub fn hit_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut hitboxes: Query<&mut Hitbox>, // mut yapıldı 
    hurtboxes: Query<&Hurtbox>,      // Yeni: Hurtbox'ları sorguluyoruz [cite: 3]
    mut damage_events: EventWriter<DamageEvent>,
    mut stagger_events: EventWriter<StaggerEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // İki taraflı kontrol: (e1=Hitbox, e2=Hurtbox)
            check_and_apply_hit(*e1, *e2, &mut hitboxes, &hurtboxes, &mut damage_events, &mut stagger_events);
            // Veya (e2=Hitbox, e1=Hurtbox)
            check_and_apply_hit(*e2, *e1, &mut hitboxes, &hurtboxes, &mut damage_events, &mut stagger_events);
        }
    }
}

// Güncellenmiş yardımcı fonksiyon
fn check_and_apply_hit(
    attacker_entity: Entity,
    victim_entity: Entity,
    hitboxes: &mut Query<&mut Hitbox>,
    hurtboxes: &Query<&Hurtbox>, // Hurtbox parametresi eklendi
    damage_events: &mut EventWriter<DamageEvent>,
    stagger_events: &mut EventWriter<StaggerEvent>,
) {
    // 1. Saldırgan bir hitbox mı?
    if let Ok(mut hitbox) = hitboxes.get_mut(attacker_entity) {
        // 2. Kurban bir hurtbox mı?
        if let Ok(hurtbox) = hurtboxes.get(victim_entity) {
            
            // Kendi kendine vurma kontrolü (Hurtbox'ın sahibi saldırgan mı?)
            if hitbox.owner == hurtbox.owner { return; }

            // 3. Bu saldırıda bu hedefe (owner) daha önce vurmadıysan vur
            if !hitbox.hit_entities.contains(&hurtbox.owner) {
                hitbox.hit_entities.insert(hurtbox.owner);

                // Hasarı hurtbox'un sahibine (Health'in olduğu entity) yönlendir
                damage_events.send(DamageEvent {
                    target: hurtbox.owner, 
                    amount: hitbox.damage,
                });
                
                stagger_events.send(StaggerEvent {
                    target: hurtbox.owner,
                    force: 1.0, 
                    duration: 0.25,
                });
            }
        }
    }
}
// GÜNCELLENDİ: Hangi saldırı türü olduğuna göre farklı yörüngelerde animasyon oynatır
pub fn animate_weapon(
    entity_query: Query<(&CombatState, &Children)>,
    mut weapon_query: Query<&mut Transform, With<EquippedWeapon>>,
) {
    for (state, children) in entity_query.iter() {
        let Some(&weapon_entity) = children.iter().find(|&&child| weapon_query.contains(child))
        else {
            continue;
        };
        let Ok(mut transform) = weapon_query.get_mut(weapon_entity) else {
            continue;
        };

        let idle_pos = Vec3::new(0.6, 0.0, -0.5);
        let idle_rot = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4);

        match state {
            CombatState::Idle => {
                transform.translation = idle_pos;
                transform.rotation = idle_rot;
            }
            CombatState::Windup { timer, attack_type } => {
                let t = timer.fraction();
                let (windup_pos, windup_rot) = get_windup_transform(*attack_type);
                transform.translation = idle_pos.lerp(windup_pos, t);
                transform.rotation = idle_rot.slerp(windup_rot, t);
            }
            CombatState::Active {
                timer, attack_type, ..
            } => {
                let t = timer.fraction();
                let (windup_pos, windup_rot) = get_windup_transform(*attack_type);
                let (active_pos, active_rot) = get_active_transform(*attack_type);
                transform.translation = windup_pos.lerp(active_pos, t);
                transform.rotation = windup_rot.slerp(active_rot, t);
            }
            CombatState::Recovery { timer, attack_type } => {
                let t = timer.fraction();
                let (active_pos, active_rot) = get_active_transform(*attack_type);
                transform.translation = active_pos.lerp(idle_pos, t);
                transform.rotation = active_rot.slerp(idle_rot, t);
            }
        }
    }
}

// YARDIMCI FONKSİYONLAR: Farklı saldırılar için keyframe (anahtar kare) konumları
fn get_windup_transform(attack: AttackType) -> (Vec3, Quat) {
    match attack {
        AttackType::Thrust => (Vec3::new(0.6, 0.5, 0.2), Quat::from_rotation_x(-0.1)),
        AttackType::Overhead => (Vec3::new(0.0, 1.5, 0.0), Quat::from_rotation_x(0.5)),
        AttackType::LeftSwing => (
            Vec3::new(-0.8, 0.5, -0.2),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_4),
        ),
        AttackType::RightSwing => (
            Vec3::new(1.2, 0.5, -0.2),
            Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4),
        ),
    }
}

fn get_active_transform(attack: AttackType) -> (Vec3, Quat) {
    match attack {
        AttackType::Thrust => (
            Vec3::new(0.0, 0.8, -1.3),
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
        AttackType::Overhead => (
            Vec3::new(0.0, 0.2, -1.2),
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
        AttackType::LeftSwing => (
            Vec3::new(1.0, 0.5, -1.0),
            Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)
                * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
        AttackType::RightSwing => (
            Vec3::new(-1.0, 0.5, -1.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
                * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ),
    }
}
