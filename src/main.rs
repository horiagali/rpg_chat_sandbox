// mod npc;

// use crate::npc::agent::interact_with_blacksmith;
// use crate::npc::characters::Blacksmith;
// use rig::client::Nothing;
// use rig::providers::ollama;
// use std::io::{self, Write};

// #[tokio::main]
// async fn main() {
//     // Legacy terminal chat loop kept for reference while integrating NPC chat into the in-game loop.
//     let client = ollama::Client::new(Nothing).expect("Failed to initialize Ollama client");
//     let mut blacksmith = Blacksmith::new();

//     println!("--- Talk to the Blacksmith (Type 'exit' to quit, 'leave' to reset memory) ---");

//     loop {
//         print!("You: ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         let input = input.trim();

//         if input == "exit" {
//             break;
//         }

//         if input == "leave" {
//             blacksmith.forget_session();
//             println!(
//                 "(You walked away. The Blacksmith forgot the conversation, but still has an opinion of {} of you.)\n",
//                 blacksmith.opinion
//             );
//             continue;
//         }

//         blacksmith.update_opinion(input);

//         let response = interact_with_blacksmith(
//             &client,
//             &blacksmith.get_preamble(),
//             input,
//             &mut blacksmith.history,
//         )
//         .await;

//         println!("Blacksmith: {}", response);
//         println!("(Opinion: {})\n", blacksmith.opinion);
//     }
// }

use macroquad::prelude::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "RPG Chat Sandbox".to_string(),
        window_width: rpg_chat_sandbox::game::MAP_PIXEL_WIDTH,
        window_height: rpg_chat_sandbox::game::MAP_PIXEL_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rpg_chat_sandbox::game::run().await;
}
