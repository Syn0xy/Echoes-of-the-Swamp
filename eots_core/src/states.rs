use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AppState {
    Menu,
    Game,
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameState {
    // Init,
    BiomeSelection,
    // Loading,
    GamePhase,
    // BossPhase,
    // GameOver,
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MenuState {
    Main,
    // Settings,
}
