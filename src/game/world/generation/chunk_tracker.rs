use bevy::prelude::*;
use std::collections::HashSet;
use crate::game::world::generation::world_config::WorldGenerationConfig;

/// Bu bileşene sahip olan entity (genellikle kamera veya oyuncu), 
/// dünyadaki aktif chunk'ların merkezini belirler.
#[derive(Component)]
pub struct ChunkViewer;

#[derive(Event, Debug, Clone, Copy)]
pub struct ChunkLoadEvent(pub IVec3);

#[derive(Event, Debug, Clone, Copy)]
pub struct ChunkUnloadEvent(pub IVec3);

#[derive(Resource, Default)]
pub struct ActiveChunks {
    pub chunks: HashSet<IVec3>,
    pub last_viewer_chunk: Option<IVec3>,
}

/// Herhangi bir kameraya otomatik olarak ChunkViewer ekler. 
/// Bu sayede camera kodlarına dokunmamıza gerek kalmaz.
pub fn attach_chunk_viewer_to_camera(
    mut commands: Commands,
    query: Query<Entity, (With<Camera>, Without<ChunkViewer>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(ChunkViewer);
    }
}

pub fn chunk_tracking_system(
    viewer_query: Query<&Transform, With<ChunkViewer>>,
    config: Res<WorldGenerationConfig>,
    mut active_chunks: ResMut<ActiveChunks>,
    mut load_writer: EventWriter<ChunkLoadEvent>,
    mut unload_writer: EventWriter<ChunkUnloadEvent>,
) {
    let Ok(viewer_transform) = viewer_query.get_single() else {
        return;
    };

    let pos = viewer_transform.translation;
    let chunk_x = (pos.x / config.chunk_logical_size.x as f32).floor() as i32;
    let chunk_y = (pos.y / config.chunk_logical_size.y as f32).floor() as i32;
    let chunk_z = (pos.z / config.chunk_logical_size.z as f32).floor() as i32;
    
    let current_chunk = IVec3::new(chunk_x, chunk_y, chunk_z);

    // Eğer viewer aynı chunk içindeyse işlem yapmaya gerek yok
    if active_chunks.last_viewer_chunk == Some(current_chunk) {
        return;
    }

    active_chunks.last_viewer_chunk = Some(current_chunk);
    
    let rd = config.render_distance as i32;
    let ud = config.unload_distance as i32;

    let mut current_visible = HashSet::new();

    // Görüş mesafesindeki chunk'ları hesapla
    for x in -rd..=rd {
        for y in -rd..=rd {
            for z in -rd..=rd {
                let offset = IVec3::new(x, y, z);
                current_visible.insert(current_chunk + offset);
            }
        }
    }

    // Yüklenmesi gereken YENİ chunk'ları bul (görünür olanlar - zaten aktif olanlar)
    let chunks_to_load: Vec<IVec3> = current_visible
        .difference(&active_chunks.chunks)
        .cloned()
        .collect();

    for chunk_coord in chunks_to_load {
        active_chunks.chunks.insert(chunk_coord);
        load_writer.send(ChunkLoadEvent(chunk_coord));
        //info!("ChunkLoadEvent gönderildi: {:?}", chunk_coord);
    }

    // Silinmesi gereken UZAK chunk'ları bul
    let chunks_to_unload: Vec<IVec3> = active_chunks.chunks
        .iter()
        .filter(|&&c| {
            let diff = c - current_chunk;
            diff.x.abs() > ud || diff.y.abs() > ud || diff.z.abs() > ud
        })
        .cloned()
        .collect();

    for chunk_coord in chunks_to_unload {
        active_chunks.chunks.remove(&chunk_coord);
        unload_writer.send(ChunkUnloadEvent(chunk_coord));
        //info!("ChunkUnloadEvent gönderildi: {:?}", chunk_coord);
    }
}
