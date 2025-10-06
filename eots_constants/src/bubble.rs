use bevy::prelude::*;

mod classic_bubble;

pub trait Bubble: Reflect {
    fn damage(&self) -> u32;
}
