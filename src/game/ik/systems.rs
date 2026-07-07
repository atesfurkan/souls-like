use super::components::{IkTarget, TwoBoneIkChain};
use crate::game::combat::components::GuardIntent;
use crate::game::lock_on::components::LockOnState;
use crate::game::player::components::Player;
use crate::game::shield::components::BlockingState;
use crate::game::shield::components::EquippedShield;
use bevy::prelude::*;
use crate::game::shield::components::ShieldVisualAnchor;

// Bu fonksiyonu game/ik/systems.rs dosyasının en altına ekle
pub fn maintain_shield_orientation(
    player_query: Query<(Entity, &IkTarget), With<Player>>,
    // Sadece yeni oluşturduğumuz Anchor'ı arıyoruz
    anchor_query: Query<(Entity, &Parent), With<ShieldVisualAnchor>>, 
    mut transforms: Query<&mut Transform>,
    parents: Query<&Parent>,
) {
    let Ok((player_entity, ik_target)) = player_query.get_single() else { return; };
    if ik_target.weight <= 0.01 { return; }

    let player_global = compute_manual_global_transform(player_entity, &transforms, &parents);

    for (anchor_entity, anchor_parent) in anchor_query.iter() {
        // Kalkanın bağlı olduğu ön kolun (parent) global rotasyonu
        let root_global = compute_manual_global_transform(anchor_parent.get(), &transforms, &parents);
        
        // Kalkanın orjinal duruş ofseti (shield/data.rs içindeki değer)
        let base_shield_rot = Quat::from_euler(EulerRot::XYZ, 0.0, 3.0, 3.0);
        
        // MUCİZE BURADA: Kalkanın dünyada bakacağı yön = Karakterin baktığı yön + Kalkanın kendi ofseti
        let desired_global_rot = player_global.rotation * base_shield_rot;

        // Bunu Local Uzaya (Ebeveynin uzayına) çevirerek kolun bükülmesini SIFIRLIYORUZ
        let target_local_rot = root_global.rotation.inverse() * desired_global_rot;

        if let Ok(mut anchor_transform) = transforms.get_mut(anchor_entity) {
            anchor_transform.rotation = anchor_transform.rotation.slerp(target_local_rot, ik_target.weight);
        }
    }
}
pub fn update_shield_ik_target(
    // Sorguya 'mut TwoBoneIkChain' eklendi!
    mut query: Query<
        (
            &mut IkTarget,
            &mut TwoBoneIkChain,
            &GuardIntent,
            &BlockingState,
            &Transform,
        ),
        With<Player>,
    >,
    lock_on: Res<LockOnState>,
    time: Res<Time>,
) {
    for (mut ik_target, mut chain, intent, state, transform) in query.iter_mut() {
        let is_guarding = lock_on.target.is_some() && matches!(state, BlockingState::Blocking);

        let target_w = if is_guarding { 1.0 } else { 0.0 };
        ik_target.weight = ik_target.weight.lerp(target_w, time.delta_seconds() * 15.0);

        if ik_target.weight > 0.01 {
            // 1. Kalkan Hedefi (Elin gideceği nokta)
            
            // DÜZELTİLDİ: Eli gövdeye çok daha yakın ve hafif sola aldık.
            // Z: -0.5'ten -0.2'ye çektik. El göğse yaklaşınca IK mecburen dirseği bükecek (L şekli).
            // X: -0.15 ile eli tam ortada değil, hafif sol omuza yakın tuttuk (kalkan bloklaması için daha doğal).
            let local_base_pos = Vec3::new(-0.15, 0.3, -0.2); 
            
            let offset_x = intent.offset.x * 0.8; 
            let offset_y = -intent.offset.y * 0.8; 
            let offset_z = offset_x.abs() * 0.3; 

            let local_target = local_base_pos + Vec3::new(offset_x, offset_y, offset_z);
            ik_target.position = transform.transform_point(local_target);
            
            // 2. ÇÖZÜM: Pole Vector (Dirseğin bakacağı nokta)
            // L şeklinin düzgün durması için dirseği TAM SOLA (-X) itiyoruz.
            let local_pole = Vec3::new(-1.0, 0.0, 0.0);
            chain.pole_vector = local_pole.normalize_or_zero();
        }

    }
}

// Hiyerarşiyi tırmanarak 1-frame gecikmesiz anlık Global Transform hesaplayan yardımcı fonksiyon
fn compute_manual_global_transform(
    entity: Entity,
    transforms: &Query<&mut Transform>,
    parents: &Query<&Parent>,
) -> Transform {
    let mut current_entity = entity;
    let mut global_transform = Transform::IDENTITY;
    let mut hierarchy = Vec::new();

    while let Ok(transform) = transforms.get(current_entity) {
        hierarchy.push(*transform);
        if let Ok(parent) = parents.get(current_entity) {
            current_entity = parent.get();
        } else {
            break;
        }
    }

    for transform in hierarchy.into_iter().rev() {
        global_transform = global_transform * transform;
    }
    global_transform
}

