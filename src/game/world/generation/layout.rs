use bevy::prelude::*;

pub fn test_camera_zone_system(
    q_camera: Query<&Transform, With<Camera>>,
    layout_res: Option<Res<CaveLayout>>,
) {
    let Some(layout) = layout_res else { return; };
    let Ok(camera_transform) = q_camera.get_single() else { return; };
    
    let current_pos = camera_transform.translation;
    let zone = layout.get_zone_at(current_pos);
    
    match zone {
        ZoneType::Room(id) => {
            info!("[Zoning Test] Şu an {} Nolu Odanın içindesiniz!", id);
        }
        ZoneType::Corridor(from, to) => {
            info!("[Zoning Test] Şu an Oda {} ile Oda {} arasındaki Koridordesiniz!", from, to);
        }
        ZoneType::Wilderness => {}
    }
}


#[derive(Clone, Debug)]
pub struct EntranceData {
    pub start_pos: Vec3, // Yeraltındaki odanın merkezi
    pub end_pos: Vec3,   // Yeryüzüne (gökyüzüne) açılan nokta
    pub radius: f32,     // Deliğin genişliği
}

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub distance: f32,
}

#[derive(Clone, Debug)]
pub struct RoomData {
    pub id: usize,
    pub center: Vec3,
    pub radius: f32,
    pub shape: crate::game::world::generation::config::ShapeType,
    pub scale: Vec3,                                 
    pub height_variance: f32,
    pub floor_noise: f32,
}

#[derive(Clone, Debug)]
pub struct CorridorData {
    pub from_room_id: usize,
    pub to_room_id: usize,
    pub radius: f32,
    pub shape: crate::game::world::generation::config::ShapeType, 
    pub scale: Vec3,                                 
    pub noise_scale: f32,
    pub floor_noise: f32,
}

#[derive(Reflect, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ZoneType {
    Room(usize),
    Corridor(usize, usize),
    #[default]
    Wilderness,
}

// YENİ: Clone ve Debug eklendi
#[derive(Resource, Clone, Debug)]
pub struct CaveLayout {
    pub rooms: Vec<RoomData>,
    pub corridors: Vec<CorridorData>,
    pub entrances: Vec<EntranceData>, 
    pub seed: i32, 
}

impl CaveLayout {
    pub fn get_zone_at(&self, point: Vec3) -> ZoneType {
        for room in &self.rooms {
            if point.distance_squared(room.center) <= (room.radius * room.radius) {
                return ZoneType::Room(room.id);
            }
        }

        for corridor in &self.corridors {
            let p_a = self.rooms[corridor.from_room_id].center;
            let p_b = self.rooms[corridor.to_room_id].center;
            
            let pa = point - p_a;
            let ba = p_b - p_a;
            let h = (pa.dot(ba) / ba.dot(ba)).clamp(0.0, 1.0);
            let closest_point = p_a + ba * h;
            
            if point.distance_squared(closest_point) <= (corridor.radius * corridor.radius) {
                return ZoneType::Corridor(corridor.from_room_id, corridor.to_room_id);
            }
        }

        ZoneType::Wilderness
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        
        if root_i != root_j {
            self.parent[root_i] = root_j;
            true
        } else {
            false
        }
    }
}

pub fn calculate_mst(rooms: &[Vec3]) -> Vec<Edge> {
    let mut edges = Vec::new();
    let num_rooms = rooms.len();

    for i in 0..num_rooms {
        for j in (i + 1)..num_rooms {
            let dist = rooms[i].distance(rooms[j]);
            edges.push(Edge { from: i, to: j, distance: dist });
        }
    }

    edges.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    
    let mut mst = Vec::new();
    let mut uf = UnionFind::new(num_rooms);
    
    for edge in edges {
        if uf.union(edge.from, edge.to) {
            mst.push(edge);
            if mst.len() == num_rooms - 1 {
                break;
            }
        }
    }

    mst
}
