use bevy::prelude::*;

use super::analysis::AnalysisPlugin;
use super::generation::GenerationPlugin;
use super::mesh::MeshGenPlugin;
use super::physics::PhysicsPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Procedural World Sistemleri
        app.add_plugins(GenerationPlugin)
            .add_plugins(MeshGenPlugin)
            .add_plugins(PhysicsPlugin)
            .add_plugins(AnalysisPlugin);

        // Global ışık sistemleri startup aşamasında kaydediliyor
        app.add_systems(Startup, setup_global_light)
           .add_systems(Startup, spawn_test_lights);
    }
}

fn setup_global_light(mut commands: Commands) {
    // 1. Ortam Işığı (Ambient Light)
    // Bevy 0.14 PBR standartlarına uygun olarak mağara içi ortam aydınlatması ayarlanıyor.
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.85, 0.85, 0.89), // Hafif soğuk mağara tonu
        brightness: 500.0,                   // Dengeli ortam parlaklığı
    });

    // 2. Yönlü Işık (Directional Light / Güneş)
    // Bevy 0.14'te lüks (lux) karşılığı standart güneş ışığı ~100,000.0 değerindedir.
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_00.0, // Doğru PBR güneş şiddeti
            shadows_enabled: true,  // Gölgeler aktif
            ..default()
        },
        // Işığın geliş açısı: Çaprazdan ve tepeden aşağıya doğru
        transform: Transform::from_xyz(50.0, 100.0, 50.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Sahneye ışıklandırma ve gölge testleri için 3 farklı noktadan PointLight ekler.
pub fn spawn_test_lights(mut commands: Commands) {
    // 1. Merkezde Yüksek Yoğunluklu Sıcak Işık (Ana Aydınlatma)
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb(1.0, 0.85, 0.7), // Sıcak sarımtırak
            intensity: 15000.0,                 // Bevy 0.14 PointLight şiddeti
            range: 150.0,                       // Işığın ulaşacağı maksimum mesafe
            shadows_enabled: true,              // Gölgeler aktif
            ..default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 0.0),
        ..default()
    });

    // 2. Sağ İleride Soğuk (Mavi) Işık (Kontrast Yaratmak İçin)
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb(0.5, 0.7, 1.0), // Soğuk mavi
            intensity: 8000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(20.0, 10.0, 20.0),
        ..default()
    });

    // 3. Sol Geride Yeşilimsi Vurgu Işığı (Yüzey detaylarını görmek için)
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb(0.6, 1.0, 0.6), // Hafif yeşil
            intensity: 5000.0,
            range: 80.0,
            shadows_enabled: false,             // Performans için gölgesi kapalı
            ..default()
        },
        transform: Transform::from_xyz(-20.0, 5.0, -15.0),
        ..default()
    });
}
