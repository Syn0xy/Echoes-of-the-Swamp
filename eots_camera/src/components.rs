use bevy::prelude::*;

#[derive(Reflect, Component, Debug, Default, Clone, Copy)]
pub struct SmoothCameraTarget {
    pub(crate) decay_rate: f32,
}

impl SmoothCameraTarget {
    pub const fn new(decay_rate: f32) -> Self {
        Self { decay_rate }
    }
}
