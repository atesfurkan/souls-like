#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var wall_texture: texture_2d<f32>;
@group(2) @binding(1) var wall_sampler: sampler;
@group(2) @binding(2) var floor_texture: texture_2d<f32>;
@group(2) @binding(3) var floor_sampler: sampler;
@group(2) @binding(4) var ceiling_texture: texture_2d<f32>;
@group(2) @binding(5) var ceiling_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    
    // 1. Doku Ölçeği (Texture Scale)
    // Sünme veya çok tekrar etme (tiling) varsa bu sayıyla oyna. 
    // Rakamı küçültürsen (ör: 0.1) texture büyür, büyütürsen (ör: 1.0) texture sıklaşır.
    let scale = 0.25; 
    let pos = mesh.world_position.xyz * scale;
    
    // 2. Yüzeyin Normalini (Baktığı Yönü) Al
    let normal = normalize(mesh.world_normal);
    
    // 3. Karışım (Blend) Ağırlıklarını Hesapla
    // Keskin kenarlarda dokuların birbirine yumuşak geçmesi için gücü artırıyoruz (pow)
    var blend = abs(normal);
    blend = pow(blend, vec3<f32>(4.0)); // 4.0 değeri geçişlerin keskinliğini belirler
    blend = blend / (blend.x + blend.y + blend.z); // Toplamı 1.0 olacak şekilde normalize et

    // 4. Üç Eksen İçin (X, Y, Z) Doku Örnekleri Al
    // Z ekseninden bakıldığında (Duvar)
    let color_z = textureSample(wall_texture, wall_sampler, pos.xy);
    
    // X ekseninden bakıldığında (Duvar)
    let color_x = textureSample(wall_texture, wall_sampler, pos.zy);
    
    // Y ekseninden bakıldığında (Zemin veya Tavan ayrımı)
    var color_y: vec4<f32>;
    if normal.y > 0.0 {
        // Yüzey yukarı bakıyor (Zemin/Floor)
        color_y = textureSample(floor_texture, floor_sampler, pos.xz);
    } else {
        // Yüzey aşağı bakıyor (Tavan/Ceiling)
        color_y = textureSample(ceiling_texture, ceiling_sampler, pos.xz);
    }

    // 5. Üç eksenden gelen renkleri, yüzeyin normal ağırlığına göre birbirine yedir
    let final_color = color_x * blend.x + color_y * blend.y + color_z * blend.z;
    
    return final_color;
}
