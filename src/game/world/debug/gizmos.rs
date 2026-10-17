use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::player::systems::bone_proxies::BoneColliderMarker;
use crate::game::world::generation::layout::CaveLayout;

pub fn draw_reference_grid(mut gizmos: Gizmos) {
    // Ana Eksenler (X = Kırmızı, Y = Yeşil, Z = Mavi)
    gizmos.line(Vec3::new(-100.0, 0.0, 0.0), Vec3::new(100.0, 0.0, 0.0), Color::srgb(1.0, 0.0, 0.0));
    gizmos.line(Vec3::new(0.0, -100.0, 0.0), Vec3::new(0.0, 100.0, 0.0), Color::srgb(0.0, 1.0, 0.0));
    gizmos.line(Vec3::new(0.0, 0.0, -100.0), Vec3::new(0.0, 0.0, 100.0), Color::srgb(0.0, 0.5, 1.0));

    // Zemin Izgarası
    let grid_size = 20;
    let step = 5.0;
    let grid_color = Color::srgba(0.5, 0.5, 0.5, 0.2);

    for i in -grid_size..=grid_size {
        if i == 0 {
            continue;
        }
        let offset = i as f32 * step;

        // X eksenine paralel çizgiler
        gizmos.line(
            Vec3::new(offset, 0.0, -grid_size as f32 * step),
            Vec3::new(offset, 0.0, grid_size as f32 * step),
            grid_color,
        );

        // Z eksenine paralel çizgiler
        gizmos.line(
            Vec3::new(-grid_size as f32 * step, 0.0, offset),
            Vec3::new(grid_size as f32 * step, 0.0, offset),
            grid_color,
        );
    }
}

pub fn draw_layout_graph(mut gizmos: Gizmos, layout_res: Option<Res<CaveLayout>>) {
    if let Some(layout) = layout_res {
        for room in &layout.rooms {
            gizmos.sphere(room.center, Quat::IDENTITY, room.radius, Color::srgb(0.0, 0.5, 1.0));
        }

        for corridor in &layout.corridors {
            let from_room = layout.rooms.iter().find(|r| r.id == corridor.from_room_id);
            let to_room = layout.rooms.iter().find(|r| r.id == corridor.to_room_id);

            if let (Some(from), Some(to)) = (from_room, to_room) {
                gizmos.line(from.center, to.center, Color::srgb(1.0, 1.0, 0.0));
            }
        }
    }
}

pub fn draw_bone_colliders(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &BoneColliderMarker)>,
) {
    for (global_transform, marker) in query.iter() {
        let transform = global_transform.compute_transform();
        let color = Color::srgba(0.1, 0.9, 0.2, 0.8);

        gizmos.sphere(transform.translation, transform.rotation, 0.03, Color::srgb(1.0, 1.0, 0.0));

        match marker.bone_name {
            crate::game::animation::systems::BONE_RIGHT_HAND => {
                gizmos.cuboid(transform, color);
            }
            _ => {
                let forward = transform.rotation * Vec3::Z;
                let length = 0.2;
                gizmos.line(
                    transform.translation - forward * (length * 0.5),
                    transform.translation + forward * (length * 0.5),
                    color,
                );
                gizmos.sphere(transform.translation, transform.rotation, 0.08, color);
            }
        }
    }
}