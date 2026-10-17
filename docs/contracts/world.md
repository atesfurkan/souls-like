# World Module Architectural Contract

Bu doküman, `world` ve `procedural generation` modüllerinin mimari sınırlarını, tasarım prensiplerini ve katmanlar arası iletişim kurallarını tanımlar. Gelecekte bu modülde yapılacak tüm geliştirmeler, eklenecek yeni algoritmalar ve sistemler bu sözleşmedeki (contract) kurallara kesin olarak uymak zorundadır.

---

## 1. Modülün Amacı
World modülü, oyun dünyasının topografyasını, bağlantılarını, ekosistemlerini ve içerik yerleşimlerini prosedürel olarak üretmekten, doğrulamaktan ve yönetmekten sorumludur. Nihai amaç; deterministik, genişletilebilir, performansı yüksek ve tamamen veriye dayalı (data-driven) bir dünya üretim hattı (pipeline) sunmaktır.

---

## 2. Genel Mimari Prensipler

* **Separation of Concerns (Sorumlulukların Ayrılığı):** Karar alma (logic) ve görselleştirme (rendering) kesinlikle ayrılmalıdır. Jeneratörler ağaç veya su mesh'i üretmez; yalnızca ağacın veya suyun nerede olması gerektiğini matematiksel olarak tanımlar.
* **Data-Driven Design (Veri Odaklı Tasarım):** Oyun içi kurallar, ağırlıklar, boyutlar ve kısıtlamalar koda gömülmemelidir (hardcoding yasaktır). Her şey konfigürasyon veri yapıları üzerinden yönetilmelidir.
* **Stateless Analysis (Durumsuz Analiz):** Analiz katmanları durum (state) tutmaz. Ham veriyi okur ve anlık türetilmiş matematiksel sonuçlar döndürür.
* **Fail-Fast Validation (Hızlı Hata Tespiti):** Kısıtlama (constraint) kontrolleri en ucuz ve hızlı olanından (örneğin O(1) doluluk kontrolü) en pahalı olanına (A* veya Raycasting) doğru sıralanarak çalıştırılmalıdır.

---

## 3. Katmanlar ve Modül Sorumlulukları

World mimarisi kesin sınırlarla ayrılmış ardışık katmanlardan oluşur:

### 3.1. Foundation (Temel Uzay ve Topoloji) Katmanları
Dünyanın fiziksel ve topolojik gerçekliğini temsil eder. Diğer tüm sistemler bu veriye güvenir.
* **Spatial Foundation:** Dünya üzerindeki koordinatların yürünebilirliğini, yüksekliğini ve doluluk (occupied) durumunu tutar. Sonsuz büyüklükteki alanları desteklemek üzere seyrek (sparse) yapıda tasarlanmıştır.
* **Navigation Graph:** Hücreler arasındaki topolojik bağlantıları (edge) ve erişilebilirlik kümelerini (connected components) tutar. Geçitler veya engeller üzerinden haritanın parçalanıp parçalanmadığı bu katmandan sorgulanır.

### 3.2. Analysis (Analiz ve Anlamlandırma) Katmanları
Foundation katmanını okuyarak yüksek seviyeli çevresel anlamlar çıkartır.
* **Terrain Analysis:** Yüzey normallerini, zemin eğimini, çukur/tepe karakteristiklerini ve yoğunluğu (density) hesaplar.
* **Biome System:** İklimsel verilere (sıcaklık, nem) ve topografyaya bakarak dünyanın ekosistem haritasını çıkartır.

### 3.3. Generator (Üretim) Katmanları
Belirli bir konsepti (Orman, Yol, Nehir, Mağara) tasarlayan mantık birimleridir. Jeneratörler Foundation'ı değiştirme yetkisine sahiptir, ancak kendi aralarındaki çakışmaları Placement ve Validation sistemleri aracılığıyla çözmek zorundadırlar.

### 3.4. Placement ve Validation (Yerleşim ve Doğrulama) Sistemi
Mimari kilit taşıdır. Hiçbir jeneratör, Placement sistemini atlayarak (bypass) dünyaya içerik yerleştiremez. 
* Jeneratörler yalnızca "Aday Koordinatlar" ve "Geçerlilik Kuralları" listesi (Placement Request) sunar.
* Placement sistemi, bu talebi mekânsal, topografik ve navigasyonel filtrelerden geçirerek onaylar veya reddeder.

---

## 4. Modüller Arası Haberleşme ve Veri Akışı

