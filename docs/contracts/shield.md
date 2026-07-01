
## Purpose

Modül, oyuncu ve düşmanların gelen fiziksel saldırıları durum makinesi (FSM) ile savuşturmasını (parry) veya bloklamasını yönetir; kalkanların istatistiksel, fiziksel ve hiyerarşik 3D (Programmer Art) verilerini merkezi bir sağlayıcı üzerinden sunar.

## Public API

### Public Components

* **`Shield`**: Kalkanın ana istatistiklerini (block_rate, stamina_cost_multiplier, parry_window) tutan bileşen.
* **`ShieldBox`**: Kalkanın fiziksel çarpışmasını algılayan ve sahibini (owner) belirten işaretçi.
* **`EquippedShield`**: Kalkanın fiziksel/görsel kök (anchor) entity'sini işaretleyen marker.
* **`BlockingState`**: Savunma evrelerini yöneten durum makinesi (`Idle`, `Parrying`, `Blocking`, `GuardBroken`).

### Public Data & Factories (`game::shield::data`)

* **`ShieldType`**: Sistemde kayıtlı kalkanların enum listesi (`Buckler`, `KiteShield`, `TowerShield`).
* **`get_shield_stats_bundle`**: Root entity (Player/Enemy) için kalkan statlarını üretir.
* **İmza:** `fn get_shield_stats_bundle(shield_type: ShieldType) -> (Shield, BlockingState)`


* **`build_shield_hierarchy`**: Kalkanın ana anchor'ını, fiziksel collider'ını, görsel gövdesini, kol kayışını ve tutamacını otomatik spawn eder. `Commands` DEĞİL, doğrudan `ChildBuilder` kabul eder.
* **İmza:** `fn build_shield_hierarchy(builder: &mut ChildBuilder, shield_type: ShieldType, owner: Entity, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>)`
* **Parametreler:**
* `builder`: Ebeveyn kemik (örn. ön kol) üzerinde açılan `.with_children()` bloğundan gelen referans.
* `shield_type`: Üretilecek kalkan türü.
* `owner`: Kalkanın asıl sahibi olan Root Entity'nin (Player/Enemy) ID'si (Hasar/Recoil hesaplamaları için hayati önem taşır).
* `meshes` & `materials`: Bevy'nin görsel asset kaynakları.





### Public Events

* **`RecoilEvent`**: Başarılı bir savuşturma sonucunda saldırganı sektirmek için fırlatılır.
* **`BlockEvent`**: Bir saldırı engellendiğinde fırlatılır.

### Public Systems

* **`handle_shield_input`**: Sağ tık girdisine göre `BlockingState` geçişlerini yönetir.

## Integration & API Usage Examples

1. **Plugin Kaydı:** Uygulama başlatılırken `app.add_plugins(ShieldPlugin)` çağrılır.
2. **Kalkan Kuşanma Akışı (Örnek Kod Prensibi):**
* **Adım 1 (Root):** Ana karakter oluşturulurken statlar doğrudan root entity'ye eklenir.
```rust
let player_entity = commands.spawn((
    Player,
    get_shield_stats_bundle(ShieldType::KiteShield)
)).id();

```


* **Adım 2 (Child/Hierarchy):** Karakterin ön kol (forearm) kemiği oluşturulurken veya mevcut kemiğe erişilirken `with_children` açılarak görsel hiyerarşi inşa edilir.
```rust
commands.entity(forearm_entity).with_children(|builder| {
    build_shield_hierarchy(
        builder, 
        ShieldType::KiteShield, 
        player_entity, // Owner (Root ID)
        &mut meshes, 
        &mut materials
    );
});

```




3. **Savunma Başlatma:** Right Click Input -> `BlockingState` `Parrying` olur -> Timer biter -> `BlockingState` `Blocking` olur.
4. **Parry Başarısı:** `Hitbox`, `ShieldBox` ile çarpışır -> Owner (Root) `Parrying` durumundadır -> Hitbox despawn edilir -> Saldırgana `RecoilEvent` gönderilir.

## Dependencies

* `bevy_rapier3d`: Çarpışma kutuları ve sensorler.
* `combat` (module): `Hitbox` algılaması ve durum kesintisi (interrupt).
* `input`: `MouseButton` girdileri.

## Rules

* **Rapier İzolasyonu:** Dış modüller kalkan takarken asla manuel `Collider` oluşturmamalı veya `bevy_rapier3d` import etmemelidir.
* **Hiyerarşi Kuralı:** `build_shield_hierarchy` fonksiyonuna parametre olarak `Commands` GEÇİLEMEZ; kesinlikle bir entity'nin `ChildBuilder`'ı geçilmelidir.
* **Owner (Sahip) Kuralı:** `build_shield_hierarchy` fonksiyonuna geçilen `owner` parametresi, kemik entity'si DEĞİL, `Shield` statlarını taşıyan ana karakterin (Root Entity) ID'si olmak zorundadır.
* **Görsel İzolasyon:** Dış sistemler kalkanı sadece anchor noktasından ebeveyn kemiğe bağlar, manuel pozisyon kaydırması (offset) yapmaz. Kalkan parçaları kendi iç ofsetlerini otomatik ayarlar.
* **Hasar İptali:** Kalkan tarafından yakalanan bir `Hitbox` dünyadan despawn edilmelidir.
