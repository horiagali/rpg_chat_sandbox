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
    blacksmith: NpcData,
}

impl Game {
    async fn new() -> Self {
        let tiled_map = TiledMap::load("assets/map.json")
            .await
            .expect("Failed to load map");
        let world_truth = WorldTruthIndex::from_map(&tiled_map);
        let player_position = find_actor_position(&tiled_map, "Player")
            .expect("Could not find `Player` object in map EntityLayer");
        let blacksmith_position = find_actor_position(&tiled_map, "Blacksmith")
            .expect("Could not find `Blacksmith` object in map EntityLayer");
        let player = PlayerData::new("player_01", "Player", player_position);
        let blacksmith = NpcData::new(
            "blacksmith_01",
            "Blacksmith",
            NpcRole::Merchant,
            NpcAttitude::Friendly,
            blacksmith_position,
        );

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
            blacksmith,
        }
    }

    async fn frame(&mut self) {
        clear_background(WHITE);
        let view_min = vec2(0.0, 0.0);
        let view_max = vec2(MAP_PIXEL_WIDTH as f32, MAP_PIXEL_HEIGHT as f32);
        self.tiled_map.draw(view_min, view_max);

        draw_circle(self.player.position.x, self.player.position.y, 6.0, RED);
        draw_text(
            "Player",
            self.player.position.x + 10.0,
            self.player.position.y + 5.0,
            18.0,
            RED,
        );

        draw_circle(
            self.blacksmith.position.x,
            self.blacksmith.position.y,
            6.0,
            BLUE,
        );
        draw_text(
            "Blacksmith",
            self.blacksmith.position.x + 10.0,
            self.blacksmith.position.y + 5.0,
            18.0,
            BLUE,
        );

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

        for (layer_index, layer) in map.object_layers().iter().enumerate() {
            for (object_index, obj) in layer.objects.iter().enumerate() {
                let handle = ObjectHandle {
                    layer_index,
                    object_index,
                };

                if let Some(entity_id) = obj.properties.get_string("entity_id") {
                    entity_handles.insert(entity_id.to_string(), handle);
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
