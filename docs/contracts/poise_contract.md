## Purpose
Modül, varlıkların (entity) duruş (poise) ve sarsılma direncini yönetir. Gelen poise hasarlarını işler, değeri sıfırlandığında sistemin geri kalanına haber vererek Stagger veya Guard Break gibi durumların tetiklenmesine olanak tanır.

## Public API

### Public Components
* **`Poise`**: Varlığın anlık ve maksimum poise değerini tutan bileşen (current, max).

### Public Events
* **`ApplyPoiseDamageEvent`**: Bir varlığa poise hasarı uygulanmasını talep eden event (target, amount).
* **`PoiseBrokenEvent`**: Bir varlığın poise değeri sıfırlandığında fırlatılan event. Diğer sistemler (Stagger, Animation vb.) bu eventi dinler.

### Public Systems
* **`handle_poise_damage`**: `ApplyPoiseDamageEvent`'leri dinleyip ilgili entity'nin `Poise` component'inden düşer. Değer sıfırlanırsa `PoiseBrokenEvent` fırlatır ve poise'ı (şimdilik) yeniler.

### Public Plugin
* **`PoisePlugin`**: Poise eventlerini ve sistemlerini register eder.

## Integration

1. **Plugin:** `app.add_plugins(PoisePlugin)` ile modülü ana uygulamaya kaydedin.
2. **Setup:** Duruş direnci olmasını istediğiniz varlıklara (oyuncu, düşman) `Poise { current: 100.0, max: 100.0 }` bileşenini ekleyin.
3. **Hasar İletimi:** Combat/Hit Detection sistemlerinizde çarpışma gerçekleştiğinde `ApplyPoiseDamageEvent` fırlatın.
4. **Tepki:** Stagger veya Guard Break sistemlerinde `PoiseBrokenEvent`'i dinleyerek gerekli durum değişikliklerini (state transition) uygulayın.

## Dependencies
* Yok (Tamamen decoupled). Diğer sistemler eventler üzerinden haberleşir.

## Rules
* **Decoupled from Health:** Poise hasarı ve normal can hasarı (Health) birbirini etkilememelidir. İkisi aynı anda alınabilir ancak ayrı hesaplanır.
* **Single Source of Truth:** Poise eksilmesi sadece `handle_poise_damage` sistemi üzerinden, eventler ile yapılmalıdır. Entity üzerindeki değere doğrudan müdahale edilmemelidir.
* **Charged Attack Handling:** `weapon/data.rs` içindeki değerler **Base (Taban)** poise hasarıdır. Charged attack gibi dinamik çarpanlar `combat` modülünde (Hitbox oluşturulurken) hesaplanmalı ve `ApplyPoiseDamageEvent`'e **net değer** olarak gönderilmelidir. Poise modülü charge veya saldırı mantığını bilmemelidir.

## Extension Points
* **Poise Regeneration:** Poise hasarı almadıktan X saniye sonra `current` değerini `max` değere yavaşça veya birden dolduran bir yenilenme sistemi eklenebilir.
* **Hyperarmor:** Bazı ağır saldırıların (örn. Overhead attack) `Active` evresinde oyuncuya sonsuz veya ekstra Poise veren "Hyperarmor" modifier'ları entegre edilebilir.
* **Stagger Systems:** Bir sonraki sprintte `PoiseBrokenEvent` dinlenerek Entity'nin durumu `CombatState::Staggered` (veya benzeri) bir yapıya zorlanacaktır.
