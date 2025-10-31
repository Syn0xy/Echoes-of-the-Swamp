use bevy::prelude::*;
use eots_core::states::{AppState, MenuState};

use super::systems::{
    despawn_main_menu, interact_with_play_button, interact_with_quit_button, spawn_main_menu,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_main_menu)
            .add_systems(OnExit(AppState::Menu), despawn_main_menu)
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::Menu))
                    .run_if(in_state(MenuState::Main)),
            );
    }
}
