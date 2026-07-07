use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::utils::HashMap;
use crate::game::common::components::MainCamera;
use crate::game::stamina::components::Stamina;
use crate::game::dodge::components::DodgeState;
use crate::game::combat::components::{CombatState, EquippedWeapon,
      AttackIntent};
use crate::game::lock_on::components::LockOnState;
use crate::game::weapon::components::{Weapon, AttackDefinition,
     AttackType };
use crate::game::weapon::events::WeaponSwapRequestEvent;
use super::components::{Player, PlayerState, PlayerMovementConfig, 
    PendingEquipment}; 
use crate::game::shield::data::ShieldType; // bu ne alaka burada?
use crate::game::animation::components::{CharacterBones,AnimationIntent};

use bevy::input::mouse::MouseMotion; // Gerekli import

use crate::game::combat::components::GuardIntent;
use crate::game::shield::components::BlockingState;
use crate::game::ik::components::TwoBoneIkChain;

use bevy::render::primitives::Aabb;

pub fn auto_fit_player_collider(
    mut player_query: Query<(Entity, &mut Collider), With<Player>>,
    children_query: Query<&Children>,
    aabb_query: Query<&Aabb>,
    // Bu sistemin her frame çalışmasını engellemek için bir bayrak tutuyoruz
    mut is_fitted: Local<bool>, 
) {
    if *is_fitted { return; }

    for (player_entity, mut collider) in player_query.iter_mut() {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        let mut found_mesh = false;

        // Karakterin içindeki tüm 3D model parçalarını (meshleri) tara
        for descendant in children_query.iter_descendants(player_entity) {
            if let Ok(aabb) = aabb_query.get(descendant) {
                let center = Vec3::from(aabb.center);
                let half_extents = Vec3::from(aabb.half_extents);
                
                // Modelin en uç noktalarını hesapla
                min = min.min(center - half_extents);
                max = max.max(center + half_extents);
                found_mesh = true;
            }
        }

        if found_mesh {
            // Modelin toplam boyutlarını (genişlik, yükseklik, derinlik) bul
            let size = max - min;
            
            // Kapsülün yarıçapı, karakterin X (genişlik) veya Z (derinlik) eksenindeki en büyük değerinin yarısıdır
            let radius = (size.x.max(size.z)) / 2.0;
            
            // Rapier kapsülünde 'half_height', silindir kısmının yarısıdır (toplam boydan küreleri çıkarıyoruz)
            let half_height = (size.y / 2.0) - radius;

            // Kapsülü karakterin gerçek 3D vücut ölçüleriyle GÜNCELLE!
            *collider = Collider::capsule_y(half_height.max(0.1), radius.max(0.1));
            
            println!("BAŞARILI: Oyuncu fiziği 3D modele göre otomatik sarıldı! Yarıçap: {:.2}", radius);
            *is_fitted = true; 
        }
    }
}


pub fn update_guard_intent(
    mut mouse_motion: EventReader<MouseMotion>,
    lock_on: Res<LockOnState>,
    mut query: Query<(&mut GuardIntent, &BlockingState), With<Player>>,
) {
    if lock_on.target.is_none() { 
        return; }

    for (mut intent, state) in query.iter_mut() {
        // Sadece Blocking durumunda kalkan yönlendirilebilir
        if matches!(state, BlockingState::Blocking) {
            for motion in mouse_motion.read() {
                //println!("DEBUG: Fare hareketi algılandı: {:?}", motion.delta); // Bu yazıyor mu?
                // Hassasiyet ayarı (ileride config'e alınabilir)
                let sensitivity = 0.005;
                intent.offset += motion.delta * sensitivity;
                
                // İnsan anatomisi sınırı (Kalkanı çok uzağa savurma)
                intent.offset = intent.offset.clamp(Vec2::splat(-0.5), Vec2::splat(0.5));
            }
        } else {
            // Bloklamıyorsak intent'i merkeze sıfırla
            intent.offset = intent.offset.lerp(Vec2::ZERO, 0.1);
        }
    }
}


