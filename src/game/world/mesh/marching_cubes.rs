use bevy::prelude::*;
use crate::game::world::generation::field::ScalarField;
use super::tables::{EDGE_TABLE, TRI_TABLE};

pub struct SurfaceData {
    pub vertices: Vec<Vec3>,
    pub colors: Vec<[f32; 4]>,
    pub normals: Vec<Vec3>,
}

/// Çevre hücrelerin yoğunluk farkına bakarak yüzeyin gerçek dışa dönük normalini hesaplar
fn estimate_normal(field: &ScalarField, x: u32, y: u32, z: u32) -> Vec3 {
    let x0 = field.get_density(x.saturating_sub(1), y, z);
    let x1 = field.get_density((x + 1).min(field.padded_size.x - 1), y, z);
    let y0 = field.get_density(x, y.saturating_sub(1), z);
    let y1 = field.get_density(x, (y + 1).min(field.padded_size.y - 1), z);
    let z0 = field.get_density(x, y, z.saturating_sub(1));
    let z1 = field.get_density(x, y, (z + 1).min(field.padded_size.z - 1));

    // KÖK NEDEN ÇÖZÜMÜ (INVERTED NORMALS):
    // Yoğunluğun AZALDIĞI yön, kayanın içinden boşluğa (havaya) doğru olan yöndür.
    // x1 - x0 yaparsak vektör kayanın içine bakar ve yüzeyler PBR motorda zifiri karanlık olur.
    // x0 - x1 yaparak normali dışarıya, ışığın geldiği yöne çeviriyoruz.
    Vec3::new(x0 - x1, y0 - y1, z0 - z1).normalize_or_zero()
}

fn interpolate_vertex(
    isolevel: f32, 
    p1: Vec3, p2: Vec3, 
    val_p1: f32, val_p2: f32, 
    c1: Vec3, c2: Vec3,
    n1: Vec3, n2: Vec3
) -> (Vec3, [f32; 4], Vec3) {
    if (isolevel - val_p1).abs() < 0.00001 { return (p1, [c1.x, c1.y, c1.z, 1.0], n1); }
    if (isolevel - val_p2).abs() < 0.00001 { return (p2, [c2.x, c2.y, c2.z, 1.0], n2); }
    if (val_p1 - val_p2).abs() < 0.00001 { return (p1, [c1.x, c1.y, c1.z, 1.0], n1); }

    let t = (isolevel - val_p1) / (val_p2 - val_p1);
    let p = p1 + (p2 - p1) * t;
    let c = c1 + (c2 - c1) * t;
    let n = (n1 + (n2 - n1) * t).normalize_or_zero();
    (p, [c.x, c.y, c.z, 1.0], n)
}

