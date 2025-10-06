use bevy::prelude::*;
use eots_core::{SessionData, states::GameState};

use crate::ui::menu::biome_selection::{
    components::BiomeSelectorButton,
    styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
};

pub fn interact_with_biome_selector_button(
    mut session_data: ResMut<SessionData>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut button_query: Query<
        (&BiomeSelectorButton, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (biome_selector, interaction, mut background_color) in &mut button_query {
        match interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                session_data.set_current_biome(biome_selector.biome);
                game_state_next_state.set(GameState::GamePhase);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
