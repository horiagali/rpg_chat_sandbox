use rig::client::CompletionClient;
use rig::completion::Chat;
use rig::providers::ollama;
use std::env;

pub async fn interact_with_blacksmith(
    client: &ollama::Client,
    preamble: &str,
    user_input: &str,
    history: &mut Vec<rig::message::Message>,
) -> String {
    // Allow overriding model from the environment:
    //   OLLAMA_MODEL=llama3.2:3b cargo run
    let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3:latest".to_string());

    let blacksmith_agent = client
        .agent(&model)
        .preamble(preamble)
        .build();

    // .chat() handles the session memory by reading and updating the history vector
    let response = match blacksmith_agent.chat(user_input, history.clone()).await {
        Ok(response) => response,
        Err(err) => {
            return format!(
                "I can't answer right now. Ollama request failed for model '{}'. \
Set OLLAMA_MODEL to an installed model or run `ollama pull {}`. \
Underlying error: {}",
                model, model, err
            );
        }
    };

    // Update the actual history buffer so the next turn remembers this one
    history.push(rig::message::Message::user(user_input));
    history.push(rig::message::Message::assistant(&response));

    response
}
