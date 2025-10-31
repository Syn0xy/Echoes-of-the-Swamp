use bevy::prelude::*;
use eots_constants::biome::BiomeId;
use eots_core::SessionData;

use crate::menu::biome_selection::{
    components::{BiomeSelectionMenu, BiomeSelectorButton},
    styles::{BUTTON_FONT_SIZE, NORMAL_BUTTON_TEXT_COLOR, TEXT_FONT_PATH},
};

pub fn spawn_biome_selection_menu(
    mut commands: Commands,
    mut session_data: ResMut<SessionData>,
    asset_server: Res<AssetServer>,
) {
    let biome_pool = session_data.reset_biome_pool();
    commands.spawn(build_biome_selection_menu(biome_pool, &asset_server));
}

pub fn despawn_biome_selection_menu(
    mut commands: Commands,
    biome_selection_menu_query: Query<Entity, With<BiomeSelectionMenu>>,
) {
    if let Ok(biome_selection_menu_entity) = biome_selection_menu_query.single() {
        commands.entity(biome_selection_menu_entity).despawn();
    }
}

fn build_biome_selection_menu(
    biome_pool: &mut Vec<BiomeId>,
    asset_server: &Res<AssetServer>,
) -> impl Bundle {
    let menu_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(100.0),
        ..default()
    };

    let button_text_font = TextFont {
        font: asset_server.load(TEXT_FONT_PATH),
        font_size: BUTTON_FONT_SIZE,
        ..default()
    };

    let biome_1 = biome_pool.pop().unwrap();
    let biome_2 = biome_pool.pop().unwrap();
    let biome_3 = biome_pool.pop().unwrap();

    (
        BiomeSelectionMenu,
        menu_node,
        children![
            (
                BiomeSelectorButton { biome: biome_1 },
                button_node.clone(),
                children![(
                    Text::from(biome_1.data().name),
                    button_text_font.clone(),
                    TextColor(NORMAL_BUTTON_TEXT_COLOR),
                )]
            ),
            (
                BiomeSelectorButton { biome: biome_2 },
                button_node.clone(),
                children![(
                    Text::from(biome_2.data().name),
                    button_text_font.clone(),
                    TextColor(NORMAL_BUTTON_TEXT_COLOR),
                )]
            ),
            (
                BiomeSelectorButton { biome: biome_3 },
                button_node.clone(),
                children![(
                    Text::from(biome_3.data().name),
                    button_text_font.clone(),
                    TextColor(NORMAL_BUTTON_TEXT_COLOR),
                )]
            )
        ],
    )
}
