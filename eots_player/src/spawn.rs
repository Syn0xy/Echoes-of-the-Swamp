use bevy::prelude::*;
use eots_constants::{creature::PLAYER_GROUP_LAYER, player};
use eots_creature::components::{Creature, CreatureStats};
use eots_inventory::{Inventory, InventoryAttraction};
use eots_perception::{GroupLayers, Targetable};

use crate::{Player, SpawnPlayerEvent};

pub(crate) fn spawn_player(
    mut commands: Commands,
    mut rmessage_spawn_player: MessageReader<SpawnPlayerEvent>,
) {
    for SpawnPlayerEvent { name, creature } in rmessage_spawn_player.read() {
        let skeleton_entity =
            eots_skeleton_rig::build_skeleton_rig(&mut commands, creature.skeleton_rig());

        commands.spawn((
            Name::new(format!("Player ({})", name)),
            Player {
                _name: name,
                ..default()
            },
            Creature(*creature),
            CreatureStats::from(creature),
            GroupLayers::new().with(PLAYER_GROUP_LAYER),
            Targetable,
            // Perception::new(20.0),
            Inventory::default(),
            InventoryAttraction {
                attract_radius: 10.0,
                pickup_radius: 0.5,
            },
            eots_camera::SmoothCameraTarget::new(player::SMOOTH_CAMERA_DECAY_RATE),
            eots_skeleton_rig::SkeletonRigEntity(skeleton_entity),
            // eots_terrain::TerrainViewer::default(),
        ));
    }
}
