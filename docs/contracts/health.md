
## Health Module Technical Contract

### Purpose

Entities'in can değerlerini yönetmek, hasar alma ve iyileşme mekanizmalarını event-driven (olay güdümlü) bir yapıda işlemek ve sıfır can durumunda entity yaşam döngüsünü sonlandırarak ilgili `DeathEvent`'i fırlatmakla görevli modüldür.

### Public API

* **Public Components:**
* `Health`: Entity'nin güncel ve maksimum can değerlerini tutan temel veri yapısı.


* **Public Events:**
* `DamageEvent`: Hedef entity'e belirtilen miktarda hasar uygulamak için kullanılır.
* `HealEvent`: Hedef entity'nin canını belirtilen miktarda artırmak için kullanılır (maksimum can sınırı gözetilir).
* `DeathEvent`: Bir entity'nin canı sıfırlandığında fırlatılır. Oyunun diğer bölümleri (skor, ses, efekt vb.) bu olayı dinleyerek kendi tepkilerini verebilir.


* **Public Systems:**
* `process_damage`: `DamageEvent`'leri işleyerek ilgili `Health` bileşenini günceller.
* `process_heal`: `HealEvent`'leri işleyerek can değerini artırır ve aşırı iyileşmeyi (overheal) engeller.
* `check_death`: `Health` bileşeni $\leq 0$ olan entity'ler için `DeathEvent` fırlatır ve ardından entity'yi `despawn_recursive()` ile dünyadan kaldırır.


* **Public Plugin:**
* `HealthPlugin`: Modülün sistemlerini ve event kayıtlarını Bevy `App`'ine dahil eder.



---

### Integration

* **Plugin Registration:** Uygulama başlatılırken `HealthPlugin` register edilmelidir.
* **Component Initialization:** Hasar alması veya iyileşmesi istenen tüm entity'lere `Health` bileşeni eklenmelidir.
* **Damage Flow:** Hasar uygulamak için `EventWriter<DamageEvent>` aracılığıyla hedef entity ve hasar miktarı gönderilmelidir.
* **Healing Flow:** İyileşme uygulamak için `EventWriter<HealEvent>` aracılığıyla hedef entity ve iyileşme miktarı gönderilmelidir.

---

### API Usage Examples

1. **Entity Setup:** `commands.entity(...).insert(Health::new(max_health))`.
2. **Damage Application:** `event_writer.send(DamageEvent { target: entity, amount: val })`.
3. **Healing Application:** `event_writer.send(HealEvent { target: entity, amount: val })`.
4. **Lifecycle:** Hasar/İyileşme güncellenir → `process_...` sistemleri çalışır → `check_death` tetiklenirse `DeathEvent` fırlatılır ve entity yok olur.

### Dependencies

* `bevy::prelude::*`.

---

### Rules

* **Encapsulation:** `Health` bileşeninin `current` değeri doğrudan değiştirilmemeli; işlemler yalnızca `Event`'ler üzerinden yapılmalıdır.
* **Ordering:** `process_damage`, `process_heal` ve `check_death` sistemleri Bevy içinde `.chain()` ile bağlıdır; sistem sırası bozulmamalıdır.
* **Dependency:** `HealthPlugin` register edilmeden `Event`'ler işlenemez ve `check_death` sistemi tetiklenmez.

### Extension Points

* **Resilience:** Zırh veya hasar azaltma çarpanları `process_damage` içerisine veya hasar hesaplamasından önce çalışacak bir ara sisteme eklenebilir.

---

Bu dokümantasyon artık modülünüzün güncel halini eksiksiz yansıtıyor. Başka bir modül üzerinde çalışmamı ister misin, yoksa bu modül üzerinde yapmak istediğin başka bir ince ayar var mı?


