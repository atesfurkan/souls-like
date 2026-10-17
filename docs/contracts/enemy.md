

### CONTRACT: ENEMY VARIANTS & COMPOSITION

**Son Güncelleme:** Temmuz 2026
**Durum:** Aktif (Veri Odaklı Mimariye Geçiş Aşamasında)

**[ 1. TEMEL PRENSİP: KOMPOZİSYON ]**

* Yeni düşman tipleri (varyantlar) oluşturulurken **kesinlikle** yeni bir sistem veya düşmana özel yeni bir bileşen (component) yazılamaz.
* Tüm varyantlar; halihazırda oyuncu ile ortak olan `Health`, `Poise`, `Weapon`, `Shield` ve `EnemyAI` bileşenlerinin farklı başlangıç değerleriyle bir araya getirilmesi (Composition) yoluyla oluşturulmak ZORUNDADIR.

**[ 2. OPSİYONEL EKİPMAN YÖNETİMİ ]**

* Tüm düşmanlar kalkan veya benzeri ikincil ekipmanlara sahip olmak zorunda değildir (Örn: `LightSpearman` varyantı).
* Bu nedenle `BlockingState`, `GuardIntent` gibi spesifik ekipman bileşenleri, ortak sistemlerde (özellikle `enemy_ai_system` içerisinde) **daima** `Option<&mut T>` kullanılarak güvenli bir şekilde sorgulanmalıdır.
* Sistemler, opsiyonel bir bileşen bulunmadığında (sorgu `None` döndüğünde) hata vermek (panic) yerine ana davranışını (hareket, temel saldırı) sergilemeye devam edecek şekilde yazılmalıdır. `Option<Mut<T>>` dönüşümleri her zaman `.as_deref_mut()` ile yapılmalıdır.

**[ 3. PARAMETRİK SPAWN YAPISI ]**

* `enemy/spawn.rs` içerisinde tek tip ve hardcoded (sabit) düşman yaratımı yasaktır.
* Düşmanlar, sahneye yerleştirilirken; Can (HP), Denge (Poise), Hareket Hızı, Algılama/Saldırı Menzili ve Ekipman Tipleri (`WeaponType`, `ShieldType`) dinamik parametreler olarak geçilerek spawn edilmelidir.

**[ 4. GELECEK VİZYONU: DATA-DRIVEN ARCHITECTURE ]**

* Mevcut kod içi `EnemyVariant` enum yapısı, veri odaklı (data-driven) mimariye geçiş için sadece geçici bir köprüdür.
* Gelecekte tüm düşman varyantları; 3D model yolları (örn. `models/heavy_knight.glb`), boyutları (scale) ve combat statları dahil olmak üzere `.ron` veya `.json` gibi harici veri dosyalarından (Asset) okunacaktır.
* Mimari bu değişikliğe tamamen hazır tutulacak, görsel ve mantıksal eşleştirmeler ana sistemlerin içine bir daha gömülmeyecektir.
