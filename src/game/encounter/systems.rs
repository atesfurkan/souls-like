use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::game::world::generation::region::RegionManager;
use super::components::ZoneEncounter;
use super::definition::EncounterConfig;
use super::generator::EncounterGenerator;

pub fn encounter_planner_system(
    mut commands: Commands,
    encounter_config: Res<EncounterConfig>,
    region_manager: Res<RegionManager>,
    mut planned_regions: Local<HashSet<IVec2>>,
) {
    for (&region_coord, layout) in &region_manager.layouts {
        // Eğer bu bölge zaten planlandıysa atla
        if planned_regions.contains(&region_coord) {
            continue;
        }

        let mut rng = StdRng::seed_from_u64(layout.seed as u64);

        for room in &layout.rooms {
            // Adım 1: Bu odanın fiziksel şartlarına (Yarıçap, Derinlik) uyan kuralları filtrele
            let valid_rules: Vec<_> = encounter_config.spawn_rules.iter()
                .filter(|rule| rule.is_valid(room.radius, room.center.y))
                .collect();

            // Adım 2: Ağırlıklı rastgele seçim algoritması (Weighted Random Selection)
            // Artık hardcoded bir fallback (örneğin "weak_monsters") yok. Hiçbir kural uymazsa None kalır.
            let mut selected_profile_id: Option<String> = None; 
            
            if !valid_rules.is_empty() {
                let total_weight: f32 = valid_rules.iter().map(|r| r.weight).sum();
                if total_weight > 0.0 {
                    let mut choice = rng.random_range(0.0..total_weight);
                    for rule in valid_rules {
                        choice -= rule.weight;
                        if choice <= 0.0 {
                            selected_profile_id = Some(rule.encounter_id.clone());
                            break;
                        }
                    }
                } else {
                    selected_profile_id = Some(valid_rules[0].encounter_id.clone());
                }
            }

            // Adım 3: Eğer geçerli bir profil seçildiyse jeneratörü çalıştır ve Entity'i oluştur
            if let Some(profile_id) = selected_profile_id {
                if let Some(definition) = encounter_config.definitions.get(&profile_id) {
                    let enemies = EncounterGenerator::generate_enemies(definition, &mut rng);
                    
                    commands.spawn(ZoneEncounter::new(
                        region_coord,
                        room.id,
                        profile_id,
                        definition.target_budget,
                        enemies,
                    ));
                }
            }
            // Kurala uymayan odalar sessizce es geçilir ve o odalarda düşman spawn olmaz.
        }
        
        // Bölgeyi planlandı olarak işaretle
        planned_regions.insert(region_coord);
    }
}

pub fn debug_encounter_state_system(
    query_encounters: Query<&ZoneEncounter, Changed<ZoneEncounter>>,
) {
    for encounter in &query_encounters {
        if encounter.pending_enemies.is_empty() {
            info!(
                "[ENCOUNTER RESOLVED] Region: {}, Oda {} ({}) için tüm düşmanlar yerleştirildi!", 
                encounter.region_coord, encounter.room_id, encounter.encounter_id
            );
        } else {
            info!(
                "[ENCOUNTER PLANNED] Region: {}, Oda {} -> Profil: {}, Bekleyen Düşman Sayısı: {}", 
                encounter.region_coord, encounter.room_id, encounter.encounter_id, encounter.pending_enemies.len()
            );
        }
    }
}
