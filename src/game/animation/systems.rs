
use bevy::prelude::*;
use super::components::{
    ActiveAnimation, AnimationIntent, AnimationNodes, CharacterAnimations, CharacterBones,
};

pub const BONE_RIGHT_ARM: &str = "mixamorig:RightArm";
pub const BONE_RIGHT_FOREARM: &str = "mixamorig:RightForeArm";
pub const BONE_RIGHT_HAND: &str = "mixamorig:RightHand";

pub const BONE_LEFT_ARM: &str = "mixamorig:LeftArm";
pub const BONE_LEFT_FOREARM: &str = "mixamorig:LeftForeArm";
pub const BONE_LEFT_HAND: &str = "mixamorig:LeftHand";

pub fn update_animations_from_intent(
    mut anim_query: Query<(
        Entity,
        &AnimationNodes,
        &mut ActiveAnimation,
        &mut AnimationPlayer,
    )>,
    parent_query: Query<&Parent>,
    intent_query: Query<&AnimationIntent>,
) {
    for (entity, nodes, mut active_anim, mut player) in anim_query.iter_mut() {
        let mut current_entity = entity;
        let mut found_intent = None;

        while let Ok(parent) = parent_query.get(current_entity) {
            let parent_entity = parent.get();
            if let Ok(intent) = intent_query.get(parent_entity) {
                found_intent = Some(intent);
                break;
            }
            current_entity = parent_entity;
        }

        let Some(intent) = found_intent else {
            continue;
        };

        let target_node = if intent.is_in_air {
            nodes.jump
        } else if intent.is_moving {
            if intent.is_sprinting {
                nodes.run
            } else {
                nodes.walk
            }
        } else {
            nodes.idle
        };

        if active_anim.0 == target_node && player.is_playing_animation(target_node) {
            continue;
        }

        player.stop_all();

        if target_node == nodes.jump {
            player.play(target_node);
        } else {
            player.play(target_node).repeat();
        }

        active_anim.0 = target_node;
    }
}

pub fn load_animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    let animations = CharacterAnimations {
        idle: asset_server.load("models/animations/idle.glb#Animation0"),
        walk: asset_server.load("models/animations/walk_sword_shield.glb#Animation0"),
        run: asset_server.load("models/animations/run_sword_shield.glb#Animation0"),
        jump: asset_server.load("models/animations/jump_sword_shield.glb#Animation0"),
    };

    commands.insert_resource(animations);
    info!("Karakter animasyonları belleğe yüklendi.");
}

pub fn setup_animation_graph(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    animations: Res<CharacterAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    for (entity, mut player) in &mut players {
        let mut graph = AnimationGraph::new();

        let idle_node = graph.add_clip(animations.idle.clone(), 1.0, graph.root);
        let walk_node = graph.add_clip(animations.walk.clone(), 1.0, graph.root);
        let run_node = graph.add_clip(animations.run.clone(), 1.0, graph.root);
        let jump_node = graph.add_clip(animations.jump.clone(), 1.0, graph.root);

        let graph_handle = graphs.add(graph);
        commands.entity(entity).insert(graph_handle);

        commands.entity(entity).insert(AnimationNodes {
            idle: idle_node,
            walk: walk_node,
            run: run_node,
            jump: jump_node,
        });

        commands.entity(entity).insert(ActiveAnimation(idle_node));

        player.play(idle_node).repeat();
        info!("AnimationGraph tüm kliplerle kuruldu ve Idle başlatıldı.");
    }
}

pub fn cache_character_bones(
    mut query: Query<(Entity, &mut CharacterBones)>,
    children_query: Query<&Children>,
    names_query: Query<(Entity, &Name)>,
) {
    for (entity, mut bones) in query.iter_mut() {
        if bones.is_linked {
            continue;
        }

        let mut found_count = 0;
        let target_count = 6;

        for descendant in children_query.iter_descendants(entity) {
            if let Ok((bone_entity, name)) = names_query.get(descendant) {
                let name_str = name.as_str();
                match name_str {
                    BONE_RIGHT_ARM => {
                        bones.right_arm = Some(bone_entity);
                        found_count += 1;
                    }
                    BONE_RIGHT_FOREARM => {
                        bones.right_forearm = Some(bone_entity);
                        found_count += 1;
                    }
                    BONE_RIGHT_HAND => {
                        bones.right_hand = Some(bone_entity);
                        found_count += 1;
                    }
                    BONE_LEFT_ARM => {
                        bones.left_arm = Some(bone_entity);
                        found_count += 1;
                    }
                    BONE_LEFT_FOREARM => {
                        bones.left_forearm = Some(bone_entity);
                        found_count += 1;
                    }
                    BONE_LEFT_HAND => {
                        bones.left_hand = Some(bone_entity);
                        found_count += 1;
                    }
                    _ => {}
                }
            }
        }

        if found_count >= target_count {
            bones.is_linked = true;
            info!("Tüm kalkan ve silah IK kemikleri başarıyla tarandı!");
        }
    }
}
