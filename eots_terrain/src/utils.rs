use bevy::prelude::*;
use eots_constants::{biome::BiomeData, terrain};

use crate::{ChunkData, chunk::ChunkBuilder};

pub(super) fn from_position_to_coord(position: &Vec2) -> IVec2 {
    IVec2 {
        x: (position.x / terrain::CHUNK_SIZE_F32).round() as i32,
        y: (position.y / terrain::CHUNK_SIZE_F32).round() as i32,
    }
}

pub(super) const fn from_coord_to_position(coord: &IVec2) -> Vec2 {
    Vec2 {
        x: coord.x as f32 * terrain::CHUNK_SIZE_F32,
        y: coord.y as f32 * terrain::CHUNK_SIZE_F32,
    }
}

pub(super) fn distance_threshold(a: &Vec2, b: &Vec2) -> bool {
    (b - a).length() >= terrain::VIEWER_DISTANCE_THRESHOLD
}

pub(super) fn chunk_is_visible(viewer_position: &Vec2, chunk_position: &Vec2) -> bool {
    viewer_position.distance_squared(chunk_position + terrain::HALF_CHUNK_SIZE_F32)
        <= terrain::CHUNK_VIEW_RADIUS_SQR * terrain::CHUNK_SIZE_SQR
}

pub(super) fn build_chunk_data(biome_data: &BiomeData, chunk_position: Vec2) -> ChunkData {
    let mut builder = ChunkBuilder::with_position(chunk_position);

    for descriptor in biome_data.terrain_descriptors {
        builder = builder.generate_noise(descriptor);
    }

    builder.build()
}

pub(super) const fn get_sprite_path(value: f32) -> &'static str {
    match value {
        a if a > 0.5 => "sprites/grass.png",
        _ => "sprites/dirt.png",
    }
}
