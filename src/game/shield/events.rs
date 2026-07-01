use bevy::prelude::*;

// Kılıç kalkana vurduğunda saldıran kişiyi sektirmek için fırlatılır
#[derive(Event)]
pub struct RecoilEvent {
    pub target: Entity, // Sekecek olan kişi (Saldırgan)
}

// Başarılı bir blok/parry yapıldığında ses, vfx ve stamina düşüşü için fırlatılır
#[derive(Event)]
pub struct BlockEvent {
    pub defender: Entity,
    pub attacker: Entity,
    pub is_parry: bool, // Eğer gerçek bir parry ise stamina gitmemesi gibi özel etkiler için
}
