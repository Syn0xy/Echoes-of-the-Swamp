use std::{fs::File, io::BufReader};

use bevy::prelude::*;

use crate::ItemManager;

const ITEMS_PATH: &str = "assets/items/items.json";

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_items);
    }
}

fn load_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    let Ok(reader) = File::open(ITEMS_PATH).map(BufReader::new) else {
        return;
    };

    if let Ok(items) = serde_json::from_reader(reader) {
        commands.insert_resource(ItemManager::from_asset_server(items, asset_server.as_ref()));
    };
}
