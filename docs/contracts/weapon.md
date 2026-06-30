
---

### Purpose

Tüm silahların kimlik, istatistik (hasar, hız, hitbox) ve görsel (mesh, materyal) tanımlarını sağlayan, hiyerarşi kuralları sıkı belirlenmiş veri güdümlü (data-driven) konfigürasyon ve yaşam döngüsü modülü.

### Public API

**Public Components**

* `Weapon`: Ana veri bileşeni. UI/HUD sistemleri için kimlik bilgisini (`weapon_type: WeaponType`, `name: String`), savaş sistemleri için saldırı tanımlarını (`attacks`) ve görsel sistemler için parça verisini (`visuals`) içerir.
* `WeaponVisuals`: Silahı oluşturan parçaların hiyerarşik listesini tutar.
* `WeaponPart`: Silahın anatomisini tanımlayan enum (`Cylinder`, `Cone` vb.).

**Public Events**

* Yok.

**Public Resources**

* Yok.

**Public Functions / Data**

* `WeaponType`: Sistemde kayıtlı olan silahların enum listesi (`Spear`, `GreatSword` vb.).
* `get_weapon_data(weapon_type: WeaponType) -> Weapon`: İstenilen silahın statik verilerini ve görsel haritasını üreten factory fonksiyonu.

**Public Systems**

* `auto_spawn_weapon_visuals`: Root entity'deki `Weapon` bileşenini dinleyip, görselleri `EquippedWeapon` child entity'sine otomatik çizen sistem.

**Public Plugin**

* `WeaponPlugin`: Otomatik görsel oluşturma ve silah entegrasyon sistemlerini app'e kaydeder.

### Integration

* **Ana Kurulum:** Ana app başlatılırken `WeaponPlugin` register edilir.
* **Root Entity (Karakter):** Combat ve stat hesaplamaları için `Weapon` bileşeni doğrudan karakterin en üst entity'sine (Player/Enemy) eklenir.
* **Child Entity (Silah Tutucu):** Silahın fiziksel olarak konumlanacağı ve görsellerin ekleneceği child (alt) entity'ye `EquippedWeapon` (Combat modülünden) marker'ı eklenir.

### API Usage Examples

* **Silah Donanma (Equip):** Root entity'ye `Weapon` ekle → Plugin, Root'un çocuklarında `EquippedWeapon` arar → Görselleri bu child altına spawn eder.
* **HUD Okuma:** UI sistemi Root entity üzerindeki `Weapon` bileşenine query atar → `weapon.name` veya `weapon.weapon_type` alanını salt okunur (read-only) çeker.
* **Silah Bırakma (Unequip):** Root entity'den `Weapon` bileşeni `remove` edilir → `EquippedWeapon` entity'sinin altındaki tüm görsel child'lar (mesh'ler) `despawn_descendants` ile temizlenir.

### Dependencies

* `bevy::prelude`
* `combat` modülü (`EquippedWeapon` marker bileşeni bağımlılığı).

### Rules

* **Hiyerarşi Zorunluluğu:** `Weapon` bileşeni *kesinlikle* root (ana) entity üzerinde olmalıdır. `EquippedWeapon` marker'ı *kesinlikle* root'un bir alt (child) entity'sinde olmalıdır. Her iki bileşen aynı entity üzerinde bulunamaz.
* **Salt Okunur UI:** UI/HUD sistemleri `Weapon` bileşenindeki kimlik (`name`, `weapon_type`) alanlarını yalnızca okuyabilir, değiştiremez.
* **Temizleme (Cleanup) Kuralı:** Yeni bir silah takıldığında veya mevcut silah çıkarıldığında, eski görsel asset'lerin hafızada kalmaması için `EquippedWeapon` entity'sinin çocukları (eski mesh/materyaller) kesinlikle despawn edilmelidir. Otomatik spawn sistemi eski görselleri silmez.
* **Sanal Üretim:** `Weapon` struct'ı manuel oluşturulamaz; her zaman `get_weapon_data` kullanılmalıdır.

### Extension Points

* Silah değiştirme işlemini otomatikleştirecek ve eski görselleri temizleyecek bir `WeaponChangedEvent` ve cleanup sistemi modüle eklenebilir.
* `WeaponPart` enum'ı `GltfModel` ile 3D asset desteği sağlayacak şekilde genişletilebilir.
* `Weapon` bileşenine dayanıklılık (durability) verisi eklenebilir.
