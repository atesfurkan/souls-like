use crate::game::combat::components::EquippedWeapon;
use crate::game::weapon::components::{Weapon, WeaponChildrenCreated, WeaponPart, WeaponType};
use crate::game::weapon::data::get_weapon_data;
use crate::game::weapon::events::WeaponSwapRequestEvent;
use bevy::prelude::*;
use crate::game::player::components::PendingEquipment; 

pub fn process_weapon_swap(
    mut events: EventReader<WeaponSwapRequestEvent>,
    mut commands: Commands,
    equipped_query: Query<Entity, With<EquippedWeapon>>,
    parent_query: Query<&Parent>,
) {
    for event in events.read() {
        println!("🔄 SWAP İSTEĞİ GELDİ! HEDEF: {:?}", event.target_weapon);
        
        // 1. ESKİ SİLAHI (varsa) TAMAMEN SİL
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
                commands.entity(equipped_entity).despawn_recursive();
            }
        }

        // Ana entity üzerindeki weapon verilerini güvenle sil
        commands.entity(event.entity).remove::<Weapon>();
        commands.entity(event.entity).remove::<WeaponChildrenCreated>();

        // 2. EĞER YENİ BİR SİLAH İSTENMİŞSE EKLE
        if let Some(next_weapon_type) = event.target_weapon {
            let weapon_data = get_weapon_data(next_weapon_type);
            commands.entity(event.entity).insert(weapon_data);

            let new_weapon_entity = commands
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0)
                            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
                        ..default()
                    },
                    EquippedWeapon,
                    PendingEquipment,
                ))
                .id();

            commands.entity(event.entity).add_child(new_weapon_entity);
        } else {
            println!("✅ Silah çıkarıldı (Sağ El: Unarmed).");
        }
    }
}

pub fn auto_spawn_weapon_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Weapon), Without<WeaponChildrenCreated>>,
    equipped_query: Query<Entity, With<EquippedWeapon>>,
    parent_query: Query<&Parent>,
) {
    for (owner_entity, weapon) in query.iter() {
        println!(
            "🛠️ DEBUG: auto_spawn_weapon_visuals TETIKLENDI! Silah Sahibi (Owner): {:?}",
            owner_entity
        );
        let mut holder_entity = None;

        for equipped_entity in equipped_query.iter() {
            let mut current_entity = equipped_entity;
            while let Ok(parent) = parent_query.get(current_entity) {
                if parent.get() == owner_entity {
                    holder_entity = Some(equipped_entity);
                    break;
                }
                current_entity = parent.get();
            }
            if holder_entity.is_some() { break; }
        }

        if let Some(holder) = holder_entity {
            println!(
                "✅ DEBUG: Silah tutucu (El/Holder) BULUNDU! Holder Entity: {:?}",
                holder
            );
            commands.entity(holder).with_children(|parent| {
                for (part, transform) in &weapon.visuals.parts {
                    match part {
                        WeaponPart::Cylinder { radius, height, color } => {
                            parent.spawn(PbrBundle {
                                mesh: meshes.add(Cylinder {
                                    radius: *radius,
                                    half_height: *height / 2.0,
                                }),
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
                        WeaponPart::Cuboid { x, y, z, color } => {
                            parent.spawn(PbrBundle {
                                mesh: meshes.add(Cuboid::new(*x, *y, *z)),
                                material: materials.add(*color),
                                transform: *transform,
                                ..default()
                            });
                        }
                    }
                }
            });
            commands.entity(owner_entity).insert(WeaponChildrenCreated);
        }
    }
}