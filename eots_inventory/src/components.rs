use bevy::prelude::*;
use eots_item::{ItemData, ItemId};

use crate::errors::InventoryError;

#[derive(Reflect, Component, Debug, Clone, Copy)]
pub struct InventoryAttraction {
    pub attract_radius: f32,
    pub pickup_radius: f32,
}

#[derive(Reflect, Component, Debug, Clone, Copy)]
#[require(Transform)]
pub struct CollectibleItem {
    pub id: ItemId,
    pub count: u32,
}

#[derive(Reflect, Component, Clone)]
pub struct Inventory {
    pub(crate) slots: Vec<Option<ItemStack>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![None; 6],
        }
    }
}

#[derive(Reflect, Clone, Copy)]
pub struct ItemStack {
    pub id: ItemId,
    pub count: u32,
}

impl ItemStack {
    fn new(item: &ItemData, count: u32) -> (Self, u32) {
        match item.max_stack {
            Some(max) => {
                let new_count = max.min(count);
                let rest = count - new_count;

                (
                    Self {
                        id: item.id,
                        count: new_count,
                    },
                    rest,
                )
            }
            None => (Self { id: item.id, count }, 0),
        }
    }

    fn to_add(&self, item: &ItemData, count: u32) -> Option<(u32, u32)> {
        if self.id == item.id {
            match item.max_stack {
                Some(max) => {
                    let to_add = (max - self.count).min(count);
                    Some((to_add, count - to_add))
                }
                None => Some((count, 0)),
            }
        } else {
            None
        }
    }
}

impl Inventory {
    fn check_count(count: u32) -> Result<(), InventoryError> {
        if count == 0 {
            Err(InventoryError::InvalidAmount(0))
        } else {
            Ok(())
        }
    }

    pub fn add_item(&mut self, item: &ItemData, count: u32) -> Result<u32, InventoryError> {
        Inventory::check_count(count)?;

        let mut rest = count;

        for slot in &mut self.slots {
            if let Some(stack) = slot {
                if let Some((to_add, new_rest)) = stack.to_add(item, rest) {
                    stack.count += to_add;
                    rest = new_rest;

                    if rest == 0 {
                        return Ok(0);
                    }
                }
            }
        }

        for slot in &mut self.slots {
            if slot.is_none() {
                let (stack, new_rest) = ItemStack::new(item, rest);
                *slot = Some(stack);
                rest = new_rest;

                if rest == 0 {
                    return Ok(0);
                }
            }
        }

        Ok(rest)
    }

    pub fn remove_item(&mut self, _item: &ItemData, _count: u32) -> Result<(), InventoryError> {
        Ok(())
    }
}
