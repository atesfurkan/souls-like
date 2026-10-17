benim_oyun/
├── Cargo.toml
└── src/
    ├── main.rs                 # Laboratuvar giriş noktası, Bevy App ve Plugin kayıtları
    ├── lib.rs                  # Başka projelere taşınacak ana kütüphane export dosyası
    ├── camera/                 # Debug ve inceleme kameraları
    │   ├── mod.rs
    │   └── freecam.rs          # Mağara içinde serbestçe uçup geometriyi incelemek için
    ├── config/                 # Parametrik yapılandırma resourceları
    │   ├── mod.rs
    │   └── cave_config.rs      # Oda sayıları, tünel genişlikleri, gürültü frekansları
    ├── generation/             # Matematik ve algoritma çekirdeği (Bevy Render'dan bağımsız)
    │   ├── mod.rs
    │   ├── noise.rs            # Gürültü haritaları ve yoğunluk (density) hesaplamaları
    │   ├── sdf.rs              # Signed Distance Fields (Organik şekiller için temel)
    │   ├── layout.rs           # Oda ve koridor grafıkları (Delaunay / MST)
    │   └── surface.rs          # Yoğunluk verisini yüzeye çeviren algoritmalar (Marching Cubes vb.)
    ├── mesh/                   # Geometrik veriyi Bevy Mesh formatına çevirme
    │   ├── mod.rs
    │   ├── builder.rs          # Vertex, Index, Normal, UV ve Tangent hesaplayıcıları
    │   └── material.rs         # Mağara test materyalleri ve shader hazırlıkları
    ├── physics/                # Bevy Rapier3D entegrasyonyu
    │   ├── mod.rs
    │   └── collider.rs         # Üretilen Mesh'ten TriMesh veya Convex decomposition üretimi
    ├── sockets/                # Meta-veri ve bağlantı noktaları
    │   ├── mod.rs
    │   ├── detector.rs         # Düz zemin, tavan ve duvar tespiti (matematiksel)
    │   └── types.rs            # Spawn, Boss, Trap, Loot soket tanımları
    └── debug/                  # Görselleştirme araçları
        ├── mod.rs
        ├── gizmos.rs           # Oda sınırları, bağlantı çizgileri, normal vektör çizgileri
        └── inspector.rs        # Adım adım üretim süreci görselleştiricisi

**klasör yapısı burada lan!  **
