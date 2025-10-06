use bevy::prelude::*;

#[derive(Reflect, Component, Debug, Clone)]
pub struct Perception {
    pub vision_range: f32,
    pub primary_target: Option<Entity>,
    pub visible_targets: Vec<Entity>,
}

#[derive(Reflect, Component, Default, Debug, Clone)]
pub struct Targetable;

#[derive(Reflect, Component, Debug, Default, Clone, Copy)]
pub struct GroupLayers {
    mask: u8,
}

impl Perception {
    pub fn new(vision_range: f32) -> Self {
        Self {
            vision_range,
            primary_target: Default::default(),
            visible_targets: Default::default(),
        }
    }
}

impl GroupLayers {
    pub const fn new() -> Self {
        Self { mask: 0 }
    }

    pub const fn with(mut self, layer: u8) -> Self {
        self.mask |= 1 << layer;
        self
    }

    pub const fn intersects(&self, other: &GroupLayers) -> bool {
        self.mask & other.mask != 0
    }
}
