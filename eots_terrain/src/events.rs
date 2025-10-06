use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy)]
pub struct CheckChunksEvent {
    pub viewer_position: Vec2,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct UpdateChunkEvent {
    pub chunk_coord: IVec2,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct HideChunkEvent {
    pub chunk_coord: IVec2,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct SpawnChunkEvent {
    pub chunk_coord: IVec2,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct SpawnChunkTilesEvent {
    pub chunk_entity: Entity,
}
