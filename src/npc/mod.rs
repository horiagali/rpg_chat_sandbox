pub mod characters; // This looks for a folder named 'characters'
pub mod agent;

pub trait Character {
    fn get_full_prompt(&self) -> String;
    fn update_opinion(&mut self, amount: i32);
    fn take_damage(&mut self, amount: f32);
    fn sell_item(&mut self, item_index: usize) -> Option<Item>;
    fn attack_player(&mut self);
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub weight: f32,
}