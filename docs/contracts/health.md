## Health Module Technical Contract

### Purpose

Entities'in can değerlerini yönetmek, hasar alma mekanizmalarını event-driven bir yapıda işlemek ve sıfır can durumunda entity yaşam döngüsünü sonlandırmakla görevli modüldür.

### Public API

* 
**Public Components:** * `Health`: Entity'nin güncel ve maksimum can değerlerini tutan temel veri yapısı.


* 
**Public Events:** * `DamageEvent`: Hedef entity'e belirtilen miktarda hasar uygulamak için kullanılır.


* **Public Resources:** Yok.
* 
**Public Systems:** * `process_damage`: `DamageEvent`'leri işleyerek ilgili `Health` bileşenini günceller.


* 
`check_death`: `Health` bileşeni $\leq 0$ olan entity'leri `despawn_recursive()` ile dünyadan kaldırır.




* 
**Public Plugin:** * `HealthPlugin`: Modülün sistemlerini ve event kayıtlarını Bevy `App`'ine dahil eder.



---

### Integration

* 
**Plugin Registration:** Uygulama başlatılırken `HealthPlugin` register edilmelidir.


* 
**Component Initialization:** Hasar alması istenen tüm entity'lere `Health` bileşeni eklenmelidir.


* 
**Damage Flow:** Hasar uygulamak için `EventWriter<DamageEvent>` aracılığıyla hedef entity ve hasar miktarı gönderilmelidir.



### API Usage Examples

1. 
**Entity Setup:** Spawn Entity → `commands.entity(...).insert(Health::new(max_health))`.


2. 
**Damage Application:** `event_writer.send(DamageEvent { target: entity, amount: val })`.


3. 
**Automatic Lifecycle:** Hasar `Health.current` değerini günceller → `process_damage` sonrası `check_death` çalışır → Entity yok olur.



### Dependencies

* 
`bevy::prelude::*`.



---

### Rules

* 
**Encapsulation:** `Health` bileşeninin `current` değeri doğrudan değiştirilmemeli; hasar işlemleri yalnızca `DamageEvent` üzerinden yapılmalıdır.


* 
**Ordering:** `process_damage` ve `check_death` sistemleri Bevy içinde `.chain()` ile bağlıdır; sistem sırası bozulmamalıdır.


* 
**Dependency:** `HealthPlugin` register edilmeden `DamageEvent` işlenemez ve `check_death` sistemi tetiklenmez.



### Extension Points

* **Healing:** `HealEvent` veya `Health` bileşenini güncelleyen bir artış mekanizması eklenebilir.
* **Death Handling:** Ölüm anında sadece despawn yerine özel bir `DeathEvent` fırlatılarak oyun mantığına (efektler, skor vb.) entegrasyon sağlanabilir.
* **Resilience:** Zırh veya hasar azaltma çarpanları `process_damage` içerisine eklenebilir.
