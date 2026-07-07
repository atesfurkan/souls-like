use bevy::prelude::*; // Component, Entity, Bundle, Color, Transform vb. için gerekli
use bevy_rapier3d::prelude::*; // Collider, Sensor, ActiveEvents için gerekli
use super::components::{BlockingState, EquippedShield, Shield,
    ShieldBox , ShieldVisualAnchor};

// Oyundaki tüm kalkan türlerini tanımlayan liste
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShieldType {
    Buckler,
    KiteShield,
    TowerShield,
}

// 3D Assetlere geçişi kolaylaştırmak için görsel veri yapısı
pub enum ShieldVisualData {
    // Şimdilik test için kullanacağımız "Programmer Art"
    ProgrammerArt {
        shape: DefaultShieldShape,
        color: Color,
    },
    // İleride buraya GltfModel eklenecek
}

// Basit mesh şekilleri
pub enum DefaultShieldShape {
    Box { width: f32, height: f32, depth: f32 },
    Cylinder { radius: f32, height: f32 },
}

// Kalkan oluşturulurken dışarıya aktarılacak paket (Template)
pub struct ShieldTemplate {
    pub stats: Shield,
    pub visual: ShieldVisualData,
}

// Kalkan fabrikası: İstenen kalkan türüne göre veriyi üretir
pub fn get_shield_data(shield_type: ShieldType) -> ShieldTemplate {
    match shield_type {
        ShieldType::Buckler => ShieldTemplate {
            stats: Shield {
                block_rate: 0.65,
                stamina_cost_multiplier: 1.5,
                parry_window: 0.35,
            },
            visual: ShieldVisualData::ProgrammerArt {
                shape: DefaultShieldShape::Cylinder { radius: 0.25, height: 0.05 },
                color: Color::srgb(0.6, 0.4, 0.2),
            },
        },
        ShieldType::KiteShield => ShieldTemplate {
            stats: Shield {
                block_rate: 1.0,
                stamina_cost_multiplier: 0.8,
                parry_window: 0.15,
            },
            visual: ShieldVisualData::ProgrammerArt {
                shape: DefaultShieldShape::Box { width: 0.4, height: 0.6, depth: 0.05 },
                color: Color::srgb(0.7, 0.7, 0.8),
            },
        },
        ShieldType::TowerShield => ShieldTemplate {
            stats: Shield {
                block_rate: 1.0,
                stamina_cost_multiplier: 0.2,
                parry_window: 0.0,
            },
            visual: ShieldVisualData::ProgrammerArt {
                shape: DefaultShieldShape::Box { width: 0.6, height: 1.2, depth: 0.15 },
                color: Color::srgb(0.2, 0.2, 0.25),
            },
        },
    }
}

// 1. Ana Karakterin (Player/Enemy) üzerine eklenecek stat paketi
pub fn get_shield_stats_bundle(shield_type: ShieldType) -> (Shield, BlockingState) {
    let template = get_shield_data(shield_type);
    (
        template.stats,
        BlockingState::default(),
    )
}

// 2. Çocuğa (Child Entity) eklenecek fiziksel ve görsel HİYERARŞİ paketi
pub fn build_shield_hierarchy(
    builder: &mut ChildBuilder,
    shield_type: ShieldType,
    owner: Entity,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let template = get_shield_data(shield_type);

    // Kalkan gövdesinin şeklini ve collider'ını belirliyoruz
    let (collider, mesh, material, board_offset) = match &template.visual {
        ShieldVisualData::ProgrammerArt { shape, color } => {
            let mat = materials.add(StandardMaterial {
                base_color: *color,
                ..default()
            });

            match shape {
                DefaultShieldShape::Box { width, height, depth } => (
                    Collider::cuboid(*width / 2.0, *height / 2.0, *depth / 2.0),
                    meshes.add(Cuboid::new(*width, *height, *depth)),
                    mat,
                    Vec3::new(0.0, 0.0, 0.1), // Gövdeyi koldan +Z ekseninde biraz dışarı itiyoruz
                ),
                DefaultShieldShape::Cylinder { radius, height } => (
                    Collider::cylinder(*height / 2.0, *radius),
                    meshes.add(Cylinder { radius: *radius, half_height: *height / 2.0 }),
                    mat,
                    Vec3::new(0.0, 0.0, 0.05), // Buckler ince olduğu için daha az itiyoruz
                ),
            }
        }
    };

    // --- 1. ANCHOR (Çıpa) ---
    // Ana kalkan entity'si: Çarpışma kutusu buradadır, ancak görünmez bir noktadır (SpatialBundle).
    // Dışarıdaki sistemler (Player) bu çıpayı doğrudan ön kola takar.
    builder.spawn((
        ShieldVisualAnchor,
        ShieldBox { owner },
        collider,
        
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        SpatialBundle {
        // TÜM KALKANI DÖNDÜREN NOKTA BURASI
        // Bu root entity olduğu için, buradaki rotasyon tüm çocuklarını (board, handle, strap) etkiler.
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 3.0, 3.0)),
        ..default()
    },
         
    )).with_children(|shield_anchor| {
        
        // --- 2. BOARD (Tahta Gövde) ---
        // Kalkanın asıl yüzeyi. Çıpadan biraz dışarıda durur ki kol içine girmesin.
        shield_anchor.spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(board_offset)
               .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..default()
        });

        // Ortak Siyah Deri/Metal Materyali (Kayış ve Tutamaç için)
        let black_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.05, 0.05),
            ..default()
        });

        // --- 3. STRAP (Ön Kol Kayışı) ---
        // Tam çıpa noktasında bulunur. Karakterin kolu bu halkanın içinden geçiyormuş gibi görünür.
        shield_anchor.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.12, 0.04, 0.12)),
            material: black_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0), // Tam çıpada
            ..default()
        });

        // --- 4. HANDLE (El Tutamacı) ---
        // Çıpadan aşağıya (Y ekseninde eksiye) kaydırılmıştır. Karakterin yumruğuna denk gelir.
        shield_anchor.spawn(PbrBundle {
            mesh: meshes.add(Cylinder { radius: 0.02, half_height: 0.08 }),
            material: black_material,
            // Y ekseninde aşağıya ittik ve silindiri yan çevirdik (X ekseninde döndürdük)
            transform: Transform::from_xyz(0.0, -0.25, 0.05)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
    });
}