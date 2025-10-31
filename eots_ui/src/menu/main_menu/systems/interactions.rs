use bevy::prelude::*;
use eots_core::states::{AppState, GameState};

use crate::menu::main_menu::{
    components::{PlayButton, QuitButton},
    styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
};

pub fn interact_with_play_button(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
                game_state_next_state.set(GameState::BiomeSelection);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut wmessage_app_exit: MessageWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                wmessage_app_exit.write(AppExit::Success);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
