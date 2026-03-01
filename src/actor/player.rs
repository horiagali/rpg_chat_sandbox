#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerStats {
    pub level: u32,
    pub health: u32,
    pub max_health: u32,
    pub stamina: u32,
    pub max_stamina: u32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            level: 1,
            health: 100,
            max_health: 100,
            stamina: 100,
            max_stamina: 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerData {
    pub id: String,
    pub display_name: String,
    pub stats: PlayerStats,
    pub inventory: Vec<String>,
    pub gold: u32,
    pub can_move: bool,
    pub is_in_dialogue: bool,
    pub active_quest_ids: Vec<String>,
    pub completed_quest_ids: Vec<String>,
    pub spawn_point_id: Option<String>,
    pub faction_reputation: i32,
    pub position: WorldPosition,
}

impl PlayerData {
    pub fn new(id: impl Into<String>, display_name: impl Into<String>, position: WorldPosition) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            stats: PlayerStats::default(),
            inventory: Vec::new(),
            gold: 0,
            can_move: true,
            is_in_dialogue: false,
            active_quest_ids: Vec::new(),
            completed_quest_ids: Vec::new(),
            spawn_point_id: None,
            faction_reputation: 0,
            position,
        }
    }
}
