use bevy::prelude::*;
use crate::game::combat::components::EquippedWeapon;
use crate::game::weapon::components::{Weapon, WeaponPart, WeaponChildrenCreated, WeaponType};
use crate::game::weapon::events::WeaponSwapRequestEvent;
use crate::game::weapon::data::get_weapon_data;

use crate::game::player::components::PendingEquipment; // <-- Bunu importların arasına ekle!

pub fn process_weapon_swap(
    mut events: EventReader<WeaponSwapRequestEvent>,
    mut commands: Commands,
    mut root_query: Query<&mut Weapon>,
    equipped_query: Query<Entity, With<EquippedWeapon>>,
    parent_query: Query<&Parent>,
) {
    for event in events.read() {
        if let Ok(mut weapon) = root_query.get_mut(event.entity) {
            println!("🔄 SWAP İSTEĞİ GELDİ! Şu anki Silah: {:?}", weapon.weapon_type);
            // İki silah arasında geçiş yap
            let next_weapon_type = match weapon.weapon_type {
                WeaponType::Spear => WeaponType::VampireBlade,
                _ => WeaponType::Spear,
            };
            println!("➡️ Hedeflenen Yeni Silah: {:?}", next_weapon_type);
            // 1. İstatistikleri ve görsel tanımları Root Entity'de güncelle
            *weapon = get_weapon_data(next_weapon_type);

            // 2. ESKİ SİLAHI TAMAMEN SİL (Sadece mesh'leri değil, tutucunun kendisini de yok et)
            for equipped_entity in equipped_query.iter() {
                let mut current_entity = equipped_entity;
                let mut is_owned_by_event_entity = false;

                while let Ok(parent) = parent_query.get(current_entity) {
                    if parent.get() == event.entity {
                        is_owned_by_event_entity = true;
                        break;
                    }
                    current_entity = parent.get();
                }

                if is_owned_by_event_entity {
                    // despawn_descendants yerine despawn_recursive kullanıyoruz!
                    commands.entity(equipped_entity).despawn_recursive();
                }
            }

            // 3. YENİ TEMİZ BİR SİLAH TUTUCU YARAT VE KEMİĞE BAĞLANMASINI İSTE
            let new_weapon_entity = commands.spawn((
                SpatialBundle {
                    // Silahın elde doğru açıyla durması için varsayılan rotasyon offseti
                    transform: Transform::from_xyz(0.0, 0.0, 0.0)
                        .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
                    ..default()
                },
                EquippedWeapon,
                PendingEquipment, // Animasyon modülümüz bunu görüp sağ ele bağlayacak!
            )).id();

            // Yeni silahı geçici olarak ana karaktere çocuk olarak ekliyoruz ki sistemler sahibini bilsin
            commands.entity(event.entity).add_child(new_weapon_entity);

            // 4. auto_spawn_weapon_visuals sistemini tetiklemek için marker'ı sil
            commands.entity(event.entity).remove::<WeaponChildrenCreated>();
        }
    }
}

pub fn auto_spawn_weapon_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Weapon), Without<WeaponChildrenCreated>>,
    equipped_query: Query<Entity, With<EquippedWeapon>>,
    parent_query: Query<&Parent>, // YENİ: Hiyerarşide yukarı çıkmak için
) {
    for (owner_entity, weapon) in query.iter() {
        
        // DEBUG 1: Sistem çalıştı ve silahsız bir karakter buldu!
        println!("🛠️ DEBUG: auto_spawn_weapon_visuals TETIKLENDI! Silah Sahibi (Owner): {:?}", owner_entity);
        let mut holder_entity = None;

        // Tüm EquippedWeapon'ları tara ve hangisinin bu 'owner_entity'ye ait olduğunu bul
        for equipped_entity in equipped_query.iter() {
            let mut current_entity = equipped_entity;
            
            // Yukarı doğru tırmanarak kontrol et
            while let Ok(parent) = parent_query.get(current_entity) {
                if parent.get() == owner_entity {
                    holder_entity = Some(equipped_entity);
                    break;
                }
                current_entity = parent.get();
            }

            if holder_entity.is_some() {
                break;
            }
        }

        // Eğer eli bulduysak, silah görsellerini karakterin kendisine değil, eline (holder_entity) ekle!
        if let Some(holder) = holder_entity {
            // DEBUG 2: Hiyerarşide tırmanıp el kemiğini bulmayı başardı!
            println!("✅ DEBUG: Silah tutucu (El/Holder) BULUNDU! Holder Entity: {:?}", holder);
            commands.entity(holder).with_children(|parent| {
                for (part, transform) in &weapon.visuals.parts {
                    match part {
                        WeaponPart::Cylinder { radius, height, color } => {
                            parent.spawn(PbrBundle {
                                mesh: meshes.add(Cylinder { radius: *radius, half_height: *height / 2.0 }),
                                material: materials.add(*color),
                                transform: *transform,
                                ..default()
                            });
                        }
                        WeaponPart::Cone { radius, height, color } => {
                            parent.spawn(PbrBundle {
                                mesh: meshes.add(Cone { radius: *radius, height: *height }),
                                material: materials.add(*color),
                                transform: *transform,
                                ..default()
                            });
                        }
                    }
                }
            });

            // İşlemi bir kez yap ve ana karakteri işaretle
            commands.entity(owner_entity).insert(WeaponChildrenCreated);
            println!("✅ DEBUG: Silah görselleri eklendi ve karakter WeaponChildrenCreated ile işaretlendi.");
            
        } else {
            // DEBUG 3: Karakter var ama EquippedWeapon etiketli bir çocuk bulunamadı!
            println!("❌ DEBUG HATA: Karakter ({:?}) için 'EquippedWeapon' taşıyan bir alt obje BULUNAMADI!", owner_entity);
        }
    }
}