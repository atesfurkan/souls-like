## Purpose

Modül, Bevy üzerinde varlık tabanlı (entity-based) yakın dövüş sistemini yönetir; saldırı niyetini belirler, durum makinesi (FSM) ile saldırı evrelerini (windup, active, recovery) yönetir ve fizik tabanlı hitbox/hurtbox etkileşimiyle hasar iletimini sağlar.

## Public API

### Public Components

* **`CombatPlugin`**: Modülün ana giriş noktası.
* **`Weapon`**: Saldırı tiplerini (`AttackType`) ve `AttackDefinition` verilerini tutan harita bileşeni.
* **`AttackIntent`**: Oyuncunun bir sonraki saldırı niyetini belirleyen bileşen.
* **`CombatState`**: Saldırı evrelerini yöneten durum makinesi (Idle, Windup, Active, Recovery).
* **`Hurtbox`**: Hasar alacak varlığı (owner) tanımlayan bileşen.
* **`Hitbox`**: Saldırı anında oluşturulan ve hasar/etkileşim verisi taşıyan bileşen.
* **`EquippedWeapon`**: Silah olarak işaretlenen entity'leri belirlemek için kullanılır.

### Public Events

* **`StaggerEvent`**: Varlık üzerinde sersemletme etkisi yaratır.

### Public Systems

* **`update_attack_intent`**: Fare hareketlerine göre `AttackType` günceller.
* **`process_attack_states`**: `CombatState` geçişlerini ve animasyon sürelerini işler.
* **`hit_detection`**: `CollisionEvent` verilerini dinleyerek hasarı ve sersemletmeyi tetikler.

### Public Plugin

* **`CombatPlugin`**: Tüm sistemleri register eder.

## Integration

1. **Plugin:** `app.add_plugins(CombatPlugin)` ile modülü register edin.
2. **Setup:** Saldıracak entity üzerine `CombatState::Idle`, `AttackIntent`, `Weapon` ve (görsel olarak) child entity olarak `EquippedWeapon` ekleyin.
3. **Hurtbox:** Hasar almasını istediğiniz entity üzerine `Hurtbox` bileşenini (owner olarak kendi entity ID'si ile) ekleyin.
4. **Events:** `DamageEvent` ve `StaggerEvent` dinleyicilerini kendi sağlık/sersemletme modüllerinizde işleyin.

### API Usage Examples

* **Weapon Definition:** `Weapon` oluştur -> Saldırı tiplerini `AttackDefinition` ile doldur -> `CombatState` ve `AttackIntent` ile entity'ye attach et.
* **Combat Flow:** `AttackIntent` güncelle -> `MouseClick` ile `CombatState`'i `Windup`'a al -> FSM (Sistemler) otomatik olarak `Active`'e geçer ve `Hitbox` spawn eder.

## Dependencies

* 
`bevy_rapier3d`: Fizik ve çarpışma algılama.


* 
`health` (module): `DamageEvent`.


* 
`stamina` (module): `Stamina` bileşeni.


* 
`lock_on` (module): `LockOnState`.



## Rules

* 
**State Control:** `CombatState` manuel değil, sistemler aracılığıyla güncellenmelidir.


* 
**Damage Pipeline:** Hasar doğrudan `Health` üzerinden değil, `DamageEvent` fırlatılarak uygulanmalıdır.


* 
**Self-Hit:** `Hitbox` ve `Hurtbox` aynı `owner`'a sahipse çarpışmalar yoksayılmalıdır.


* 
**Hit-Registration:** Bir `Hitbox`, aynı saldırı döngüsünde aynı `Hurtbox`'a sadece bir kez hasar vermelidir.



## Extension Points

* 
**Poise System:** `StaggerEvent` içerisinde `force` değeri `Poise` değeri ile karşılaştırılarak direnç eklenebilir.


* **Armor Ratings:** `DamageEvent` işlenirken `Hurtbox` sahibinin zırh değerleri hesaba katılabilir.
* 
**Weapon Types:** `AttackType` enum'ına yeni tipler eklenerek `animate_weapon` sistemine yeni keyframe'ler tanımlanabilir.
