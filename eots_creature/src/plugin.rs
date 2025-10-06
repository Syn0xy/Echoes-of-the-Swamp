use bevy::prelude::*;
use eots_constants::creature::ENEMY_GROUP_LAYER;
use eots_core::SessionData;
use eots_inventory::Inventory;
use eots_perception::{GroupLayers, Perception, Targetable};
use eots_skeleton_rig::SkeletonRigEntity;
use eots_spawner::{RandomPositionStrategy, SpawnerConfig, SpawnerManager};

use crate::components::{Creature, CreatureStats};

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Creature>()
            .register_type::<CreatureStats>()
            .add_systems(Startup, setup_spawners)
            .add_systems(FixedUpdate, update_transform_creatures);
    }
}

fn setup_spawners(mut spawner_manager: ResMut<SpawnerManager>, session_data: Res<SessionData>) {
    let Some(biome_data) = session_data.current_biome_data() else {
        return;
    };

    let spawnable_creatures = biome_data.creatures;
    let current_level = spawnable_creatures.level_1;

    spawner_manager.add(
        Timer::from_seconds(1.0, TimerMode::Repeating),
        SpawnerConfig {
            position_strategy: Box::new(RandomPositionStrategy),
            origin: Vec2::new(0.0, 0.0),
            radius: 200.0,
            spawn_action: Box::new(|commands, position| {
                let Some(creature_id) = current_level.first() else {
                    return;
                };

                let creature_data = creature_id.data();

                let skeleton_entity =
                    eots_skeleton_rig::build_skeleton_rig(commands, creature_id.skeleton_rig());

                commands.spawn((
                    Name::new(format!("Creature ({})", creature_data.name)),
                    Transform::from_translation(position.extend(0.0)),
                    Creature(*creature_id),
                    CreatureStats::from(creature_id),
                    GroupLayers::new().with(ENEMY_GROUP_LAYER),
                    Perception::new(creature_data.vision_range),
                    Targetable,
                    Inventory::default(),
                    SkeletonRigEntity(skeleton_entity),
                ));
            }),
        },
    );
}

fn update_transform_creatures(
    mut transform_q: Query<&mut Transform>,
    mut creature_q: Query<(Entity, &Perception, &CreatureStats)>,
    time: Res<Time>,
) {
    for (creature_entity, perception, creature_stats) in &mut creature_q {
        let Some(target_entity) = perception.primary_target else {
            continue;
        };

        let Ok(target_transform) = transform_q.get(target_entity) else {
            continue;
        };

        let target_position = target_transform.translation.truncate();

        let Ok(mut creature_transform) = transform_q.get_mut(creature_entity) else {
            continue;
        };

        let delta_time = time.delta_secs();
        let creature_position = creature_transform.translation.truncate();
        let delta = (target_position - creature_position).normalize_or_zero()
            * creature_stats.move_speed
            * delta_time;

        creature_transform.translation.x += delta.x;
        creature_transform.translation.y += delta.y;
    }
}
