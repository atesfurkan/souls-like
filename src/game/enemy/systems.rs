use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::components::{Enemy, EnemyAI, EnemyAIState};
use crate::game::health::components::Health;
use crate::game::player::components::Player;
use crate::game::combat::components::{ StaggerEvent, CombatState, Weapon, EquippedWeapon , AttackDefinition, AttackType, AttackIntent };//burada neden eq. weapon var lan ,bi sorak
use crate::game::lock_on::components::Lockable;
use crate::game::enemy::components::StaggerState;


use bevy::utils::HashMap;


// YENİ: Hasar anında tetiklenen Stagger Event'ini yakalar
pub fn handle_stagger_events(
    mut commands: Commands,
    mut events: EventReader<StaggerEvent>,
    mut query: Query<(&mut EnemyAIState, &mut CombatState)>,
) {
    for event in events.read() {
        println!("Stagger Event Received for entity: {:?}", event.target); // LOG
        if let Ok((mut ai_state, mut combat_state)) = query.get_mut(event.target) {
            println!("Applying Stagger to Enemy!");
            *ai_state = EnemyAIState::Stagger;
            // 2. Saldırıyı iptal et (combat'ı boşa çıkar)
            *combat_state = CombatState::Idle;
            // 3. Stagger bileşenini ekle
            commands.entity(event.target).insert(StaggerState {
                timer: Timer::from_seconds(event.duration, TimerMode::Once),
            });
        }
        else{
            println!("Enemy AI State not found for entity!");
        }
    }
}

// YENİ: Stagger süresini sayan ve bitince AI'yı eski haline döndüren sistem
pub fn process_stagger(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut StaggerState, &mut EnemyAIState)>,
) {
    for (entity, mut stagger, mut ai_state) in query.iter_mut() {
        stagger.timer.tick(time.delta());
        if stagger.timer.finished() {
            *ai_state = EnemyAIState::Idle; // Stagger bitti
            commands.entity(entity).remove::<StaggerState>(); // Bileşeni sil
        }
    }
}


pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let enemy_material = materials.add(Color::srgb(0.8, 0.1, 0.1));
    let weapon_wood = materials.add(Color::srgb(0.4, 0.2, 0.1));
    let weapon_metal = materials.add(Color::srgb(0.8, 0.8, 0.8));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d { radius: 0.6, half_length: 0.6 }), 
            material: enemy_material,
            transform: Transform::from_xyz(5.0, 3.0, -5.0),
            ..default()
        },
        Enemy,
        Lockable,
        Health::new(100.0),
        
        // --- YENİ: AI ve Savaş Bileşenleri ---
        EnemyAIState::Idle,
        EnemyAI {
            detection_range: 12.0,
            attack_range: 2.0, // Mızrak menzili oyuncuya yetecek kadar uzun
            speed: 3.0,
            attack_cooldown: Timer::from_seconds(1.5, TimerMode::Once), // 1.5 saniyede bir vurabilir
        },
        CombatState::default(),
        AttackIntent::default(),
        Weapon {
            attacks: {
                let mut map = HashMap::new();
                map.insert(AttackType::Thrust, AttackDefinition {
                    windup_duration: 0.3, active_duration: 0.2, recovery_duration: 0.5,
                    stamina_cost: 15.0, damage: 22.0,
                    hitbox_radius: 0.3, hitbox_offset: Vec3::new(0.0, 0.8, -1.5), // İleri doğru, dar alan
                });
                map.insert(AttackType::Overhead, AttackDefinition {
                    windup_duration: 0.5, active_duration: 0.2, recovery_duration: 0.7,
                    stamina_cost: 22.0, damage: 40.0, // Ağır vuruş
                    hitbox_radius: 0.4, hitbox_offset: Vec3::new(0.0, 0.5, -1.0), // Yakın mesafe ezme
                });
                map.insert(AttackType::LeftSwing, AttackDefinition {
                    windup_duration: 0.4, active_duration: 0.2, recovery_duration: 0.6,
                    stamina_cost: 20.0, damage: 20.0,
                    hitbox_radius: 0.6, hitbox_offset: Vec3::new(0.0, 1.0, -0.8), // Geniş alan kontrolü
                });
                map.insert(AttackType::RightSwing, AttackDefinition {
                    windup_duration: 0.4, active_duration: 0.2, recovery_duration: 0.6,
                    stamina_cost: 20.0, damage: 20.0,
                    hitbox_radius: 0.6, hitbox_offset: Vec3::new(0.0, 1.0, -0.8), // Geniş alan kontrolü
                });
                map
            }
        },

        // --- Fizik Bileşenleri ---
        RigidBody::Dynamic, 
        Velocity::default(), // Hareketi kontrol etmek için eklendi
        Collider::capsule_y(0.6, 0.6),
        LockedAxes::ROTATION_LOCKED, 
    ))
    .with_children(|parent| {
        // --- SİLAH (SPEAR) ---
        // Oyuncunun mızrağının birebir aynısı düşmanın eline de bağlanıyor
        parent.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.7, 0.0, -0.6)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
                ..default()
            },
            EquippedWeapon, 
        )).with_children(|weapon_parent| {
            weapon_parent.spawn(PbrBundle {
                mesh: meshes.add(Cylinder { radius: 0.05, half_height: 1.0 }),
                material: weapon_wood,
                ..default()
            });
            weapon_parent.spawn(PbrBundle {
                mesh: meshes.add(Cone { radius: 0.1, height: 0.4 }),
                material: weapon_metal,
                transform: Transform::from_xyz(0.0, 1.2, 0.0),
                ..default()
            });
        });
    });
}

