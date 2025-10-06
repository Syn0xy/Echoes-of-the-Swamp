use bevy::prelude::*;

use crate::{debug, GroupLayers, Perception, Targetable};

pub struct PerceptionPlugin;

impl Plugin for PerceptionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Perception>()
            .register_type::<Targetable>()
            .register_type::<GroupLayers>()
            .add_systems(Update, update_perceptions)
            .add_systems(PostUpdate, debug::debug_perception)
            ;
    }
}

fn update_perceptions(
    mut query: Query<(
        Entity,
        &Transform,
        &GroupLayers,
        Option<&mut Perception>,
        Option<&Targetable>,
    )>,
) {
    let targets: Vec<(Entity, GroupLayers, Vec2)> = query
        .iter()
        .filter_map(|(entity, transform, layers, _, target)| {
            target.map(|_| (entity, layers.clone(), transform.translation.truncate()))
        })
        .collect();

    for (entity, transform, layers, perception, _) in &mut query {
        let Some(mut perception) = perception else {
            continue;
        };

        perception.primary_target = None;
        perception.visible_targets.clear();

        let position = transform.translation.truncate();
        let mut nearest_target: Option<(&Entity, f32)> = None;

        for (target_entity, target_layers, target_position) in &targets {
            if entity.index() == target_entity.index() {
                continue;
            }

            if layers.intersects(target_layers) {
                continue;
            }

            let distance = target_position.distance(position);
            if distance <= perception.vision_range {
                perception.visible_targets.push(*target_entity);

                if nearest_target.map_or(true, |(_, old_distance)| distance < old_distance) {
                    nearest_target = Some((target_entity, distance));
                }
            }
        }

        if let Some((nearest_position, _)) = nearest_target {
            perception.primary_target = Some(*nearest_position);
        }
    }
}
