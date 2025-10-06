use bevy::prelude::*;

use crate::{SpawnerManager};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnerManager::default())
            .add_systems(Update, update_spawners);
    }
}

fn update_spawners(
    mut commands: Commands,
    mut spawner_handler: ResMut<SpawnerManager>,
    time: Res<Time>,
) {
    for (timer, config) in &mut spawner_handler.spawners {
        if timer.tick(time.delta()).just_finished() {
            (config.spawn_action)(&mut commands, config.generate_position());
        }
    }
}