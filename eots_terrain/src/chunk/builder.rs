use std::collections::HashMap;

use bevy::prelude::*;

use eots_constants::{biome::TerrainDescriptor, terrain};
use perlin_noise::NoiseOffset;

use crate::ChunkData;

pub struct ChunkBuilder {
    offset: NoiseOffset,
    noisemaps: HashMap<&'static str, Vec<f32>>,
}

impl ChunkBuilder {
    pub fn with_offset(offset: NoiseOffset) -> Self {
        Self {
            offset,
            noisemaps: Default::default(),
        }
    }

    pub fn with_position(position: Vec2) -> Self {
        Self::with_offset(NoiseOffset::from(position.to_array()))
    }

    pub fn generate_noise(mut self, descriptor: &TerrainDescriptor) -> Self {
        self.noisemaps.insert(
            descriptor.label,
            perlin_noise::generate_map(
                terrain::CHUNK_SIZE,
                terrain::CHUNK_SIZE,
                descriptor.noise_descriptor,
                &self.offset,
            ),
        );
        self
    }

    pub fn build(self) -> ChunkData {
        ChunkData {
            _offset: self.offset,
            noisemaps: self.noisemaps,
        }
    }
}