pub fn solve_two_bone_ik(
    chain_query: Query<(&TwoBoneIkChain, &IkTarget)>,
    mut transforms: Query<&mut Transform>,
    parent_query: Query<&Parent>,
) {
    for (chain, target) in chain_query.iter() {
        if target.weight <= 0.0 {
            continue;
        }

        // 1. KEMİK UZUNLUKLARINI LOKAL UZAYDAN ÇEKME
        // Dirsek omuzun çocuğu, el dirseğin çocuğu olduğu için lokal translation uzunlukları doğrudan kemik boyudur.
        let len1 = transforms
            .get(chain.mid)
            .map(|t| t.translation.length())
            .unwrap_or(0.001);
        let len2 = transforms
            .get(chain.tip)
            .map(|t| t.translation.length())
            .unwrap_or(0.001);
        let total_len = len1 + len2;

        // 2. OMUZUN ANLIK DÜNYA UZAYI (Jitter-Free)
        let root_parent = parent_query
            .get(chain.root)
            .map(|p| p.get())
            .unwrap_or(chain.root);
        let root_parent_global =
            compute_manual_global_transform(root_parent, &transforms, &parent_query);

        let root_local = transforms.get(chain.root).cloned().unwrap_or_default();
        let root_global_pos = (root_parent_global * root_local).translation;

        // 3. HEDEFE YÖNELİK SOFT IK HESAPLAMASI
        let mut dir_target = target.position - root_global_pos;
        let mut dist_target = dir_target.length();
        let dir_target_norm = dir_target.normalize_or_zero();

        let d_max = total_len * 0.95;

        if dist_target > d_max {
            // Soft IK Formülü uygulaması
            let diff = dist_target - d_max;
            let max_extend = total_len - d_max;
            dist_target = d_max + max_extend * (1.0 - (-diff / max_extend).exp());
        } else {
            dist_target = dist_target.max(0.001); // 0'a bölme hatasını engelle
        }

        // 4. KUTUPSAL VEKTÖR (POLE VECTOR) DÖNÜŞÜMÜ
        // Pole Vector'u karakterin (root_parent) dönüş yönüne göre hizala
        let global_pole_vector = root_parent_global.rotation * chain.pole_vector;
        let mut bend_normal = dir_target_norm
            .cross(global_pole_vector)
            .normalize_or_zero();

        if bend_normal == Vec3::ZERO {
            bend_normal = dir_target_norm.cross(Vec3::Y).normalize_or_zero();
        }

        // 5. İÇ AÇI (KOSİNÜS TEOREMİ) VE YENİ POZİSYONLAR
        let cos_angle_root = ((len1 * len1) + (dist_target * dist_target) - (len2 * len2))
            / (2.0 * len1 * dist_target);
        let angle_root = cos_angle_root.clamp(-1.0, 1.0).acos();

        let new_mid_dir = Quat::from_axis_angle(bend_normal, angle_root) * dir_target_norm;

        let root_global_rot = (root_parent_global * root_local).rotation;

        // Varsayılan kolların uzanma eksenini (Forward) belirliyoruz. Mixamo riglerinde genellikle Y veya -Y eksenidir.
        let default_bone_dir = transforms
            .get(chain.mid)
            .map(|t| t.translation.normalize_or_zero())
            .unwrap_or(Vec3::Y);
        let current_mid_global_dir = root_global_rot * default_bone_dir;

        // 6. OMUZ VE DİRSEK LOKAL ROTASYON YAZIMI
        let root_rot_diff = Quat::from_rotation_arc(current_mid_global_dir, new_mid_dir);
        let target_root_global_rot = root_rot_diff * root_global_rot;

        // Global rotasyonu Root'un ebeveynine göre Lokal'e çeviriyoruz
        let target_root_local_rot = root_parent_global.rotation.inverse() * target_root_global_rot;

        if let Ok(mut root_transform) = transforms.get_mut(chain.root) {
            root_transform.rotation = root_transform
                .rotation
                .slerp(target_root_local_rot, target.weight);
        }

        // Dirsek (Mid) dönüşü lokal eksende bükülme açısına eşittir.
        // Omuz zaten doğru yöne döndüğü için, dirseği sadece 'bend_normal' etrafında bükmek yeterlidir.
        let cos_angle_mid =
            ((len1 * len1) + (len2 * len2) - (dist_target * dist_target)) / (2.0 * len1 * len2);
        let angle_mid = cos_angle_mid.clamp(-1.0, 1.0).acos();

        // Düz kol açısı PI'dir, bükülme miktarı PI - angle_mid
        let bend_amount = std::f32::consts::PI - angle_mid;
        let mid_local_rot = Quat::from_axis_angle(Vec3::X, bend_amount); // Mixamo'da diz/dirsek bükülme ekseni X veya Z olabilir, rig'e göre test edilmeli.
        if let Ok(mut mid_transform) = transforms.get_mut(chain.mid) {
            // Quat::IDENTITY yerine kemiğin o anki animasyon rotasyonunu al!
            let base_rot = mid_transform.rotation;
            mid_transform.rotation = base_rot.slerp(mid_local_rot, target.weight);
        }
    }
}
