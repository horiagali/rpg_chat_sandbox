use crate::actor::player::WorldPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcAttitude {
    Friendly,
    Neutral,
    Hostile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcRole {
    Merchant,
    QuestGiver,
    Guard,
    Villager,
    Crafter,
    Enemy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpcCombatStats {
    pub health: u32,
    pub max_health: u32,
    pub attack: u32,
    pub defense: u32,
}

impl Default for NpcCombatStats {
    fn default() -> Self {
        Self {
            health: 100,
            max_health: 100,
            attack: 10,
            defense: 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpcDialogueState {
    pub opinion_of_player: i32,
    pub has_met_player: bool,
    pub dialogue_memory: Vec<String>,
}

impl Default for NpcDialogueState {
    fn default() -> Self {
        Self {
            opinion_of_player: 0,
            has_met_player: false,
            dialogue_memory: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NpcData {
    pub id: String,
    pub display_name: String,
    pub role: NpcRole,
    pub attitude: NpcAttitude,
    pub combat: NpcCombatStats,
    pub dialogue: NpcDialogueState,
    pub position: WorldPosition,
    pub patrol_point_ids: Vec<String>,
    pub shop_item_ids: Vec<String>,
    pub quest_ids: Vec<String>,
    pub is_active: bool,
}

impl NpcData {
    pub fn new(
        id: impl Into<String>,
        display_name: impl Into<String>,
        role: NpcRole,
        attitude: NpcAttitude,
        position: WorldPosition,
    ) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            role,
            attitude,
            combat: NpcCombatStats::default(),
            dialogue: NpcDialogueState::default(),
            position,
            patrol_point_ids: Vec::new(),
            shop_item_ids: Vec::new(),
            quest_ids: Vec::new(),
            is_active: true,
        }
    }
}
