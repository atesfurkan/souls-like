### Purpose

Tüm silahların istatistiksel verilerini (hasar, hız, hitbox) ve görsel tanımlarını (mesh, materyal) tek bir merkezden sağlayan, veri güdümlü (data-driven) bir silah konfigürasyon ve otomatik görsel üretim (spawning) modülüdür.

### Public API

**Public Components**

* `Weapon`: Silahın tüm saldırı tanımlarını (`attacks`) ve görsel blueprint verisini (`visuals`) tutan ana bileşen.
* `WeaponVisuals`: Silahı oluşturan parçaların hiyerarşik listesini tutar.
* `WeaponPart`: Silahın görsel anatomisini tanımlayan enum (`Cylinder`, `Cone` vb.).

**Public Events**

* Yok.

**Public Resources**

* Yok.

**Public Functions / Data**

* `WeaponType`: Sistemde kayıtlı olan tüm silahların listesini tutan enum (`Spear`, `GreatSword` vb.).
* `get_weapon_data(weapon_type: WeaponType) -> Weapon`: İstenilen silahın statik verilerini ve görsel haritasını üreten factory fonksiyonu.

**Public Systems**

* `auto_spawn_weapon_visuals`: Üzerinde `Weapon` bileşeni olan entity'leri dinleyip görsel parçalarını otomatik oluşturan sistem.

**Public Plugin**

* `WeaponPlugin`: Modülün otomatik görsel oluşturma ve gelecekteki silah yaşam döngüsü sistemlerini Bevy app'ine kaydeder.

### Integration

* Ana app başlatılırken `WeaponPlugin` register edilir.
* Silah takılacak bir karakter (Player/Enemy) spawn edilirken, karakterin kök dizinine (root) veya bir silah tutucu (holder) child entity'sine `get_weapon_data()` çağrılarak üretilen `Weapon` component attach edilir.
* Görsel sistemin doğru child entity üzerine silahı çizebilmesi için, hedeflenen tutucu entity'ye `EquippedWeapon` (Combat modülünden) marker component'i attach edilmelidir.

### API Usage Examples

* Register `WeaponPlugin` → Görsel (Visual) auto-spawning sistemi aktif olur.
* Call `get_weapon_data(WeaponType::Spear)` → `Weapon` verisi elde edilir.
* Attach `Weapon` to entity → Plugin, `visuals` verisini okur ve mesh child entity'leri otomatik spawn eder.
* Combat System reads `Weapon.attacks` → Aktif saldırıya ait hitbox yarıçapı, hasarı ve offset verisi elde edilir.

### Dependencies

* `bevy::prelude` (Transform, Mesh, Color, SpatialBundle vb. temel render/fizik araçları).
* `combat` modülü (`EquippedWeapon` marker bileşeni, görselin nereye spawn edileceğini belirlemek için kullanılır).

### Rules

* `Weapon` struct'ı ve içerisindeki `HashMap` doğrudan manuel olarak oluşturulmaz; her zaman `get_weapon_data` factory fonksiyonu kullanılmalıdır.
* Silahların 3D/Mesh görselleri `spawn.rs` veya başka modüller içerisinde manuel olarak spawn edilmez; `Weapon` bileşeni eklenerek modülün otomatik spawn etmesi beklenir.
* Combat modülü `Weapon` içerisindeki veriyi (hasar, süre, hitbox radius) yalnızca okumalı (read-only), bu değerler üzerinden mutasyon yapmamalıdır.
* Görsel spawn sisteminin bir döngüye girmemesi için modülün kendi içinde kullandığı `WeaponChildrenCreated` marker'ı dışarıdan manuel olarak silinmemeli veya eklenmemelidir.

### Extension Points

* `WeaponPart` enum'ı genişletilerek `GltfModel { path: String }` ile 3D asset desteği eklenebilir.
* `Weapon` bileşenine `durability` (dayanıklılık) veya `weight` (ağırlık) statları eklenebilir.
* Silah geliştirmeleri (Upgrade) için taban statları modifiye eden bir ara katman/sistem yazılabilir.
* Farklı hasar tipleri (Elemental, Slash, Pierce) `AttackDefinition` içerisine eklenebilir.
