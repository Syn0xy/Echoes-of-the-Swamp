use bevy::prelude::*;

use crate::{
    Chunk, EndlessTerrain, TerrainViewer, chunk,
    events::{
        CheckChunksEvent, HideChunkEvent, SpawnChunkEvent, SpawnChunkTilesEvent, UpdateChunkEvent,
    },
    utils,
};

pub struct EndlessTerrainPlugin;

impl Plugin for EndlessTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EndlessTerrain::default())
            .add_message::<CheckChunksEvent>()
            .add_message::<UpdateChunkEvent>()
            .add_message::<HideChunkEvent>()
            .add_message::<SpawnChunkEvent>()
            .add_message::<SpawnChunkTilesEvent>()
            .add_systems(
                Update,
                (
                    check_viewer_threshold,
                    update_chunks,
                    update_chunk,
                    hide_visible_chunks,
                    chunk::spawn_chunk,
                    chunk::spawn_chunk_tiles,
                )
                    .chain(),
            );
    }
}

fn check_viewer_threshold(
    mut wmessage_update_chunks: MessageWriter<CheckChunksEvent>,
    mut terrain_viewer_q: Query<(&Transform, &mut TerrainViewer)>,
) {
    for (viewer_transform, mut terrain_viewer) in &mut terrain_viewer_q {
        let viewer_position = viewer_transform.translation.truncate();

        if terrain_viewer
            .old_position
            .map_or(true, |old_viewer_position| {
                utils::distance_threshold(&old_viewer_position, &viewer_position)
            })
        {
            terrain_viewer.old_position = Some(viewer_position);
            wmessage_update_chunks.write(CheckChunksEvent { viewer_position });
        }
    }
}

fn update_chunks(
    mut rmessage_update_chunks: MessageReader<CheckChunksEvent>,
    mut wmessage_hide_visible_chunk: MessageWriter<HideChunkEvent>,
    mut wmessage_spawn_chunk: MessageWriter<UpdateChunkEvent>,
    mut endless_terrain: ResMut<EndlessTerrain>,
) {
    use eots_constants::terrain::CHUNK_VIEW_RADIUS;

    for &CheckChunksEvent { viewer_position } in rmessage_update_chunks.read() {
        let viewer_coord = utils::from_position_to_coord(&viewer_position);

        for &chunk_coord in &endless_terrain.visible_chunks {
            let chunk_position = utils::from_coord_to_position(&chunk_coord);

            if !utils::chunk_is_visible(&viewer_position, &chunk_position) {
                wmessage_hide_visible_chunk.write(HideChunkEvent { chunk_coord });
            }
        }

        endless_terrain.visible_chunks.clear();

        for ox in -CHUNK_VIEW_RADIUS..CHUNK_VIEW_RADIUS {
            for oy in -CHUNK_VIEW_RADIUS..CHUNK_VIEW_RADIUS {
                let chunk_coord = (ox + viewer_coord.x, oy + viewer_coord.y).into();
                let chunk_position = utils::from_coord_to_position(&chunk_coord);

                if utils::chunk_is_visible(&viewer_position, &chunk_position) {
                    endless_terrain.visible_chunks.push(chunk_coord);

                    wmessage_spawn_chunk.write(UpdateChunkEvent { chunk_coord });
                }
            }
        }
    }
}

fn update_chunk(
    mut rmessage_update_chunk: MessageReader<UpdateChunkEvent>,
    mut wmessage_spawn_chunk: MessageWriter<SpawnChunkEvent>,
    mut chunk_visibilities: Query<&mut Visibility, With<Chunk>>,
    endless_terrain: Res<EndlessTerrain>,
) {
    for &UpdateChunkEvent { chunk_coord } in rmessage_update_chunk.read() {
        if let Some(chunk_entity) = endless_terrain.chunks.get(&chunk_coord) {
            if let Ok(mut visibility) = chunk_visibilities.get_mut(*chunk_entity) {
                *visibility = Visibility::Visible;
            }
        } else {
            wmessage_spawn_chunk.write(SpawnChunkEvent { chunk_coord });
        }
    }
}

fn hide_visible_chunks(
    mut rmessage_update_chunk: MessageReader<HideChunkEvent>,
    mut chunk_visibilities: Query<&mut Visibility, With<Chunk>>,
    endless_terrain: Res<EndlessTerrain>,
) {
    for HideChunkEvent { chunk_coord } in rmessage_update_chunk.read() {
        if let Some(chunk_entity) = endless_terrain.chunks.get(chunk_coord) {
            if let Ok(mut visibility) = chunk_visibilities.get_mut(*chunk_entity) {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
