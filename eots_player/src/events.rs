use bevy::prelude::*;
use eots_constants::creature::CreatureId;

#[derive(Message, Debug, Clone, Copy)]
pub struct SpawnPlayerEvent {
    pub name: &'static str,
    pub creature: CreatureId,
}
