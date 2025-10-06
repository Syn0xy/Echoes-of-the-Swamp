use bevy::prelude::*;

pub type ItemId = u32;

#[derive(serde::Deserialize, Reflect, Asset, Debug)]
pub struct ItemData {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub sprite_path: String,
    pub max_stack: Option<u32>,
    pub effects: Vec<ItemEffect>,
}

#[derive(serde::Deserialize, Reflect, Debug, Clone, Copy)]
pub enum ItemEffect {
    Heal { amount: u32 },
}
