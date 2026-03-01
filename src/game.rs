use macroquad::prelude::*;
use macroquad_tiled_clone::Map as TiledMap;
use std::path::Path;

pub async fn run() {
    let mut game = Game::new().await;

    loop {
        game.frame().await;
    }
}

struct Game {
    tiled_map: TiledMap,
}

impl Game {
    async fn new() -> Self {
        let tiled_map = TiledMap::load("game/map.json")
            .await
            .expect("Failed to load map");
        Self { tiled_map }
    }

    async fn frame(&mut self) {
        clear_background(WHITE);
        let view_min = vec2(0.0, 0.0);
        let view_max = vec2(screen_width(), screen_height());
        self.tiled_map.draw(view_min, view_max);
    }
}