// YENİ SİSTEM: Düşman FSM Yapay Zekası
pub fn enemy_ai_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut EnemyAI,
        &mut EnemyAIState,
        &mut CombatState,
        &Weapon,
        &Health,
    ), (With<Enemy>, Without<Player>)>,
) {
    // Oyuncu hayatta değilse veya yoksa AI çalışmasın
    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation;

    for (mut transform, mut velocity, mut ai, mut ai_state, mut combat_state, weapon, health) in enemy_query.iter_mut() {
        // Cooldown sayacı her frame akar
        ai.attack_cooldown.tick(time.delta());

        // 1. KURAL: Can sıfır veya altındaysa AI sistemlerinden tamamen çık
        if health.current <= 0.0 {
            *ai_state = EnemyAIState::Dead;
            velocity.linvel.x = 0.0;
            velocity.linvel.z = 0.0;
            // Y'ye dokunmuyoruz ki düşebilsin // Hareketi durdur
            continue; // Döngünün kalanını atla (Sistemden çıkış)
        }

        let enemy_pos = transform.translation;
        let distance = enemy_pos.distance(player_pos);

        // Sadece XZ düzleminde (düz zeminde) takip yönü hesaplama
        let mut direction = player_pos - enemy_pos;
        direction.y = 0.0;
        let direction_flat = direction.normalize_or_zero();

        match *ai_state {
            EnemyAIState::Idle => {
                velocity.linvel.x = 0.0;
                velocity.linvel.z = 0.0;
                // Y'ye dokunmuyoruz ki düşebilsin
                if distance <= ai.detection_range {
                    *ai_state = EnemyAIState::Chase;
                }
            }
            EnemyAIState::Chase => {
                if distance > ai.detection_range {
                    *ai_state = EnemyAIState::Idle;
                    velocity.linvel.x = 0.0;
                    velocity.linvel.z = 0.0;
                    // Y'ye dokunmuyoruz ki düşebilsin
                } else if distance <= ai.attack_range {
                    *ai_state = EnemyAIState::Attack;
                    velocity.linvel.x = 0.0;
                    velocity.linvel.z = 0.0;
                    // Y'ye dokunmuyoruz ki düşebilsin
                } else {
                    // Oyuncuya doğru fiziksel hız vererek ilerle
                    velocity.linvel = direction_flat * ai.speed;
                    
                    // Oyuncunun olduğu yöne doğru pürüzsüzce dön
                    let target_look = Vec3::new(player_pos.x, enemy_pos.y, player_pos.z);
                    transform.look_at(target_look, Vec3::Y);
                }
            }
            EnemyAIState::Attack => {
                velocity.linvel.x = 0.0;
velocity.linvel.z = 0.0;
// Y'ye dokunmuyoruz ki düşebilsin // Saldırırken ayakları sabit dursun
                
                // Saldırırken de oyuncuyu yüzüyle takip etsin
                let target_look = Vec3::new(player_pos.x, enemy_pos.y, player_pos.z);
                transform.look_at(target_look, Vec3::Y);

                // Eğer oyuncu kaçtıysa ve o sırada aktif bir vuruş evresinde değilse takibe geri dön
                if distance > ai.attack_range && matches!(*combat_state, CombatState::Idle) {
                    *ai_state = EnemyAIState::Chase;
                } 
                // Menzildeyse, cooldown bittiyse ve şu an boştaysa (Idle) saldırıyı başlat!
                else if matches!(*combat_state, CombatState::Idle) && ai.attack_cooldown.finished() {
                    let chosen_attack = AttackType::Thrust; 
                    
                    // 2. Silahın içindeki sözlükten o saldırının verisini çek
                    if let Some(attack_def) = weapon.attacks.get(&chosen_attack) {
                        
                        // 3. CombatState'i güncelle
                        *combat_state = CombatState::Windup {
                            timer: Timer::from_seconds(attack_def.windup_duration, TimerMode::Once), 
                            attack_type: chosen_attack
                        };
                    ai.attack_cooldown.reset(); // Cooldown'ı baştan kur
                }
            }
            }
            EnemyAIState::Dead => {
                velocity.linvel.x = 0.0;
velocity.linvel.z = 0.0;
// Y'ye dokunmuyoruz ki düşebilsin
            }
            EnemyAIState::Stagger => {
                velocity.linvel = Vec3::ZERO; // Sersemlediğinde hareket etmesin
            }
            }
        }
    
}


