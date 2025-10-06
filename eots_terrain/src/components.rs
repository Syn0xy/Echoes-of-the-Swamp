use std::collections::HashMap;

use bevy::prelude::*;
use perlin_noise::NoiseOffset;

#[derive(Component)]
pub struct Chunk;

#[derive(Component)]
pub struct ChunkData {
    pub _offset: NoiseOffset,
    pub noisemaps: HashMap<&'static str, Vec<f32>>,
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct TerrainViewer {
    pub old_position: Option<Vec2>,
}
