use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CaveType {
    #[default]
    Natural,
    Mine,
    LavaTube,
    Ruin,
}

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShapeType {
    #[default]
    Round,
    Boxy,
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct ActiveCaveType(pub CaveType);

#[derive(Reflect, Clone, Debug)]
pub struct RoomProfile {
    pub id: String,
    pub shape: ShapeType, 
    pub scale: Vec3,      
    pub min_radius: f32,
    pub max_radius: f32,
    pub min_height_variance: f32,
    pub max_height_variance: f32,
    pub floor_noise: f32,
}

#[derive(Reflect, Clone, Debug)]
pub struct CorridorProfile {
    pub id: String,
    pub shape: ShapeType, 
    pub scale: Vec3,      
    pub min_width: f32,
    pub max_width: f32,
    pub noise_scale: f32,
    pub floor_noise: f32,
}

#[derive(Reflect, Clone, Debug)]
pub struct EntranceProfile {
    pub id: String, 
    pub ramp_length: i32,
    pub radius: f32,
}

#[derive(Reflect, Clone, Debug)]
pub struct SpawnRule {
    pub profile_id: String, 
    pub weight: f32,        
}

#[derive(Reflect, Clone, Debug)]
pub struct CaveTypeConfig {
    pub grid_size: UVec3,
    // YENİ EKLENDİ: Mağara çıkma ihtimali / sıklığı (0.0 - 1.0)
    pub spawn_density: f32, 
    
    pub global_noise_scale: f32,
    pub min_depth: f32,
    pub max_depth: f32,
    
    pub min_rooms: usize,
    pub max_rooms: usize,
    pub entrance_count: usize,
    
    pub room_spawn_rules: Vec<SpawnRule>,
    pub corridor_spawn_rules: Vec<SpawnRule>,
    pub entrance_spawn_rules: Vec<SpawnRule>,

    pub wall_light_chance: f32,
    pub light_color: Color,
    pub light_intensity: f32,
    pub light_range: f32,
}

#[derive(Resource, Reflect, Clone, Debug)]
#[reflect(Resource)]
pub struct CaveGenerationConfig {
    pub registered_rooms: Vec<RoomProfile>,
    pub registered_corridors: Vec<CorridorProfile>,
    pub registered_entrances: Vec<EntranceProfile>,

    pub natural: CaveTypeConfig,
    pub mine: CaveTypeConfig,
    pub lava_tube: CaveTypeConfig,
    pub ruin: CaveTypeConfig,
}

impl Default for CaveGenerationConfig {
    fn default() -> Self {
        Self {
            registered_rooms: vec![
                RoomProfile {
                    id: "room_grotto".to_string(),
                    min_radius: 5.0, max_radius: 10.0,
                    shape: ShapeType::Boxy,
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    min_height_variance: -2.0, max_height_variance: 2.0,
                    floor_noise: 0.2,
                },
                RoomProfile {
                    id: "room_giant_cavern".to_string(),
                    shape: ShapeType::Boxy,
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    min_radius: 12.0, max_radius: 20.0,
                    min_height_variance: -5.0, max_height_variance: 5.0,
                    floor_noise: 0.4,
                },
                RoomProfile {
                    id: "room_mine_storage".to_string(),
                    shape: ShapeType::Boxy,
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    min_radius: 4.0, max_radius: 7.0,
                    min_height_variance: -0.5, max_height_variance: 0.5,
                    floor_noise: 0.05,
                }
            ],
            registered_corridors: vec![
                CorridorProfile {
                    id: "corridor_crevice".to_string(),
                    shape: ShapeType::Boxy,
                    scale: Vec3::new(1.0, 3.0, 0.5),
                    min_width: 1.5, max_width: 2.5,
                    noise_scale: 0.015, floor_noise: 0.3,
                },
                CorridorProfile {
                    id: "corridor_wide_tunnel".to_string(),
                    shape: ShapeType::Boxy,
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    min_width: 4.0, max_width: 6.0,
                    noise_scale: 0.05, floor_noise: 0.1,
                },
                CorridorProfile {
                    id: "corridor_mine_shaft".to_string(),
                    shape: ShapeType::Boxy,
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    min_width: 2.0, max_width: 3.0,
                    noise_scale: 0.02, floor_noise: 0.05,
                }
            ],
            registered_entrances: vec![
                EntranceProfile {
                    id: "entrance_gentle_ramp".to_string(),
                    ramp_length: 10, radius: 5.0,
                }
            ],

            natural: CaveTypeConfig {
                grid_size: UVec3::new(192, 64, 192),
                spawn_density: 1.0, // Her bölgede 1 tane çıkacak (1.0 / 1.0 = 1)
                global_noise_scale: 0.08, min_depth: 20.0, max_depth: 35.0,
                min_rooms: 20, max_rooms: 30, entrance_count: 1,
                light_color: Color::srgb(1.0, 0.6, 0.3), light_intensity: 15000.0,
                light_range: 1200.0, wall_light_chance: 0.0, 
                //statik ışık yapmayı beceremediğimiz için wall_light_chance: 0.0 yapıyom lan
                //ilerde bir daha uğraşabiliriz
                room_spawn_rules: vec![
                    SpawnRule { profile_id: "room_grotto".to_string(), weight: 70.0 },
                    SpawnRule { profile_id: "room_giant_cavern".to_string(), weight: 30.0 },
                ],
                corridor_spawn_rules: vec![
                    SpawnRule { profile_id: "corridor_crevice".to_string(), weight: 40.0 },
                    SpawnRule { profile_id: "corridor_wide_tunnel".to_string(), weight: 60.0 },
                ],
                entrance_spawn_rules: vec![
                    SpawnRule { profile_id: "entrance_gentle_ramp".to_string(), weight: 100.0 },
                ],
            },
            
            mine: CaveTypeConfig {
                grid_size: UVec3::new(48, 24, 48),
                spawn_density: 1.0, // Sizin hesabınız: 1.0 / 0.5 = 2. Her iki bölgeden birinde çıkacak!
                global_noise_scale: 0.04, min_depth: 15.0, max_depth: 25.0,
                min_rooms: 6, max_rooms: 12, entrance_count: 2,
                light_color: Color::srgb(0.9, 0.7, 0.2), light_intensity: 2000.0, light_range: 1000.0, wall_light_chance: 0.00,
                room_spawn_rules: vec![SpawnRule { profile_id: "room_mine_storage".to_string(), weight: 100.0 }],
                corridor_spawn_rules: vec![SpawnRule { profile_id: "corridor_mine_shaft".to_string(), weight: 100.0 }],
                entrance_spawn_rules: vec![],
            },
            
            lava_tube: CaveTypeConfig {
                grid_size: UVec3::new(80, 40, 80), spawn_density: 1.0, 
                global_noise_scale: 0.1, min_depth: 40.0, max_depth: 60.0,
                min_rooms: 3, max_rooms: 5, entrance_count: 1,
                light_color: Color::srgb(1.0, 0.2, 0.0), light_intensity: 3000.0, light_range: 2000.0, wall_light_chance: 0.00,
                room_spawn_rules: vec![], corridor_spawn_rules: vec![], entrance_spawn_rules: vec![],
            },
            ruin: CaveTypeConfig {
                grid_size: UVec3::new(50, 20, 50), spawn_density: 0.3, 
                global_noise_scale: 0.02, min_depth: 10.0, max_depth: 15.0,
                min_rooms: 3, max_rooms: 6, entrance_count: 1,
                light_color: Color::srgb(0.2, 0.6, 1.0), light_intensity: 2500.0, light_range: 1500.0, wall_light_chance: 0.00,
                room_spawn_rules: vec![], corridor_spawn_rules: vec![], entrance_spawn_rules: vec![],
            },
        }
    }
}
