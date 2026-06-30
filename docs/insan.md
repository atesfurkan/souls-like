# Proje Çalışma Kuralları

Bu klasördeki dokümanlar AI hafızasının yerine geçer.

Amaç:
- Her AI sohbetinde tüm kod tabanını göndermemek.
- Modüllerin çalışma kurallarını standartlaştırmak.
- AI'ın mevcut mimariyi bozmasını engellemek.

---

## AI Sohbeti Açarken

1. Önce ilgili contract dosyalarını gönder.

Örnek:

- combat.md
- health.md
- weapon.md

2. Daha sonra sadece değişecek modüllerin kaynak kodlarını gönder.

3. AI'dan önce mevcut mimariyi analiz etmesini iste.

4. Çalışan sistemleri yeniden yazmamasını özellikle belirt.

5. Sprint sonunda:
- Manual Test Checklist
- Regression Checklist
- Edge Cases
istemeyi unutma.

---

## Contract Dosyaları

combat.md
Combat sistemi

health.md
Health sistemi

weapon.md
Weapon sistemi

player.md
Player sistemi

enemy.md
Enemy sistemi

...

---

1. Önce ilgili contract dosyalarını gönder.

2. Daha sonra yalnızca değiştirilmesi beklenen modülü gönder.

3. AI başka dosya isterse önce gerekçesini açıklamalıdır.

4. Gerekmeyen hiçbir kaynak kod gönderilmez.

5. Contract dosyaları referans kabul edilir.


## Kural

Contract dosyaları her sprintte güncellenmez.

Sadece modülün dış API'si değiştiğinde güncellenir.
