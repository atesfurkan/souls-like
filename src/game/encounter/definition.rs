use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::game::placement::components::EnemySpawnVariant;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct EnemyBudgetCost {
    pub variant: EnemySpawnVariant,
    pub cost: u32,
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct EncounterDefinition {
    pub id: String,
    pub target_budget: u32,
    pub allowed_enemies: Vec<EnemyBudgetCost>,
}

/// Bir odanın (Room) fiziksel özelliklerine göre hangi encounter'ın seçileceğini belirleyen data-driven kural.
#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct EncounterSpawnCondition {
    pub encounter_id: String,
    pub min_radius: Option<f32>,
    pub max_radius: Option<f32>,
    pub min_y: Option<f32>,
    pub max_y: Option<f32>,
    pub weight: f32, // Eğer birden fazla kural bir odaya uyuyorsa, hangisinin seçilme ihtimali daha yüksek?
}

impl EncounterSpawnCondition {
    /// Verilen odanın fiziksel özellikleri bu kural ile eşleşiyor mu kontrol eder.
    pub fn is_valid(&self, radius: f32, center_y: f32) -> bool {
        if let Some(min_r) = self.min_radius {
            if radius < min_r { return false; }
        }
        if let Some(max_r) = self.max_radius {
            if radius > max_r { return false; }
        }
        if let Some(min_y_val) = self.min_y {
            if center_y < min_y_val { return false; }
        }
        if let Some(max_y_val) = self.max_y {
            if center_y > max_y_val { return false; }
        }
        true
    }
}

#[derive(Resource, Clone, Debug)]
pub struct EncounterConfig {
    pub definitions: HashMap<String, EncounterDefinition>,
    pub spawn_rules: Vec<EncounterSpawnCondition>,
}

impl Default for EncounterConfig {
    fn default() -> Self {
        let mut definitions = HashMap::new();
        
        definitions.insert(
            "weak_monsters".to_string(),
            EncounterDefinition {
                id: "weak_monsters".to_string(),
                target_budget: 10,
                allowed_enemies: vec![
                    EnemyBudgetCost { variant: EnemySpawnVariant::UnarmedMonster, cost: 2 },
                    EnemyBudgetCost { variant: EnemySpawnVariant::Vanguard, cost: 5 },
                ],
            },
        );

        definitions.insert(
            "heavy_patrol".to_string(),
            EncounterDefinition {
                id: "heavy_patrol".to_string(),
                target_budget: 10,
                allowed_enemies: vec![
                    EnemyBudgetCost { variant: EnemySpawnVariant::Vanguard, cost: 5 },
                    EnemyBudgetCost { variant: EnemySpawnVariant::UnarmedMonster, cost: 10 },
                ],
            },
        );
        
        definitions.insert(
            "boss_chamber".to_string(),
            EncounterDefinition {
                id: "boss_chamber".to_string(),
                target_budget: 20,
                allowed_enemies: vec![
                    EnemyBudgetCost { variant: EnemySpawnVariant::HeavyDefender, cost: 20 },
                ],
            },
        );

        // ODA KURALLARI (Semantik Filtreleme)
        // Burada if-else yerine doğrudan veriyi tanımlıyoruz.
        let spawn_rules = vec![
            EncounterSpawnCondition {
                encounter_id: "boss_chamber".to_string(),
                min_radius: Some(12.0),
                max_radius: None,
                min_y: None,
                max_y: None,
                weight: 10.0, // Büyük odalarda kesin boss çıkması için yüksek ağırlık
            },
            EncounterSpawnCondition {
                encounter_id: "heavy_patrol".to_string(),
                min_radius: None,
                max_radius: None,
                min_y: None,
                max_y: None, // Y'si 5.0'dan küçük olan (derin) yerler için geçerli
                weight: 5.0,
            },
            EncounterSpawnCondition {
                encounter_id: "weak_monsters".to_string(),
                min_radius: None,
                max_radius: None,
                min_y: None,
                max_y: None,
                weight: 10.0, // Herhangi bir sınırlaması yok (Fallback kuralı)
            },
        ];

        Self { definitions, spawn_rules }
    }
}