* **Salt Okunur (Read-Only) Güvenlik:** Analiz servisleri ve Placement sistemi, Foundation katmanını yalnızca okuma izniyle referans alır. Veriyi mutasyona uğratamazlar.
* **Ağ (Network) Kayıtları:** Jeneratörler (Road, River, Cave) haritada yaptıkları değişiklikleri kendi özel ağ veri kaynaklarına (Network Resources) kaydederler. Örneğin, bir yol çizildiğinde bu yol hem `SpatialMap` üzerinde yürünebilir olarak işaretlenir hem de `RoadNetwork` içerisine semantik bir koridor olarak kaydedilir.
* **Sorgu Akışı:** Diğer jeneratörler kararlarını alırken bu ağ kaynaklarını sorgular (Örn: Orman jeneratörü, "Bu koordinat `RoadNetwork` koridorunda mı?" diye sorar).

---

## 5. Çok Katmanlı Uzay ve Yeraltı Kuralları (Z-Offsetting)

Sistemin esnekliğini korumak ve 2D Grid tabanlı algoritmaları (A*, BFS) karmaşık 3D voxel algoritmalarına dönüştürmemek için **Katman Kaydırma (Spatial Offsetting)** kuralı geçerlidir.
* Yeraltı (Mağaralar, Zindanlar) veya çok katlı yapılar aynı 2D düzlem üzerinde matematiksel olarak çok uzak bir eksene (Örn: Z + 100.000) öteleme yapılarak inşa edilir.
* Katmanlar arası geçişler (mağara girişleri, merdivenler) Navigation Graph üzerinde iki uzak koordinatı birbirine bağlayan **Portallar (Bidirectional Edges)** olarak modellenmelidir.
* Bu kural sayesinde yüzeydeki bir düşman yapay zekası veya yerleşim kuralı, yeraltında da hiçbir kod değişikliği gerektirmeden çalışmaya devam eder.

---

## 6. Yeni Generator Ekleme Kuralları

Yeni bir jeneratör (Örn: Şehir Üreteci, Görev Dağıtıcı) ekleneceğinde aşağıdaki kurallara kesin olarak uyulmalıdır:

1.  **Görsel Temsil Yasaktır:** Jeneratör mesh spawnlamamalıdır. Yalnızca semantik veri (Blueprint, Network, Coordinate List) üretmelidir.
2.  **Placement Kuralları Genişletilmelidir:** Yeni obje tipi yeni bir kontrol gerektiriyorsa, bu kural Jeneratörün içine yazılmaz; Ortak Placement kural setine (`PlacementRule`) yeni bir seçenek olarak eklenir.
3.  **Foundation Saygısı:** Jeneratör, eğer üzerine yazdığı yeri yürünemez hale getiriyorsa veya bir engelle kapatıyorsa, bunu `SpatialMap` üzerinde anında işaretlemeli ve `NavigationGraph`'ın bileşenlerini yeniden hesaplatmalıdır (Recalculate Components). Haritayı onarılamaz şekilde bölen (Unreachable Zone) hamleler Validation katmanından geçemez.

---

## 7. Dosya Organizasyonu

Dünya mimarisi mantıksal ağaca sadık kalarak şu şekilde organize edilir:
* `data/` -> Veri yapıları, konfigürasyon modelleri ve içerik tanımları.
* `spatial/` -> Yürünebilirlik, yükseklik ve temel ızgara sistemleri.
* `nav_graph/` -> Topolojik düğümler, erişilebilirlik ve portallar.
* `terrain_analysis/` -> Eğim, normal ve karakter hesaplama servisleri.
* `biome/` -> İklim simülasyonu ve ekosistem atamaları.
* `placement/` -> Kural setleri ve doğrulama motoru.
* `generators/` -> Özelleşmiş alt üretim modülleri (forest, road, river, cave vb.).

---

## 8. Kod Yazma Kuralları (Checklist)

Gelecekte bu modülde kod yazacak her yapay zeka ve geliştirici aşağıdaki kontrol listesine uymakla yükümlüdür:
* [ ] Kod içerisinde `TODO`, `FIXME`, `unimplemented!()` veya `panic!()` bırakılmamalıdır.
* [ ] Yeni veri yapıları `.ron` veya diğer Asset formlarından yüklenebilecek şekilde tasarlanmalı, ilgili framework (Örn: Bevy Reflect) ile motor ekosistemine tanıtılmalıdır.
* [ ] Tüm sistemler, modülerlik adına parametrelerini hardcoded sayılar yerine bir Yapılandırma Objesi (Config Struct) üzerinden almalıdır.
* [ ] Hata fırlatmak yerine sistemin varsayılan, güvenli bir "Fallback" rotası (Örn: Varsayılan biyom, boş jeneratör listesi) olmalıdır.
