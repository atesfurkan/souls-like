
use bevy::prelude::*;
use bevy::animation::graph::AnimationNodeIndex; // DOĞRU İMPORT

#[derive(Component, Debug, Default, Reflect)]
pub struct AnimationIntent {
    pub is_moving: bool,     // Karakter hareket etmek istiyor mu?
    pub is_sprinting: bool,  // Karakter koşmak istiyor mu?
    pub is_in_air: bool,
    pub is_blocking: bool,
    pub is_attacking: bool,
    pub look_at_target: Option<Vec3>,
}

/// Karakterin AnimationGraph içindeki düğüm (node) numaralarını tutar.
#[derive(Component)]
pub struct AnimationNodes {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub jump: AnimationNodeIndex, // YENİ: Zıplama düğümü
}

/// O an hangi animasyonun çaldığını takip etmek için (aynı animasyonu her karede baştan başlatmamak için)
#[derive(Component)]
pub struct ActiveAnimation(pub AnimationNodeIndex);


/// Oyundaki tüm karakter animasyonlarının referanslarını tutar
#[derive(Resource)]
pub struct CharacterAnimations {
    pub idle: Handle<AnimationClip>,
    pub walk: Handle<AnimationClip>,
    pub run: Handle<AnimationClip>,
    pub jump: Handle<AnimationClip>,
}

// CharacterBones struct'ını şu şekilde güncelle:
#[derive(Component, Debug, Reflect)]
pub struct CharacterBones {
    pub is_linked: bool,
    pub root: Option<Entity>,
    pub hips: Option<Entity>,
    pub spine: Option<Entity>,
    pub head: Option<Entity>,
    // --- SOL KOL ZİNCİRİ (Kalkan ve IK İçin) ---
    pub left_arm: Option<Entity>,     
    pub left_forearm: Option<Entity>, 
    pub left_hand: Option<Entity>,    
    // --- SAĞ KOL ZİNCİRİ (Silah ve IK İçin) ---
    pub right_arm: Option<Entity>,       // YENİ EKLENDİ
    pub right_forearm: Option<Entity>,   // YENİ EKLENDİ
    pub right_hand: Option<Entity>,   
    // -------------------------------------------
    pub left_foot: Option<Entity>,
    pub right_foot: Option<Entity>,
}

impl Default for CharacterBones {
    fn default() -> Self {
        Self {
            is_linked: false,
            root: None,
            hips: None,
            spine: None,
            head: None,
            left_arm: None,
            left_forearm: None,
            left_hand: None,
            right_arm: None,       // YENİ
            right_forearm: None,   // YENİ
            right_hand: None,
            left_foot: None,
            right_foot: None,
        }
    }
}
