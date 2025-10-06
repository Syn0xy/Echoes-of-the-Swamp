use bevy::prelude::*;
use eots_core::states::{AppState, GameState};

use crate::ui::menu::biome_selection::systems::interact_with_biome_selector_button;

use super::systems::{despawn_biome_selection_menu, spawn_biome_selection_menu};

pub struct BiomeSelectionMenuPlugin;

impl Plugin for BiomeSelectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::BiomeSelection),
            spawn_biome_selection_menu.run_if(in_state(AppState::Game)),
        )
        .add_systems(
            OnExit(GameState::BiomeSelection),
            despawn_biome_selection_menu.run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            interact_with_biome_selector_button
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::BiomeSelection)),
        );
    }
}
