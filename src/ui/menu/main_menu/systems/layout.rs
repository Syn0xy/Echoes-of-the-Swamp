use bevy::prelude::*;

use crate::ui::menu::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::{BUTTON_FONT_SIZE, NORMAL_BUTTON_TEXT_COLOR, TEXT_FONT_PATH},
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(build_main_menu(&asset_server));
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        if let Ok(mut entity) = commands.get_entity(main_menu_entity) {
            entity.despawn();
        }
    }
}

fn build_main_menu(asset_server: &Res<AssetServer>) -> impl Bundle {
    let menu_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
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

    (
        MainMenu,
        menu_node,
        children![
            (
                PlayButton,
                button_node.clone(),
                children![(
                    Text::from("Play"),
                    button_text_font.clone(),
                    TextColor(NORMAL_BUTTON_TEXT_COLOR),
                )]
            ),
            (
                QuitButton,
                button_node.clone(),
                children![(
                    Text::from("Quit"),
                    button_text_font.clone(),
                    TextColor(NORMAL_BUTTON_TEXT_COLOR),
                )]
            )
        ],
    )
}
