use std::collections::HashMap;

use bevy::prelude::*;
use eots_item::{ItemData, ItemId};

#[derive(Resource)]
pub struct ItemManager {
    pub datas: HashMap<ItemId, (ItemData, Handle<Image>)>,
}

impl ItemManager {
    pub fn from_asset_server(items: Vec<ItemData>, asset_server: &AssetServer) -> Self {
        Self {
            datas: items
                .into_iter()
                .map(|item| {
                    let image = asset_server.load(&item.sprite_path);
                    (item.id, (item, image))
                })
                .collect(),
        }
    }

    pub fn get_all_item_ids(&self) -> Vec<ItemId> {
        self.datas.keys().cloned().collect()
    }

    pub fn get(&self, id: &ItemId) -> Option<&(ItemData, Handle<Image>)> {
        self.datas.get(id)
    }
}
