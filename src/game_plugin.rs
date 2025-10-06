use bevy::prelude::*;
use eots_constants::{biome::BiomeId, player::PLAYER_CREATURE_ID};
use eots_core::{SessionData, states::GameState};
use eots_player::SpawnPlayerEvent;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SessionData {
            current_biome: Some(BiomeId::Swamp),
            ..Default::default()
        })
        .add_plugins((
            eots_creature::CreaturePlugin,
            eots_player::PlayerPlugin,
            eots_inventory::ItemPlugin,
            eots_skeleton_rig::SkeletonRigPlugin,
            eots_terrain::EndlessTerrainPlugin,
            eots_spawner::SpawnerPlugin,
            eots_perception::PerceptionPlugin,
        ))
        .add_systems(OnEnter(GameState::GamePhase), setup_game);
    }
}

fn setup_game(mut wmessage_spawn_player: MessageWriter<SpawnPlayerEvent>) {
    wmessage_spawn_player.write(SpawnPlayerEvent {
        name: "Moi",
        creature: PLAYER_CREATURE_ID,
    });
}
