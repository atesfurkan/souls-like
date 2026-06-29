use bevy::prelude::*;
use crate::game::weapon::components::{Weapon, WeaponPart, WeaponChildrenCreated};
use crate::game::combat::components::EquippedWeapon;


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