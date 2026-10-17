use rand::Rng;
use crate::game::placement::components::EnemySpawnVariant;
use super::definition::EncounterDefinition;

pub struct EncounterGenerator;

impl EncounterGenerator {
    /// Tanımlanmış havuzdan ve bütçeden düşman kompozisyonunu üretir.
    pub fn generate_enemies(
        definition: &EncounterDefinition,
        rng: &mut impl Rng,
    ) -> Vec<EnemySpawnVariant> {
        let mut selected = Vec::new();
        let mut current_budget = 0;
        let target = definition.target_budget;

        if definition.allowed_enemies.is_empty() || target == 0 {
            return selected;
        }

        let mut available = definition.allowed_enemies.clone();

        while current_budget < target {
            // Kalan bütçeye sığabilecek düşmanları filtrele
            let remaining = target - current_budget;
            available.retain(|cost_entry| cost_entry.cost <= remaining);

            if available.is_empty() {
                break;
            }

            let index = rng.random_range(0..available.len());
            let pick = &available[index];

            selected.push(pick.variant.clone());
            current_budget += pick.cost;
        }

        selected
    }
}
