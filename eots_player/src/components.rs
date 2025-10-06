use bevy::prelude::*;
use eots_constants::{bubble::{Bubble}, element::ElementType};
use eots_item::ItemId;

#[derive(Component, Default)]
pub struct Player {
    pub _name: &'static str,
    pub element: ElementType,
    pub bubble: Box<dyn Bubble>,
    pub protection: Option<ItemId>,
    pub consommables: Vec<ItemId>,
}