use bevy::prelude::*;

use crate::menu::main_menu::styles::{BACKGROUND_COLOR, NORMAL_BUTTON_COLOR};

#[derive(Component)]
#[require(BackgroundColor(BACKGROUND_COLOR))]
pub struct MainMenu;

#[derive(Component)]
#[require(Button, BackgroundColor(NORMAL_BUTTON_COLOR))]
pub struct PlayButton;

#[derive(Component)]
#[require(Button, BackgroundColor(NORMAL_BUTTON_COLOR))]
pub struct QuitButton;
