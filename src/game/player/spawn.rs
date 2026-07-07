use super::components::{Player, PlayerMovementConfig, PlayerState,
    PendingEquipment};
use crate::game::combat::components::{AttackIntent, CombatState, EquippedWeapon, Hurtbox};
use crate::game::health::components::Health;
use crate::game::stamina::components::Stamina;
use crate::game::weapon::components::{WeaponType};
use crate::game::weapon::data::get_weapon_data;
use crate::game::shield::data::{get_shield_stats_bundle, build_shield_hierarchy, ShieldType};
use crate::game::animation::components::{CharacterBones, AnimationIntent};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::combat::components::GuardIntent;
use crate::game::shield::components::EquippedShield;
use crate::game::ik::components::IkTarget;
use crate::game::shield::components::BlockingState;


pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>, // Modeli yüklemek için AssetServer eklendi
) {
    let player_entity = commands.spawn_empty().id();

    // 3D Modeli yüklüyoruz
    let character_scene = asset_server.load("models/character.glb#Scene0");

    // Ana oyuncu kapsülü (Ebeveyn / Parent) - Artık görünmez bir mantık/fiziksel obje
    commands
        .entity(player_entity)
        .insert((

            GuardIntent::default(),
            IkTarget { 
                position: Vec3::ZERO, // Başlangıçta önemsiz
                weight: 0.0,          // Başlangıçta IK kapalı
            },
    
    ))
        .insert((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 3.0, 0.0),
                ..default()
            },
            Player,
            CharacterBones::default(),
            AnimationIntent::default(),
            PlayerState::default(),
            CombatState::default(), // Başlangıçta Idle
            AttackIntent::default(),

            
            Health::new(100.0), // Oyuncunun canı
            Hurtbox {
                owner: player_entity,
            }, // Oyuncunun hasar alma kutusu (sahibi kendisi)
            Stamina::default(),
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.5, 0.5), // Çarpışma motoru için fiziksel kapsül korunuyor
            KinematicCharacterController::default(),
            get_weapon_data(WeaponType::VampireBlade),
            get_shield_stats_bundle(ShieldType::KiteShield),
            
            
        ))
        .with_children(|parent| {
            
            // --- 3D MODEL (GLTF/GLB) ---
            parent.spawn(SceneBundle {
                scene: character_scene,
                // Kapsülün tam ortasında doğmaması, ayaklarının yere basması için -1.0 Y ofseti veriyoruz.
                // Kendi modelinin boyutuna göre bu ofseti ileride ayarlayabilirsin.
                transform: Transform::from_xyz(0.0, -1.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)), 
                ..default()
            });

            // --- SİLAH (Bağlanmayı bekliyor) ---
            parent.spawn((
                SpatialBundle {
                    // Silahın elde doğru durması için gereken rotasyon ofsetleri
                    // (Bu değerleri Mixamo rig'ine göre ileride ince ayar yapabiliriz)
                    transform: Transform::from_xyz(0.0, 0.0, 0.0)
                        .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
                    ..default()
                },
                EquippedWeapon,
                PendingEquipment, // Kemik sistemi yüklenince el kemiğine bağlanacak
            ));

            // --- KALKAN (Bağlanmayı bekliyor) ---
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                    ..default()
                },
                
                
                EquippedShield, 
                GuardIntent::default(),
                IkTarget { position: Vec3::ZERO, weight: 0.0 }, // <-- BU EKSİK OLABİLİR!
                BlockingState::Idle,
                
                PendingEquipment, // Kemik sistemi yüklenince sol kol kemiğine bağlanacak
                ShieldType::KiteShield,
                
            )).with_children(|shield_root| {
                // Kalkanın görsel hiyerarşisini bu geçici kökün içine inşa ediyoruz.
                // Tüm bu yapı daha sonra sol kola transfer edilecek.
                build_shield_hierarchy(shield_root, ShieldType::TowerShield, player_entity, &mut meshes, &mut materials);
            });
            
        });
}