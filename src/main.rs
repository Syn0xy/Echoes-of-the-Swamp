use bevy::{prelude::*, window::PresentMode};
use eots_constants::window;
use eots_core::states::{AppState, GameState, MenuState};

mod game_plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: window::WINDOW_TITLE.to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            eots_asset_manager::AssetManagerPlugin,
            eots_camera::CameraPlugin,
            game_plugin::GamePlugin,
            eots_ui::UIPlugin,
        ))
        .insert_state(AppState::Game)
        .insert_state(MenuState::Main)
        .insert_state(GameState::GamePhase)
        .run();
}