// player/systems.rs
pub fn attach_pending_equipment(
    mut commands: Commands,
    // DİKKAT: Entity'i de sorguya ekledik ki Player'a ulaşabilelim
    character_query: Query<(Entity, &CharacterBones)>, 
    equipment_query: Query<(Entity, Option<&EquippedWeapon>, Option<&ShieldType>), With<PendingEquipment>>,
) {
    if equipment_query.is_empty() { return; }

    for (player_entity, bones) in character_query.iter() {
        if !bones.is_linked { continue; }

        for (eq_entity, is_weapon, is_shield) in equipment_query.iter() {
            if is_weapon.is_some() {
                if let Some(right_hand) = bones.right_hand {
                    commands.entity(eq_entity)
                        .remove::<PendingEquipment>()
                        .insert(Transform::IDENTITY);
                    commands.entity(right_hand).add_child(eq_entity);
                }
            } else if is_shield.is_some() {
                if let Some(left_forearm) = bones.left_forearm {
                    commands.entity(eq_entity)
                        .remove::<PendingEquipment>()
                        .insert(Transform::IDENTITY);
                    commands.entity(left_forearm).add_child(eq_entity);
                    println!("DEBUG: Kalkan ön kola (left_forearm) bağlandı.");
                    
                    // --- IK ZİNCİRİ KONTROLÜ ---
                    if let (Some(arm), Some(hand)) = (bones.left_arm, bones.left_hand) {
                        commands.entity(player_entity).insert(TwoBoneIkChain {
                            root: arm,
                            mid: left_forearm,
                            tip: hand,
                            pole_vector: Vec3::new(-1.0, -1.0, -1.0),
                        });
                        println!("BAŞARILI: IK Zinciri oyuncuya bağlandı!");
                    } else {
                        // EĞER KEMİKLER EKSİKSE BURAYA DÜŞECEK
                        println!("HATA: Kalkan bağlandı ama Omuz (left_arm) veya El (left_hand) kemiği BULUNAMADI!");
                        println!("Mevcut Kemikler -> Arm: {:?}, Hand: {:?}", bones.left_arm, bones.left_hand);
                    }
                }
            }
        }
    }
}



pub fn handle_weapon_swap_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut swap_events: EventWriter<WeaponSwapRequestEvent>,
    player_query: Query<(Entity, &CombatState), With<Player>>,
) {
    // 1 tuşuna basılıp basılmadığını kontrol et
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        if let Ok((entity, combat_state)) = player_query.get_single() {
            // Edge Case kontrolü: Sadece Idle durumundayken silah değiştirilebilir
            if *combat_state == CombatState::Idle {
                swap_events.send(WeaponSwapRequestEvent { entity });
            }
        }
    }
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<PlayerMovementConfig>, 
    lock_on: Res<LockOnState>,
    mut player_query: Query<(
        &mut Transform, 
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>, // Yere değme kontrolü
        &mut PlayerState, // Dikey hız hafızası
        &mut Stamina,
        &mut AnimationIntent, // YENİ: Animasyon niyetimizi buraya yazacağız
    ), (With<Player>, Without<DodgeState>)>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    target_query: Query<&Transform, (Without<Player>, Without<MainCamera>)>, 
) {
    // Artık 6. eleman olarak intent'i de alıyoruz
    let Ok((mut player_transform, mut controller, output, mut state, mut stamina, mut intent)) = player_query.get_single_mut() else { return };
    let Ok(camera_transform) = camera_query.get_single() else { return };

    let mut direction = Vec3::ZERO;

    let forward = camera_transform.forward();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = camera_transform.right();
    let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();
    
    if keys.pressed(KeyCode::KeyW) { direction += forward_flat; }
    if keys.pressed(KeyCode::KeyS) { direction -= forward_flat; }
    if keys.pressed(KeyCode::KeyD) { direction += right_flat; }
    if keys.pressed(KeyCode::KeyA) { direction -= right_flat; }

    // --- MANTIKSAL HAREKET VE SPRINT KONTROLÜ ---
    let mut is_moving = false;
    let mut is_sprinting = false;
    let mut speed = config.walk_speed;

    if direction != Vec3::ZERO {
        is_moving = true; // Karakter hareket etmeye çalışıyor
        
        if keys.pressed(KeyCode::ShiftLeft) {
            let cost = stamina.sprint_cost * time.delta_seconds();
            if stamina.try_drain(cost) {
                speed = config.sprint_speed;
                is_sprinting = true; // Karakter koşmaya çalışıyor ve staminası var
            }
        }
    }

    
    let mut movement = direction.normalize_or_zero() * speed * time.delta_seconds();
    
    // --- ZIPLAMA VE YERÇEKİMİ ---
    let is_grounded = match output {
        Some(out) => out.grounded,
        None => false,
    };
    
    // Niyetleri Animation Modülüne bildir
    intent.is_moving = is_moving;
    intent.is_sprinting = is_sprinting;
    intent.is_in_air = !is_grounded; // YENİ: Yerde değilsek havadayızdır lan 

    if is_grounded {
        state.vertical_velocity = -0.1;
        if keys.just_pressed(KeyCode::Space) {
            let cost = stamina.jump_cost;
            state.vertical_velocity = config.jump_velocity;
            stamina.try_drain(cost);
        }
    } else {
        state.vertical_velocity += config.gravity * time.delta_seconds();
    }
    
    movement.y = state.vertical_velocity * time.delta_seconds();
    controller.translation = Some(movement);

    // --- ROTASYON MANTIĞI ---
    if let Some(target_entity) = lock_on.target {
        if let Ok(target_transform) = target_query.get(target_entity) {
            let target_pos = Vec3::new(
                target_transform.translation.x, 
                player_transform.translation.y, 
                target_transform.translation.z
            );
            player_transform.look_at(target_pos, Vec3::Y);
        }
    } else {
        if direction != Vec3::ZERO {
            let target_position = player_transform.translation + direction;
            player_transform.look_at(target_position, Vec3::Y);
        }
    }
}