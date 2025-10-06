use bevy::prelude::*;
use eots_item::ItemId;

#[derive(Debug, Clone, Copy)]
pub enum InventoryAction {
    Add,
    Remove,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct ItemActionMessage {
    pub entity: Entity,
    pub action: InventoryAction,
    pub id: ItemId,
    pub count: u32,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct SpawnItemMessage {
    pub id: ItemId,
    pub count: u32,
}
