use bevy::prelude::*;

use crate::bubble::Bubble;

#[derive(Reflect)]
pub struct ClassicBubble;

impl Bubble for ClassicBubble {
    fn damage(&self) -> u32 {
        1
    }
}

impl Default for Box<dyn Bubble> {
    fn default() -> Self {
        Box::new(ClassicBubble)
    }
}
