use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use eots_asset_manager::ItemManager;
use rand::seq::IndexedRandom;

use crate::{
    CollectibleItem, Inventory, InventoryAction, InventoryAttraction, ItemActionMessage,
    SpawnItemMessage,
};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CollectibleItem>()
            .register_type::<Inventory>()
            .register_type::<InventoryAttraction>()
            .add_message::<ItemActionMessage>()
            .add_message::<SpawnItemMessage>()
            .add_systems(
                Update,
                (update_item_pickup, handle_add_item, handle_spawn_item).chain(),
            )
            .add_systems(
                Update,
                test_drop_item.run_if(input_just_pressed(KeyCode::KeyR)),
            );
    }
}

fn update_item_pickup(
    mut commands: Commands,
    mut wmessage_item_action: MessageWriter<ItemActionMessage>,
    mut inventory_q: Query<(Entity, &Transform, &InventoryAttraction), Without<CollectibleItem>>,
    mut item_q: Query<(Entity, &mut Transform, &CollectibleItem), Without<Inventory>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (inventory, inventory_transform, attraction) in &mut inventory_q {
        let inventory_position = inventory_transform.translation;
        let inventory_position_vec2 = inventory_transform.translation.truncate();

        for (item_entity, mut item_transform, collectible) in &mut item_q {
            let distance = item_transform
                .translation
                .truncate()
                .distance(inventory_position_vec2);

            if distance <= attraction.pickup_radius {
                commands.entity(item_entity).despawn();

                wmessage_item_action.write(ItemActionMessage {
                    entity: inventory,
                    action: InventoryAction::Add,
                    id: collectible.id,
                    count: collectible.count,
                });
            }

            if distance <= attraction.attract_radius {
                item_transform
                    .translation
                    .smooth_nudge(&inventory_position, 2.0, delta_time);
            }
        }
    }
}

fn handle_add_item(
    mut inventory_q: Query<&mut Inventory>,
    mut rmessage_add_item: MessageReader<ItemActionMessage>,
    mut wmessage_spawn_item: MessageWriter<SpawnItemMessage>,
    item_manager: Res<ItemManager>,
) {
    for ItemActionMessage {
        entity,
        action: _,
        id,
        count,
    } in rmessage_add_item.read()
    {
        let Ok(mut inventory) = inventory_q.get_mut(*entity) else {
            continue;
        };

        let Some((item, _)) = item_manager.get(id) else {
            continue;
        };

        match inventory.add_item(item, *count) {
            Ok(count) => {
                if count > 0 {
                    wmessage_spawn_item.write(SpawnItemMessage { id: *id, count });
                }
            }
            Err(inventory_error) => warn!("{:?}", &inventory_error),
        }
    }
}

fn handle_spawn_item(
    mut commands: Commands,
    mut rmessage_drop_item: MessageReader<SpawnItemMessage>,
    item_manager: Res<ItemManager>,
) {
    for &SpawnItemMessage {
        id: item,
        count: amount,
    } in rmessage_drop_item.read()
    {
        if let Some((_, image)) = item_manager.get(&item) {
            commands.spawn((
                Name::new(format!("Item ({:?}, {:?})", item, amount)),
                CollectibleItem {
                    id: item,
                    count: amount,
                },
                Sprite {
                    image: image.clone(),
                    custom_size: Some(Vec2::ONE),
                    ..default()
                },
            ));
        }
    }
}

fn test_drop_item(
    item_manager: Res<ItemManager>,
    mut wmessage_drop_item: MessageWriter<SpawnItemMessage>,
) {
    if let Some(&id) = item_manager.get_all_item_ids().choose(&mut rand::rng()) {
        wmessage_drop_item.write(SpawnItemMessage { id, count: 1 });
    }
}
