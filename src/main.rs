use macroquad::prelude::Conf;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Chat RPG".to_owned(),
        ..Default::default()
    };

    if let Ok(map) = rpg::map::RuntimeMapAdapter::from_tiled_json_wall_layer("game/map.json") {
        conf.window_width = (map.width as f32 * map.tile_size).round() as i32;
        conf.window_height = (map.height as f32 * map.tile_size).round() as i32;
    }

    conf
}

#[macroquad::main("RPG Chat Sandbox")]
async fn main() {
    rpg::game::run().await;
}
