use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use crate::game::world::mesh::marching_cubes::SurfaceData;

pub fn build_mesh(surface: SurfaceData) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(surface.vertices.len());
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(surface.vertices.len());
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(surface.vertices.len());
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(surface.vertices.len());
    let mut indices: Vec<u32> = Vec::with_capacity(surface.vertices.len());

    let texture_scale = 0.25;
    let mut i = 0;

    while i < surface.vertices.len() {
        if i + 2 >= surface.vertices.len() { break; }

        let v0 = surface.vertices[i];
        let v1 = surface.vertices[i + 1];
        let v2 = surface.vertices[i + 2];

        // UV hesaplaması ve triplanar projeksiyon için yüzeyin geometrik (flat) normali hala gereklidir
        let face_normal = (v2 - v0).cross(v1 - v0).normalize_or_zero();

        positions.push(v0.into());
        positions.push(v1.into());
        positions.push(v2.into());

        // KÖK NEDEN ÇÖZÜMÜ: Flat shading (face_normal) yerine, marching_cubes.rs'ten gelen
        // analitik/pürüzsüz normalleri mesh'e işliyoruz. Böylece yüzey ışıklandırma hesaplamaları
        // doğru veriyi kullanarak yumuşak gölgeler üretebilecek.
        normals.push(surface.normals[i].into());
        normals.push(surface.normals[i + 1].into());
        normals.push(surface.normals[i + 2].into());

        if surface.colors.len() == surface.vertices.len() {
            colors.push(surface.colors[i]);
            colors.push(surface.colors[i + 1]);
            colors.push(surface.colors[i + 2]);
        } else {
            colors.push([1.0, 1.0, 1.0, 1.0]);
            colors.push([1.0, 1.0, 1.0, 1.0]);
            colors.push([1.0, 1.0, 1.0, 1.0]);
        }

        let calc_uv = |p: Vec3, n: Vec3| -> [f32; 2] {
            let abs_n = n.abs();
            if abs_n.x > abs_n.y && abs_n.x > abs_n.z {
                [p.z * texture_scale, p.y * texture_scale]
            } else if abs_n.y > abs_n.x && abs_n.y > abs_n.z {
                [p.x * texture_scale, p.z * texture_scale]
            } else {
                [p.x * texture_scale, p.y * texture_scale]
            }
        };

        // Kaplama (UV) koordinatları yüzeyin baktığı ana yöne göre yansıtılır
        uvs.push(calc_uv(v0, face_normal));
        uvs.push(calc_uv(v1, face_normal));
        uvs.push(calc_uv(v2, face_normal));

        // Index swap ile culling yönünü hizalama işlemi korunmuştur
        indices.push(i as u32);
        indices.push((i as u32) + 2);
        indices.push((i as u32) + 1);

        i += 3;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    
    mesh
}