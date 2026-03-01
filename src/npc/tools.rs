use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AttackArgs { pub target: String }

#[derive(Deserialize, Serialize)]
pub struct TradeArgs { pub item: String, pub price: u32 }

// Tool: Attack the player
pub fn attack(args: AttackArgs) -> String {
    format!("SYSTEM_ACTION: You swing your heavy hammer at {}!", args.target)
}

// Tool: Follow the player
pub fn follow() -> String {
    "SYSTEM_ACTION: I'm right behind you, lead the way.".to_string()
}

// Tool: Sell an item
pub fn sell(args: TradeArgs) -> String {
    if args.price == 0 {
        format!("SYSTEM_ACTION: You hand over the {} for free. 'On the house!'", args.item)
    } else {
        format!("SYSTEM_ACTION: Sold {} for {} gold.", args.item, args.price)
    }
}