use macroquad::prelude::*;
use macroquad_tiled_clone::{IrObject, Map as TiledMap};
use std::collections::{HashMap, HashSet};

pub async fn run() {
    let mut game = Game::new().await;

    loop {
        game.frame().await;
    }
}

struct Game {
    tiled_map: TiledMap,
    world_truth: WorldTruthIndex,
}

impl Game {
    async fn new() -> Self {
        let tiled_map = TiledMap::load("assets/map.json")
            .await
            .expect("Failed to load map");
        let world_truth = WorldTruthIndex::from_map(&tiled_map);

        println!(
            "Loaded map truth index: {} entities, {} spawn points, {} colliders",
            world_truth.entity_handles.len(),
            world_truth.spawn_handles.len(),
            world_truth.collider_count()
        );

        Self {
            tiled_map,
            world_truth,
        }
    }

    async fn frame(&mut self) {
        clear_background(WHITE);
        let view_min = vec2(0.0, 0.0);
        let view_max = vec2(screen_width(), screen_height());
        self.tiled_map.draw(view_min, view_max);

        // Example of querying "map as truth":
        // read by stable id -> handle -> object record.
        if let Some(player_spawn) = self.world_truth.get_spawn(self, "player_start") {
            draw_circle(player_spawn.x, player_spawn.y, 4.0, RED);
        }

        if let Some(blacksmith_pos) = self.world_truth.entity_world_pos(self, "blacksmith_01") {
            draw_circle(blacksmith_pos.x, blacksmith_pos.y, 5.0, BLUE);
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
