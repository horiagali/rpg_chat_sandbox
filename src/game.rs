use macroquad::prelude::*;
use macroquad_tiled_clone::{IrObject, Map as TiledMap};
use std::collections::HashMap;

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
        let tiled_map = TiledMap::load("game/map.json")
            .await
            .expect("Failed to load map");
        let world_truth = WorldTruthIndex::from_map(&tiled_map);

        println!(
            "Loaded map truth index: {} entities, {} spawn points, {} colliders",
            world_truth.entities_by_id.len(),
            world_truth.spawns_by_id.len(),
            world_truth.collider_objects.len()
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

        draw_text(
            &format!("entities: {}", self.world_truth.entities_by_id.len()),
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
    entities_by_id: HashMap<String, ObjectHandle>,
    spawns_by_id: HashMap<String, ObjectHandle>,
    collider_objects: Vec<ObjectHandle>,
}

impl WorldTruthIndex {
    fn from_map(map: &TiledMap) -> Self {
        let mut entities_by_id = HashMap::new();
        let mut spawns_by_id = HashMap::new();
        let mut collider_objects = Vec::new();

        for (layer_index, layer) in map.object_layers().iter().enumerate() {
            for (object_index, obj) in layer.objects.iter().enumerate() {
                let handle = ObjectHandle {
                    layer_index,
                    object_index,
                };

                if let Some(entity_id) = obj.properties.get_string("entity_id") {
                    entities_by_id.insert(entity_id.to_string(), handle);
                }

                if let Some(spawn_id) = obj.properties.get_string("spawn_id") {
                    spawns_by_id.insert(spawn_id.to_string(), handle);
                }

                if obj.properties.get_bool("collider").unwrap_or(false) {
                    collider_objects.push(handle);
                }
            }
        }

        Self {
            entities_by_id,
            spawns_by_id,
            collider_objects,
        }
    }

    fn get_spawn<'a>(&self, game: &'a Game, spawn_id: &str) -> Option<&'a IrObject> {
        let handle = self.spawns_by_id.get(spawn_id)?;
        game.object_from_handle(*handle)
    }
}