pub fn extract_surface(field: &ScalarField, isolevel: f32, step: f32) -> SurfaceData {
    let mut vertices = Vec::new();
    let mut colors = Vec::new(); 
    let mut normals = Vec::new(); 

    for z in 0..field.logical_size.z {
        for y in 0..field.logical_size.y {
            for x in 0..field.logical_size.x {
                
                let mut corners = [Vec3::ZERO; 8];
                let mut values = [0.0; 8];
                let mut light_vals = [Vec3::ZERO; 8]; 
                let mut norm_vals = [Vec3::ZERO; 8]; 

                let corner_offsets = [
                    UVec3::new(0, 0, 0), UVec3::new(1, 0, 0), UVec3::new(1, 0, 1), UVec3::new(0, 0, 1),
                    UVec3::new(0, 1, 0), UVec3::new(1, 1, 0), UVec3::new(1, 1, 1), UVec3::new(0, 1, 1),
                ];
                
                for i in 0..8 {
                    let offset = corner_offsets[i];
                    let cx = x + offset.x;
                    let cy = y + offset.y;
                    let cz = z + offset.z;
                    light_vals[i] = field.get_light(cx, cy, cz);
                    values[i] = field.get_density(cx, cy, cz);
                    norm_vals[i] = estimate_normal(field, cx, cy, cz); 
                    corners[i] = field.world_origin + Vec3::new(cx as f32, cy as f32, cz as f32) * step;
                }

                let mut cube_index = 0;
                if values[0] < isolevel { cube_index |= 1; }
                if values[1] < isolevel { cube_index |= 2; }
                if values[2] < isolevel { cube_index |= 4; }
                if values[3] < isolevel { cube_index |= 8; }
                if values[4] < isolevel { cube_index |= 16; }
                if values[5] < isolevel { cube_index |= 32; }
                if values[6] < isolevel { cube_index |= 64; }
                if values[7] < isolevel { cube_index |= 128; }

                if EDGE_TABLE[cube_index] == 0 {
                    continue;
                }

                let mut vert_list = [Vec3::ZERO; 12];
                let mut color_list = [[0.0; 4]; 12];
                let mut norm_list = [Vec3::ZERO; 12]; 
                let edges = EDGE_TABLE[cube_index];

                if (edges & 1) != 0 { let r = interpolate_vertex(isolevel, corners[0], corners[1], values[0], values[1], light_vals[0], light_vals[1], norm_vals[0], norm_vals[1]); vert_list[0] = r.0; color_list[0] = r.1; norm_list[0] = r.2; }
                if (edges & 2) != 0 { let r = interpolate_vertex(isolevel, corners[1], corners[2], values[1], values[2], light_vals[1], light_vals[2], norm_vals[1], norm_vals[2]); vert_list[1] = r.0; color_list[1] = r.1; norm_list[1] = r.2; }
                if (edges & 4) != 0 { let r = interpolate_vertex(isolevel, corners[2], corners[3], values[2], values[3], light_vals[2], light_vals[3], norm_vals[2], norm_vals[3]); vert_list[2] = r.0; color_list[2] = r.1; norm_list[2] = r.2; }
                if (edges & 8) != 0 { let r = interpolate_vertex(isolevel, corners[3], corners[0], values[3], values[0], light_vals[3], light_vals[0], norm_vals[3], norm_vals[0]); vert_list[3] = r.0; color_list[3] = r.1; norm_list[3] = r.2; }
                if (edges & 16) != 0 { let r = interpolate_vertex(isolevel, corners[4], corners[5], values[4], values[5], light_vals[4], light_vals[5], norm_vals[4], norm_vals[5]); vert_list[4] = r.0; color_list[4] = r.1; norm_list[4] = r.2; }
                if (edges & 32) != 0 { let r = interpolate_vertex(isolevel, corners[5], corners[6], values[5], values[6], light_vals[5], light_vals[6], norm_vals[5], norm_vals[6]); vert_list[5] = r.0; color_list[5] = r.1; norm_list[5] = r.2; }
                if (edges & 64) != 0 { let r = interpolate_vertex(isolevel, corners[6], corners[7], values[6], values[7], light_vals[6], light_vals[7], norm_vals[6], norm_vals[7]); vert_list[6] = r.0; color_list[6] = r.1; norm_list[6] = r.2; }
                if (edges & 128) != 0 { let r = interpolate_vertex(isolevel, corners[7], corners[4], values[7], values[4], light_vals[7], light_vals[4], norm_vals[7], norm_vals[4]); vert_list[7] = r.0; color_list[7] = r.1; norm_list[7] = r.2; }
                if (edges & 256) != 0 { let r = interpolate_vertex(isolevel, corners[0], corners[4], values[0], values[4], light_vals[0], light_vals[4], norm_vals[0], norm_vals[4]); vert_list[8] = r.0; color_list[8] = r.1; norm_list[8] = r.2; }
                if (edges & 512) != 0 { let r = interpolate_vertex(isolevel, corners[1], corners[5], values[1], values[5], light_vals[1], light_vals[5], norm_vals[1], norm_vals[5]); vert_list[9] = r.0; color_list[9] = r.1; norm_list[9] = r.2; }
                if (edges & 1024) != 0 { let r = interpolate_vertex(isolevel, corners[2], corners[6], values[2], values[6], light_vals[2], light_vals[6], norm_vals[2], norm_vals[6]); vert_list[10] = r.0; color_list[10] = r.1; norm_list[10] = r.2; }
                if (edges & 2048) != 0 { let r = interpolate_vertex(isolevel, corners[3], corners[7], values[3], values[7], light_vals[3], light_vals[7], norm_vals[3], norm_vals[7]); vert_list[11] = r.0; color_list[11] = r.1; norm_list[11] = r.2; }

                for i in (0..16).step_by(3) {
                    if TRI_TABLE[cube_index][i] == -1 { break; }
                    
                    let idx0 = TRI_TABLE[cube_index][i] as usize;
                    let idx1 = TRI_TABLE[cube_index][i + 1] as usize;
                    let idx2 = TRI_TABLE[cube_index][i + 2] as usize;

                    vertices.push(vert_list[idx0]);
                    vertices.push(vert_list[idx1]);
                    vertices.push(vert_list[idx2]);

                    colors.push(color_list[idx0]);
                    colors.push(color_list[idx1]);
                    colors.push(color_list[idx2]);

                    normals.push(norm_list[idx0]);
                    normals.push(norm_list[idx1]);
                    normals.push(norm_list[idx2]);
                }
            }
        }
    }

    SurfaceData { vertices, colors, normals }
}