use bevy::prelude::*;
use eots_constants::terrain;
use eots_core::SessionData;

use crate::{
    Chunk, ChunkData, EndlessTerrain,
    events::{SpawnChunkEvent, SpawnChunkTilesEvent},
    utils,
};

pub fn spawn_chunk(
    mut commands: Commands,
    mut rmessage_spawn_chunk: MessageReader<SpawnChunkEvent>,
    mut wmessage_spawn_chunk_tiles: MessageWriter<SpawnChunkTilesEvent>,
    mut endless_terrain: ResMut<EndlessTerrain>,
    session_data: Res<SessionData>,
) {
    let Some(biome_data) = session_data.current_biome_data() else {
        return;
    };

    for SpawnChunkEvent { chunk_coord } in rmessage_spawn_chunk.read() {
        let chunk_position = utils::from_coord_to_position(&chunk_coord);
        let chunk_entity = commands
            .spawn((
                Name::new(format!("Chunk ({}, {})", chunk_coord.x, chunk_coord.y)),
                Chunk,
                utils::build_chunk_data(biome_data, chunk_position),
                Transform::from_translation(chunk_position.extend(0.0)),
                InheritedVisibility::default(),
                Visibility::Visible,
            ))
            .id();

        endless_terrain.chunks.insert(*chunk_coord, chunk_entity);
        wmessage_spawn_chunk_tiles.write(SpawnChunkTilesEvent { chunk_entity });
    }
}

pub fn spawn_chunk_tiles(
    mut commands: Commands,
    mut rmessage_spawn_chunk_tiles: MessageReader<SpawnChunkTilesEvent>,
    chunk_data: Query<&ChunkData, With<Chunk>>,
    asset_server: Res<AssetServer>,
) {
    for &SpawnChunkTilesEvent { chunk_entity } in rmessage_spawn_chunk_tiles.read() {
        let Ok(data) = chunk_data.get(chunk_entity) else {
            continue;
        };

        let Some(terrainmap) = data.noisemaps.get("map") else {
            continue;
        };

        for ox in 0..terrain::CHUNK_SIZE {
            for oy in 0..terrain::CHUNK_SIZE {
                let Some(noise_value) = terrainmap
                    .get((ox * terrain::CHUNK_SIZE + (terrain::CHUNK_SIZE - oy - 1)) as usize)
                else {
                    continue;
                };

                let sprite_path = utils::get_sprite_path(*noise_value);
                let tile_image = asset_server.load(sprite_path);
                let tile_position = Vec2::new(ox as f32 + 0.5, oy as f32 + 0.5);

                commands.spawn((
                    Transform::from_translation(tile_position.extend(0.0)),
                    Sprite {
                        image: tile_image.clone(),
                        custom_size: Some(Vec2::ONE),
                        ..default()
                    },
                    ChildOf(chunk_entity),
                ));
            }
        }
    }
}
