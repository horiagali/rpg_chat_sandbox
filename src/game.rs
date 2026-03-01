use macroquad::prelude::*;
use macroquad_tiled_clone::{IrObject, Map as TiledMap};
use std::collections::{HashMap, HashSet};

use crate::actor::npc::{NpcAttitude, NpcData, NpcRole};
use crate::actor::player::{PlayerData, WorldPosition};

pub const MAP_PIXEL_WIDTH: i32 = 960;
pub const MAP_PIXEL_HEIGHT: i32 = 640;

pub async fn run() {
    let mut game = Game::new().await;

    loop {
        game.frame().await;
    }
}

struct Game {
    tiled_map: TiledMap,
    world_truth: WorldTruthIndex,
    player: PlayerData,
    blacksmiths: Vec<NpcData>,
    player_texture: Texture2D,
    blacksmith_texture: Texture2D,
}

impl Game {
    async fn new() -> Self {
        let tiled_map = TiledMap::load("assets/map.json")
            .await
            .expect("Failed to load map");
        let world_truth = WorldTruthIndex::from_map(&tiled_map);
        let player_position = find_spawn_position(&tiled_map, "player_start")
            .expect("Could not find spawn marker with spawn_id=`player_start`");
        let blacksmith_positions = find_spawn_positions(&tiled_map, "blacksmith");
        assert!(
            !blacksmith_positions.is_empty(),
            "Could not find spawn marker(s) with spawn_id=`blacksmith`"
        );
        let player = PlayerData::new("player_01", "Player", player_position);
        let blacksmiths: Vec<NpcData> = blacksmith_positions
            .into_iter()
            .enumerate()
            .map(|(index, position)| {
                NpcData::new(
                    format!("blacksmith_{:02}", index + 1),
                    "Blacksmith",
                    NpcRole::Merchant,
                    NpcAttitude::Friendly,
                    position,
                )
            })
            .collect();
        let player_texture = load_texture("assets/heeheemisphere.png")
            .await
            .expect("Failed to load player texture");
        let blacksmith_texture = load_texture("assets/big_yahu.png")
            .await
            .expect("Failed to load blacksmith texture");

        player_texture.set_filter(FilterMode::Nearest);
        blacksmith_texture.set_filter(FilterMode::Nearest);

        println!(
            "Loaded map truth index: {} entities, {} spawn points, {} colliders",
            world_truth.entity_handles.len(),
            world_truth.spawn_handles.len(),
            world_truth.collider_count()
        );

        Self {
            tiled_map,
            world_truth,
            player,
            blacksmiths,
            player_texture,
            blacksmith_texture,
        }
    }

    async fn frame(&mut self) {
        clear_background(WHITE);
        let view_min = vec2(0.0, 0.0);
        let view_max = vec2(MAP_PIXEL_WIDTH as f32, MAP_PIXEL_HEIGHT as f32);
        self.tiled_map.draw(view_min, view_max);

        draw_actor_sprite(&self.player_texture, self.player.position, vec2(56.0, 56.0));
        draw_text(
            "Player",
            self.player.position.x + 10.0,
            self.player.position.y + 5.0,
            18.0,
            RED,
        );

        for (index, blacksmith) in self.blacksmiths.iter().enumerate() {
            draw_actor_sprite(&self.blacksmith_texture, blacksmith.position, vec2(56.0, 56.0));
            draw_text(
                &format!("Blacksmith {}", index + 1),
                blacksmith.position.x + 10.0,
                blacksmith.position.y + 5.0,
                18.0,
                BLUE,
            );
        }

        draw_text(
            &format!("entities: {}", self.world_truth.entity_handles.len()),
            12.0,
            24.0,
            24.0,
            BLACK,
        );

        next_frame().await;
    }

    fn object_from_handle(&self, handle: ObjectHandle) -> Option<&IrObject> {
        self.tiled_map
            .object_layers()
            .get(handle.layer_index)
            .and_then(|layer| layer.objects.get(handle.object_index))
    }
}

#[derive(Clone, Copy)]
struct ObjectHandle {
    layer_index: usize,
    object_index: usize,
}

struct WorldTruthIndex {
    // Immutable map-derived indices.
    entity_handles: HashMap<String, ObjectHandle>,
    spawn_handles: HashMap<String, ObjectHandle>,
    collider_handles: Vec<ObjectHandle>,

    // Mutable runtime overlays keyed by stable map IDs.
    entity_position_overrides: HashMap<String, Vec2>,
    disabled_entities: HashSet<String>,
}

