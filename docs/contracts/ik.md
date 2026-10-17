Aşağıda, hazırladığımız `game::ik` modülünün diğer sistemler (Combat, Shield vb.) tarafından nasıl kullanılacağını ve iç yapısını açıklayan API referans belgesi bulunmaktadır.

---

## Modül: `game::ik`

`game::ik` modülü, Bevy 0.14 için özel olarak yazılmış, analitik (Trigonometri tabanlı) bir **Two-Bone Inverse Kinematics (2-Bone IK)** çözümüdür. Karakterlerin kollarının veya bacaklarının, oynatılan animasyon üzerine (post-process) belirli bir hedefe (hedef nokta) ulaşmasını sağlar.

### Eklenti (Plugin)

#### `IkPlugin`

IK hesaplamalarını oyun döngüsüne (ECS) dahil eder. Sistemlerin doğru fazda çalışmasını garanti eder.

* **Çalışma Fazı:** `PostUpdate`
* **Sıralama:** Bevy'nin animasyon sistemi kemikleri güncelledikten sonra çalışır; `TransformSystem::TransformPropagate` (çocuk kemiklerin dünya pozisyonuna aktarımı) aşamasından hemen **önce** devreye girer. Bu sıralama *1 frame gecikme (jitter)* sorununu çözer.

---

### Bileşenler (Components)

#### `IkTarget`

Bir uzvun (genellikle el veya ayak) dünyada ulaşması istenen konumu ve animasyonlar arası harmanlama (blending) oranını belirler. Gameplay (oyun mantığı) modülleri tarafından yönetilir.

| Alan | Tür | Açıklama |
| --- | --- | --- |
| `position` | `Vec3` | Elin/Ayağın ulaşması istenen uzaysal hedef pozisyonu (Dünya veya Lokal koordinat). |
| `weight` | `f32` | IK'nin etkin olma oranını belirler. **`0.0`** tamamen standart animasyonu (Forward Kinematics), **`1.0`** tamamen IK pozunu kullanır. `0.0` ile `1.0` arasındaki değerler `slerp` ile yumuşak geçiş (blending) sağlar. |

#### `TwoBoneIkChain`

IK'nin uygulanacağı kemik zincirini ve bükülme eksenini (Pole Vector) tanımlayan yapısal bileşendir. Karakter oluşturulurken (spawn) veya silah kuşanılırken eklenir.

| Alan | Tür | Açıklama |
| --- | --- | --- |
| `root` | `Entity` | Zincirin başlangıç kemiği. Örneğin: Omuz (`LeftArm`). |
| `mid` | `Entity` | Zincirin bükülme noktasındaki kemik. Örneğin: Dirsek (`LeftForeArm`). |
| `tip` | `Entity` | Ulaşılması istenen uç kemik. Örneğin: El (`LeftHand`). |
| `pole_vector` | `Vec3` | Dirseğin veya dizin ne tarafa doğru büküleceğini belirten dünya/lokal yön vektörü. |

---

### Sistemler (Systems)

#### `solve_two_bone_ik`

Tüm `TwoBoneIkChain` ve `IkTarget` bileşenlerini okuyan ve açı hesaplamalarını yapan ana çözücüdür.

* **Çalışma Mantığı:**
1. Hedefin `weight` değeri `0.0` ise işlemi atlar (performans tasarrufu).
2. `root`, `mid` ve `tip` için kemik uzunluklarını ( $L_{1}$ ve $L_{2}$ ) ölçer.
3. Hedefe olan toplam mesafe ( $D$ ) ölçülür. Kopmayı önlemek için uzanma mesafesi $L_{1} + L_{2}$ toplamı ile sınırlandırılır (Clamp).
4. Omuz açısını bulmak için **Kosinüs Teoremi** (Law of Cosines) uygulanır:
$$ \theta = \arccos\left(\frac{L_{1}^2 + D^2 - L_{2}^2}{2 \cdot L_{1} \cdot D}\right) $$
5. Hesaplanan dönüşler önce Quaternion olarak belirlenir. Ardından ebeveyn kemiklerin (*parent*) gerçek dünya rotasyonları ile ters çevrilerek (Inverse) kemiklerin lokal `Transform` bileşenlerine geri yazılır.



---

### Gameplay İçin Örnek Kullanım (Entegrasyon)

Gameplay modülleriniz (`Shield`, `Combat` veya `Weapon`), kemik rotasyonlarına asla doğrudan müdahale etmemelidir. Bunun yerine sadece bir `IkTarget` oluşturmalı veya güncellemelidir.

**Örnek: Kalkan Modülü Tarafından IK Hedefi Belirleme**

```rust
use bevy::prelude::*;
use crate::game::ik::components::{IkTarget, TwoBoneIkChain};
use crate::game::animation::components::CharacterBones;

/// Karakter kalkanı kaldırdığında veya indirdiğinde çalışacak örnek sistem
pub fn update_shield_guard_intent(
    mut commands: Commands,
    // Karakterin kemik referanslarını okuyoruz
    query: Query<(Entity, &CharacterBones, &ShieldState)>, 
) {
    for (entity, bones, shield_state) in query.iter() {
        // Gerekli kemikler Animation modülü tarafından önbelleğe (cache) alınmış mı?
        if !bones.is_linked { continue; } 

        let left_arm = bones.left_arm.unwrap();
        let left_forearm = bones.left_forearm.unwrap();
        let left_hand = bones.left_hand.unwrap();

        // Kalkan aktifse IK weight'i 1.0, değilse 0.0 yap
        let target_weight = if shield_state.is_guarding { 1.0 } else { 0.0 };

        // Hedefi karakter entity'sine ekle/güncelle
        commands.entity(entity).insert((
            TwoBoneIkChain {
                root: left_arm,
                mid: left_forearm,
                tip: left_hand,
                pole_vector: Vec3::new(-1.0, -1.0, -1.0), // Dirsek dışarı/aşağı bükülsün
            },
            IkTarget {
                position: Vec3::new(0.3, 1.2, 0.5), // Kalkanın göğüs hizasındaki konumu
                weight: target_weight, // İleride bu değer bir sistem tarafından 'Lerp' edilebilir
            }
        ));
    }
}

```
