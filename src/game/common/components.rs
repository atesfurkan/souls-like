use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct MainCamera {
    pub distance: f32,
    pub pitch: f32,
    pub yaw: f32,
}

// --- ORTAK HURTBOX ALTYAPISI ---
#[derive(Component)]
pub struct HurtboxProfile {
    pub proxies: &'static [&'static str],
}

#[derive(Component)]
pub struct BoneColliderMarker {
    pub bone_name: String,
    pub owner: Entity,
}

#[derive(Component)]
pub struct ProxyProcessed;

// Standart insansı karakterler (Player, Vanguard, Spearman vb.) için ortak proxy listesi
pub const HUMAN_PROXIES: &[&str] = &[
    "Head_Proxy",
    "Hips_Proxy",
    "LeftArm_Proxy",
    "LeftFoot_Proxy",
    "LeftForeArm_Proxy",
    "LeftHand_Proxy",
    "LeftLeg_Proxy",
    "LeftUpLeg_Proxy",
    "RightArm_Proxy",
    "RightFoot_Proxy",
    "RightForeArm_Proxy",
    "RightHand_Proxy",
    "RightLeg_Proxy",
    "RightUpLeg_Proxy",
    "Spine2_Proxy",
];