impl WorldTruthIndex {
    fn from_map(map: &TiledMap) -> Self {
        let mut entity_handles = HashMap::new();
        let mut spawn_handles = HashMap::new();
        let mut collider_handles = Vec::new();
        let mut class_name_counts: HashMap<String, u32> = HashMap::new();

        for (layer_index, layer) in map.object_layers().iter().enumerate() {
            for (object_index, obj) in layer.objects.iter().enumerate() {
                let handle = ObjectHandle {
                    layer_index,
                    object_index,
                };

                if let Some(entity_id) = obj.properties.get_string("entity_id") {
                    entity_handles.insert(entity_id.to_string(), handle);
                } else if !obj.class_name.trim().is_empty() {
                    let normalized_class = normalize_entity_key(&obj.class_name);
                    let counter = class_name_counts
                        .entry(normalized_class.clone())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    let generated_id = format!("{}_{}", normalized_class, counter);
                    entity_handles.insert(generated_id, handle);
                }

                if let Some(spawn_id) = obj.properties.get_string("spawn_id") {
                    spawn_handles.insert(spawn_id.to_string(), handle);
                }

                if obj.properties.get_bool("collider").unwrap_or(false) {
                    collider_handles.push(handle);
                }
            }
        }

        Self {
            entity_handles,
            spawn_handles,
            collider_handles,
            entity_position_overrides: HashMap::new(),
            disabled_entities: HashSet::new(),
        }
    }

    fn get_spawn<'a>(&self, game: &'a Game, spawn_id: &str) -> Option<&'a IrObject> {
        let handle = self.spawn_handles.get(spawn_id)?;
        game.object_from_handle(*handle)
    }

    fn get_entity<'a>(&self, game: &'a Game, entity_id: &str) -> Option<&'a IrObject> {
        let handle = self.entity_handles.get(entity_id)?;
        game.object_from_handle(*handle)
    }

    fn entity_world_pos(&self, game: &Game, entity_id: &str) -> Option<Vec2> {
        if self.disabled_entities.contains(entity_id) {
            return None;
        }

        if let Some(pos) = self.entity_position_overrides.get(entity_id) {
            return Some(*pos);
        }

        let obj = self.get_entity(game, entity_id)?;
        Some(vec2(obj.x, obj.y))
    }

    fn set_entity_world_pos(&mut self, entity_id: impl Into<String>, new_pos: Vec2) {
        self.entity_position_overrides
            .insert(entity_id.into(), new_pos);
    }

    fn set_entity_enabled(&mut self, entity_id: impl Into<String>, enabled: bool) {
        let id = entity_id.into();
        if enabled {
            self.disabled_entities.remove(&id);
        } else {
            self.disabled_entities.insert(id);
        }
    }

    fn collider_count(&self) -> usize {
        self.collider_handles.len()
    }
}

fn normalize_entity_key(class_name: &str) -> String {
    let mut normalized = String::with_capacity(class_name.len());

    for ch in class_name.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
        } else if !normalized.ends_with('_') {
            normalized.push('_');
        }
    }

    normalized.trim_matches('_').to_string()
}

fn find_spawn_position(map: &TiledMap, spawn_id: &str) -> Option<WorldPosition> {
    find_spawn_positions(map, spawn_id).into_iter().next()
}

fn find_spawn_positions(map: &TiledMap, spawn_id: &str) -> Vec<WorldPosition> {
    let mut positions = Vec::new();

    for layer in map.object_layers() {
        for object in &layer.objects {
            if let Some(object_spawn_id) = object.properties.get_string("spawn_id") {
                if object_spawn_id.eq_ignore_ascii_case(spawn_id) {
                    positions.push(WorldPosition {
                        x: object.x,
                        y: object.y,
                    });
                }
            }
        }
    }

    positions
}

#[allow(dead_code)]
fn find_actor_position(map: &TiledMap, actor_class_name: &str) -> Option<WorldPosition> {
    for layer in map.object_layers() {
        for object in &layer.objects {
            if object.class_name.eq_ignore_ascii_case(actor_class_name) {
                return Some(WorldPosition {
                    x: object.x,
                    y: object.y,
                });
            }
        }
    }

    None
}

fn draw_actor_sprite(texture: &Texture2D, world_position: WorldPosition, draw_size: Vec2) {
    // Tiled point objects are treated as anchor points, so draw centered on the point.
    let draw_x = world_position.x - draw_size.x * 0.5;
    let draw_y = world_position.y - draw_size.y * 0.5;

    draw_texture_ex(
        texture,
        draw_x,
        draw_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(draw_size),
            ..Default::default()
        },
    );
}
