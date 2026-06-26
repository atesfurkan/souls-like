use super::components::{StaggerEvent ,AttackIntent, AttackType, CombatState, EquippedWeapon, Hitbox, Weapon};
use crate::game::health::events::DamageEvent; // Event'imizi import ediyoruz
use crate::game::lock_on::components::LockOnState;
use crate::game::player::components::Player;
use crate::game::stamina::components::Stamina;
use crate::game::enemy::components::StaggerState;


use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::*;

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

// YENİ SİSTEM: Çarpışmaları yakalayıp Hasar Event'ine dönüştüren sistem
pub fn hit_detection(
    mut collision_events: EventReader<CollisionEvent>,
    mut hitboxes: Query<&mut Hitbox>,
    mut damage_events: EventWriter<DamageEvent>,
    mut stagger_events: EventWriter<StaggerEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // Hangi objenin Hitbox, hangi objenin Hedef olduğunu bilmiyoruz. İki ihtimali de deniyoruz.
            check_and_apply_hit(*entity1, *entity2, &mut hitboxes, &mut damage_events,&mut stagger_events);
            check_and_apply_hit(*entity2, *entity1, &mut hitboxes, &mut damage_events,&mut stagger_events);
        }
    }
}

// Yardımcı fonksiyon: Hitbox'ı ve kurbanı doğrular, event fırlatır
fn check_and_apply_hit(
    potential_hitbox: Entity,
    potential_target: Entity,
    hitboxes: &mut Query<&mut Hitbox>,
    damage_events: &mut EventWriter<DamageEvent>,
    stagger_events: &mut EventWriter<StaggerEvent>,
) {
    if let Ok(mut hitbox) = hitboxes.get_mut(potential_hitbox) {
        // 1. Kural: Kendine hasar veremezsin
        if hitbox.owner == potential_target {
            return;
        }

        // 2. Kural: Bu saldırıda bu hedefe daha önce vurmadıysan vur
        if !hitbox.hit_entities.contains(&potential_target) {
            hitbox.hit_entities.insert(potential_target); // Hedefi hafızaya al

            println!("HIT DETECTED! Dealing {} damage.", hitbox.damage);

            // Sağlık sistemine haber ver. (Eğer hedefte Health bileşeni yoksa, Health sistemi bu olayı görmezden gelecektir.)
            damage_events.send(DamageEvent {
                target: potential_target,
                amount: hitbox.damage,
            });
            stagger_events.send(StaggerEvent {
                target: potential_target,
                force: 1.0, 
                duration: 0.25,
            });
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
