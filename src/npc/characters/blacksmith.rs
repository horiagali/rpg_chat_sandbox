use crate::npc::{Character, Item}; // Reaches to npc/mod.rs
pub struct Blacksmith {
    pub name: String,
    pub base_prompt: String,   // Fixed backstory
    pub story_context: String, // Dynamic world events
    pub opinion: i32,          // -100 to 100
    pub inventory: Vec<Item>,
    pub health: f32,
    pub money: u32,
    pub position: (f32, f32),
    pub is_following: bool,
    pub history: Vec<rig::message::Message>
}

impl Blacksmith {
    pub fn new() -> Self {
        Self {
            name: "Hulgrim".to_string(),
            base_prompt: "You are Hulgrim, a weary blacksmith who values hard work over talk.".to_string(),
            story_context: "The forge is cold because the nearby mines are infested with goblins.".to_string(),
            opinion: 0,
            inventory: vec![Item { name: "Iron Sword".to_string(), price: 50, weight: 5.0 }],
            health: 100.0,
            money: 200,
            position: (10.0, 20.0),
            is_following: false,
            history: Vec::new(),
        }
    }

    // LLM ACTION: Move to coordinates
    pub fn go_to(&mut self, x: f32, y: f32) {
        if self.opinion > -20 {
            self.position = (x, y);
            println!("{} walks toward the coordinates.", self.name);
        }
    }

    // Add this helper to clear history when walking away
    pub fn forget_session(&mut self) {
        self.history.clear();
    }

    // LLM ACTION: Follow Player
    pub fn toggle_follow(&mut self) {
        if self.opinion >= 50 {
            self.is_following = true;
        } else {
            println!("{} grunts: 'I've got work to do. Leave me be.'", self.name);
        }
    }
}

impl Character for Blacksmith {
    fn get_full_prompt(&self) -> String {
        format!(
            "{}\nCURRENT SITUATION: {}\nYour opinion of the player is {}/100.",
            self.base_prompt, self.story_context, self.opinion
        )
    }

    fn update_opinion(&mut self, amount: i32) {
        // Clamp the change to max +-10 as requested
        let change = amount.clamp(-10, 10);
        self.opinion = (self.opinion + change).clamp(-100, 100);
    }

    fn sell_item(&mut self, index: usize) -> Option<Item> {
        if self.opinion < -10 {
            println!("{} refuses to trade with you.", self.name);
            return None;
        }
        if index < self.inventory.len() {
            Some(self.inventory.remove(index))
        } else {
            None
        }
    }

    fn attack_player(&mut self) {
        if self.opinion <= -80 {
            println!("{} swings his hammer at you!", self.name);
        }
    }

    fn take_damage(&mut self, amount: f32) {
        self.health -= amount;
        self.update_opinion(-50); // Attacking instantly tanks opinion
    }
}