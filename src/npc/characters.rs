use rig::message::Message; // Required for session history

pub struct Blacksmith {
    pub opinion: i32,
    pub history: Vec<Message>, // Stores the current dialogue session
}

impl Blacksmith {
    pub fn new() -> Self {
        Self { 
            opinion: 0,
            history: Vec::new(), // Initialize an empty history
        }
    }

    /// Provides the LLM with the current state and behavioral rules
    pub fn get_preamble(&self) -> String {
        format!(
            "You are a Blacksmith. Your current Opinion of the player is: {}.
            LOGIC RULES:
            - If Opinion > 0: You are willing to sell a sword.
            - If Opinion > 50: You are willing to follow the player.
            - If Opinion > 100: You give the sword for free (price 0).
            - If Opinion < -100: You must ATTACK the player.
            
            Context: You remember what was said in this current conversation. 
            If the player is polite, your opinion improves. If they are insulting, it drops.",
            self.opinion
        )
    }

    /// Updates the numerical opinion based on simple keyword sentiment
    pub fn update_opinion(&mut self, text: &str) {
        let text = text.to_lowercase();
        if text.contains("please") || text.contains("thank") { 
            self.opinion += 10; 
        }
        if text.contains("ugly") || text.contains("hate") || text.contains("stupid") { 
            self.opinion -= 10; 
        }
    }

    /// Clears the dialogue history so the NPC 'forgets' the next time you talk
    pub fn forget_session(&mut self) {
        self.history.clear();
    }
}