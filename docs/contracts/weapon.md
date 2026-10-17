---

### Purpose

Tüm silahların kimlik, istatistik (hasar, hız, hitbox) ve görsel (mesh, materyal) tanımlarını sağlayan, hiyerarşi kuralları sıkı belirlenmiş veri güdümlü (data-driven) konfigürasyon ve yaşam döngüsü modülü[cite: 6].

### Public API

**Public Components**

* `Weapon`: Ana veri bileşeni. UI/HUD sistemleri için kimlik bilgisini (`weapon_type: WeaponType`, `name: String`), savaş sistemleri için saldırı tanımlarını (`attacks`) ve görsel sistemler için parça verisini (`visuals`) içerir[cite: 6].
* `WeaponVisuals`: Silahı oluşturan parçaların hiyerarşik listesini tutar[cite: 6].
* `WeaponPart`: Silahın anatomisini tanımlayan enum (`Cylinder`, `Cone` vb.)[cite: 6].

**Public Events**

* Yok[cite: 6].

**Public Resources**

* Yok[cite: 6].

**Public Functions / Data**

* `WeaponType`: Sistemde kayıtlı olan silahların enum listesi (`Spear`, `GreatSword` vb.)[cite: 6].
* `get_weapon_data(weapon_type: WeaponType) -> Weapon`: İstenilen silahın statik verilerini ve görsel haritasını üreten factory fonksiyonu[cite: 6].

**Public Systems**

* `auto_spawn_weapon_visuals`: Root entity'deki `Weapon` bileşenini dinleyip, görselleri `EquippedWeapon` child entity'sine otomatik çizen sistem[cite: 6].

**Public Plugin**

* `WeaponPlugin`: Otomatik görsel oluşturma ve silah entegrasyon sistemlerini app'e kaydeder[cite: 6].

### Integration

* **Ana Kurulum:** Ana app başlatılırken `WeaponPlugin` register edilir[cite: 6].
* **Root Entity (Karakter):** Combat ve stat hesaplamaları için `Weapon` bileşeni doğrudan karakterin en üst entity'sine (Player/Enemy) eklenir[cite: 6].
* **Child Entity (Silah Tutucu):** Silahın fiziksel olarak konumlanacağı ve görsellerin ekleneceği child (alt) entity'ye `EquippedWeapon` (Combat modülünden) marker'ı eklenir[cite: 6].

### API Usage Examples

* **Silah Donanma (Equip):** Root entity'ye `Weapon` ekle → Plugin, Root'un çocuklarında `EquippedWeapon` arar → Görselleri bu child altına spawn eder[cite: 6].
* **HUD Okuma:** UI sistemi Root entity üzerindeki `Weapon` bileşenine query atar → `weapon.name` veya `weapon.weapon_type` alanını salt okunur (read-only) çeker[cite: 6].
* **Silah Bırakma (Unequip):** Root entity'den `Weapon` bileşeni `remove` edilir → `EquippedWeapon` entity'sinin altındaki tüm görsel child'lar (mesh'ler) `despawn_descendants` ile temizlenir[cite: 6].

### Dependencies

* `bevy::prelude`[cite: 6]
* `combat` modülü (`EquippedWeapon` marker bileşeni bağımlılığı)[cite: 6].

### Rules

* **Hiyerarşi Zorunluluğu:** `Weapon` bileşeni *kesinlikle* root (ana) entity üzerinde olmalıdır. `EquippedWeapon` marker'ı *kesinlikle* root'un bir alt (child) entity'sinde olmalıdır. Her iki bileşen aynı entity üzerinde bulunamaz[cite: 6].
* **Salt Okunur UI:** UI/HUD sistemleri `Weapon` bileşenindeki kimlik (`name`, `weapon_type`) alanlarını yalnızca okuyabilir, değiştiremez[cite: 6].
* **Temizleme (Cleanup) Kuralı:** Yeni bir silah takıldığında veya mevcut silah çıkarıldığında, eski görsel asset'lerin hafızada kalmaması için `EquippedWeapon` entity'sinin çocukları (eski mesh/materyaller) kesinlikle despawn edilmelidir. Otomatik spawn sistemi eski görselleri silmez[cite: 6].
* **Sanal Üretim:** `Weapon` struct'ı manuel oluşturulamaz; her zaman `get_weapon_data` kullanılmalıdır[cite: 6].
* **Hitbox Offset Referansı (KRİTİK):** Hitbox sistemi, doğrudan silahın IK ile hareket eden görsel hiyerarşisine (`EquippedWeapon`) bağlı olduğu için, `AttackDefinition` içindeki `hitbox_offset` değerleri **kesinlikle silahın yerel uzayına (local space)** göre tanımlanmalıdır. Silah modelleri (Örn: Silindir) varsayılan olarak **Y ekseni** boyunca uzandığından, hitbox'ı silahın ucuna kaydırmak için `Z` ekseni değil, **sadece Y ekseni** kullanılmalıdır (Örn: `Vec3::new(0.0, 1.2, 0.0)`). Yanlış eksen kullanımı, hitbox'ların silahın hizasından çıkarak alakasız noktalarda doğmasına neden olur.

### Extension Points

* Silah değiştirme işlemini otomatikleştirecek ve eski görselleri temizleyecek bir `WeaponChangedEvent` ve cleanup sistemi modüle eklenebilir[cite: 6].
* `WeaponPart` enum'ı `GltfModel` ile 3D asset desteği sağlayacak şekilde genişletilebilir[cite: 6].
* `Weapon` bileşenine dayanıklılık (durability) verisi eklenebilir[cite: 6].
