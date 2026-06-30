use bevy::prelude::*;
use crate::game::combat::components::EquippedWeapon;


use crate::game::weapon::components::{Weapon, WeaponPart,
     WeaponChildrenCreated , WeaponType};
use crate::game::weapon::events::WeaponSwapRequestEvent;
use crate::game::weapon::data::get_weapon_data;

pub fn process_weapon_swap(
    mut events: EventReader<WeaponSwapRequestEvent>,
    mut commands: Commands,
    mut root_query: Query<&mut Weapon>,
    equipped_query: Query<(Entity, &Parent), With<EquippedWeapon>>,
) {
    for event in events.read() {
        if let Ok(mut weapon) = root_query.get_mut(event.entity) {
            // İki silah arasında geçiş yap
            let next_weapon_type = match weapon.weapon_type {
                WeaponType::Spear => WeaponType::VampireBlade,
                _ => WeaponType::Spear,
            };

            // 1. İstatistikleri ve görsel tanımları Root Entity'de güncelle
            *weapon = get_weapon_data(next_weapon_type);

            // 2. Eski görselleri temizle
            for (equipped_entity, parent) in equipped_query.iter() {
                // Eğer silah tutucu, eventi fırlatan entity'nin çocuğuysa
                if parent.get() == event.entity {
                    // Sadece silahın kendisini değil, altındaki tüm meshleri sil (Despawn Descendants)
                    commands.entity(equipped_entity).despawn_descendants();
                }
            }

            // 3. auto_spawn_weapon_visuals sistemini yeniden tetiklemek için marker'ı sil
            commands.entity(event.entity).remove::<WeaponChildrenCreated>();
        }
    }
}


pub fn auto_spawn_weapon_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Karakterin çocuklarına (&Children) bakabilmek için query'i güncelledik
    query: Query<(Entity, &Weapon, &Children), Without<WeaponChildrenCreated>>,
    weapon_holder_query: Query<Entity, With<EquippedWeapon>>,
) {
    for (entity, weapon, children) in query.iter() {
        
        // Karakterin çocukları arasında 'EquippedWeapon' olanı (yani eli) bul
        let mut holder_entity = None;
        for &child in children.iter() {
            if weapon_holder_query.contains(child) {
                holder_entity = Some(child);
                break;
            }
        }

        // Eğer eli bulduysak, silah görsellerini karakterin kendisine değil, eline (holder_entity) ekle!
        if let Some(holder) = holder_entity {
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
            commands.entity(entity).insert(WeaponChildrenCreated);
        }
    }
}