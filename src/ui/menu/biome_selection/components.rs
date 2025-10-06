use bevy::prelude::*;
use eots_constants::biome::BiomeId;

use crate::ui::menu::biome_selection::styles::{BACKGROUND_COLOR, NORMAL_BUTTON_COLOR};

#[derive(Component)]
#[require(BackgroundColor(BACKGROUND_COLOR))]
pub struct BiomeSelectionMenu;

#[derive(Component)]
#[require(Button, BackgroundColor(NORMAL_BUTTON_COLOR))]
pub struct BiomeSelectorButton {
    pub biome: BiomeId,
}
