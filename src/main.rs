mod npc;

use rig::providers::openai;
use std::io::{self, Write};
use crate::npc::characters::Blacksmith;
use crate::npc::agent::interact_with_blacksmith;

#[tokio::main]
async fn main() {
    // 1. Initialize the local Ollama client
    let client = openai::Client::from_url("ollama", "http://localhost:11434/v1");

    // 2. Create your Blacksmith instance
    let mut blacksmith = Blacksmith::new();

    println!("--- Talk to the Blacksmith (Type 'exit' to quit, 'leave' to reset memory) ---");

    loop {
        print!("You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        // Exit conditions
        if input == "exit" { break; }
        
        // Simulate walking away: clear history but keep the long-term opinion
        if input == "leave" {
            blacksmith.forget_session();
            println!("(You walked away. The Blacksmith forgot the conversation, but still has an opinion of {} of you.)\n", blacksmith.opinion);
            continue;
        }

        // 3. Update the internal state (Opinion) based on player input
        blacksmith.update_opinion(input);

        // 4. Send the prompt, preamble, and current history to the agent
        let response = interact_with_blacksmith(
            &client,
            &blacksmith.get_preamble(),
            input,
            &mut blacksmith.history,
        ).await;

        // 5. Display the NPC response and current state
        println!("Blacksmith: {}", response);
        println!("(Opinion: {})\n", blacksmith.opinion);
    }
}
